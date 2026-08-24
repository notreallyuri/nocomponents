//! A small, dependency-free tokenizer for the snippets `CodeBlock` renders.
//!
//! A lexer, not a parser: it recognises the shapes that make code readable and leaves the rest
//! alone. Tokens cover the input exactly, so concatenating every `Token::text` reproduces the
//! source byte for byte — the rendered block can never lose or reorder a character.

use crate::utils::types::Language;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenKind {
    Plain,
    Keyword,
    Type,
    /// An identifier being called, or being declared to be callable: what is on the left of a
    /// `(`. Not resolved — a lexer cannot know that `f` in `let g = f;` is the same function —
    /// which is why this is about the shape of the call rather than about the name.
    Function,
    Macro,
    Str,
    Number,
    Comment,
    Attribute,
    Lifetime,
}

impl TokenKind {
    /// Rendered as `data-token`, which is what the styled layer colours.
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenKind::Plain => "plain",
            TokenKind::Keyword => "keyword",
            TokenKind::Type => "type",
            TokenKind::Function => "function",
            TokenKind::Macro => "macro",
            TokenKind::Str => "string",
            TokenKind::Number => "number",
            TokenKind::Comment => "comment",
            TokenKind::Attribute => "attribute",
            TokenKind::Lifetime => "lifetime",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "yield",
];

const RUST_PRIMITIVES: &[&str] = &[
    "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "str", "u8", "u16",
    "u32", "u64", "u128", "usize",
];

/// A token as the scanners produce it: a kind plus a byte range into the source.
type Span = (TokenKind, usize, usize);

pub fn tokenize(source: &str, language: Language) -> Vec<Token<'_>> {
    let spans = match language {
        Language::Rust => tokenize_rust(source),
        Language::Shell => tokenize_shell(source),
        Language::JavaScript => tokenize_curly(source, false),
        Language::TypeScript => tokenize_curly(source, true),
        Language::Html => tokenize_html(source),
        Language::Css => tokenize_css(source),
        Language::Plain => vec![(TokenKind::Plain, 0, source.len())],
    };

    let spans = merge_plain(spans);

    debug_assert!(
        spans
            .iter()
            .try_fold(0, |at, (_, start, end)| (*start == at).then_some(*end))
            == Some(source.len()),
        "tokens must cover the source exactly, with no gaps or overlaps"
    );

    spans
        .into_iter()
        .map(|(kind, start, end)| Token {
            kind,
            text: &source[start..end],
        })
        .collect()
}

/// Adjacent plain runs become one token, so the DOM gets a handful of spans instead of one per
/// bracket and space.
fn merge_plain(spans: Vec<Span>) -> Vec<Span> {
    let mut merged: Vec<Span> = Vec::with_capacity(spans.len());

    for (kind, start, end) in spans {
        match merged.last_mut() {
            Some(last) if last.0 == TokenKind::Plain && kind == TokenKind::Plain => last.2 = end,
            _ => merged.push((kind, start, end)),
        }
    }

    merged
}

fn tokenize_rust(source: &str) -> Vec<Span> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;

    while i < source.len() {
        let start = i;
        let c = bytes[i];

        // Line comment
        if source[i..].starts_with("//") {
            i = source[i..]
                .find('\n')
                .map(|n| i + n)
                .unwrap_or(source.len());
            push(&mut tokens, TokenKind::Comment, source, start, i);
            continue;
        }

        // Block comment, nested like rustc allows
        if source[i..].starts_with("/*") {
            let mut depth = 0usize;
            while i < source.len() {
                if source[i..].starts_with("/*") {
                    depth += 1;
                    i += 2;
                } else if source[i..].starts_with("*/") {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    i += next_char_len(source, i);
                }
            }
            push(&mut tokens, TokenKind::Comment, source, start, i);
            continue;
        }

        // Raw string: r"…", r#"…"#, r##"…"##
        if c == b'r'
            && matches!(bytes.get(i + 1), Some(b'"') | Some(b'#'))
            && let Some(end) = raw_string_end(source, i)
        {
            i = end;
            push(&mut tokens, TokenKind::Str, source, start, i);
            continue;
        }

        // Attribute: #[…] / #![…]
        if c == b'#' && matches!(bytes.get(i + 1), Some(b'[') | Some(b'!')) {
            let mut depth = 0usize;
            let mut j = i;
            while j < source.len() {
                match bytes[j] {
                    b'[' => depth += 1,
                    b']' => {
                        depth -= 1;
                        if depth == 0 {
                            j += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                j += next_char_len(source, j);
            }
            if depth == 0 && j > i {
                i = j;
                push(&mut tokens, TokenKind::Attribute, source, start, i);
                continue;
            }
        }

        // String literal
        if c == b'"' {
            i += 1;
            while i < source.len() {
                match bytes[i] {
                    b'\\' => i += 2,
                    b'"' => {
                        i += 1;
                        break;
                    }
                    _ => i += next_char_len(source, i),
                }
            }
            push(
                &mut tokens,
                TokenKind::Str,
                source,
                start,
                i.min(source.len()),
            );
            continue;
        }

        // Char literal or lifetime
        if c == b'\'' {
            if let Some(end) = char_literal_end(source, i) {
                i = end;
                push(&mut tokens, TokenKind::Str, source, start, i);
            } else {
                i += 1;
                while i < source.len() && is_ident_byte(bytes[i]) {
                    i += 1;
                }
                push(&mut tokens, TokenKind::Lifetime, source, start, i);
            }
            continue;
        }

        // Number
        if c.is_ascii_digit() {
            while i < source.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
            {
                // don't swallow the second dot of a range like 0..3
                if bytes[i] == b'.' && source[i..].starts_with("..") {
                    break;
                }
                i += 1;
            }
            push(&mut tokens, TokenKind::Number, source, start, i);
            continue;
        }

        // Identifier
        if is_ident_start(bytes[i]) {
            while i < source.len() && is_ident_byte(bytes[i]) {
                i += 1;
            }
            let word = &source[start..i];

            let kind = if bytes.get(i) == Some(&b'!') && word != "if" && word != "while" {
                i += 1; // the `!` belongs to the macro name
                TokenKind::Macro
            } else if RUST_KEYWORDS.contains(&word) {
                TokenKind::Keyword
            } else if word.starts_with(|ch: char| ch.is_ascii_uppercase())
                || RUST_PRIMITIVES.contains(&word)
            {
                // `Self` and every other capitalised path segment read as a type, as do the
                // lowercase primitives. `Some(x)` is a call, but what a reader wants coloured
                // there is the variant, so the type wins over the call ahead of it.
                TokenKind::Type
            } else if is_called(source, i) {
                TokenKind::Function
            } else {
                TokenKind::Plain
            };

            push(&mut tokens, kind, source, start, i);
            continue;
        }

        i += next_char_len(source, i);
        push(&mut tokens, TokenKind::Plain, source, start, i);
    }

    tokens
}

fn tokenize_shell(source: &str) -> Vec<Span> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    let mut at_line_start = true;

    while i < source.len() {
        let start = i;
        let c = bytes[i];

        if c == b'#' {
            i = source[i..]
                .find('\n')
                .map(|n| i + n)
                .unwrap_or(source.len());
            push(&mut tokens, TokenKind::Comment, source, start, i);
            continue;
        }

        if c == b'"' || c == b'\'' {
            let quote = c;
            i += 1;
            while i < source.len() && bytes[i] != quote {
                i += if bytes[i] == b'\\' {
                    2
                } else {
                    next_char_len(source, i)
                };
            }
            i = (i + 1).min(source.len());
            push(&mut tokens, TokenKind::Str, source, start, i);
            continue;
        }

        if c.is_ascii_whitespace() {
            if c == b'\n' {
                at_line_start = true;
            }
            i += 1;
            push(&mut tokens, TokenKind::Plain, source, start, i);
            continue;
        }

        if c == b'-' {
            while i < source.len() && !bytes[i].is_ascii_whitespace() {
                i += next_char_len(source, i);
            }
            push(&mut tokens, TokenKind::Attribute, source, start, i);
            at_line_start = false;
            continue;
        }

        while i < source.len() && !bytes[i].is_ascii_whitespace() {
            i += next_char_len(source, i);
        }
        let kind = if at_line_start {
            TokenKind::Keyword
        } else {
            TokenKind::Plain
        };
        push(&mut tokens, kind, source, start, i);
        at_line_start = false;
    }

    tokens
}

const JS_KEYWORDS: &[&str] = &[
    "async",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "from",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "let",
    "new",
    "null",
    "of",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "try",
    "typeof",
    "undefined",
    "var",
    "void",
    "while",
    "with",
    "yield",
];

/// Recognised on top of [`JS_KEYWORDS`] when the language is TypeScript.
const TS_KEYWORDS: &[&str] = &[
    "abstract",
    "any",
    "as",
    "asserts",
    "bigint",
    "boolean",
    "declare",
    "enum",
    "implements",
    "infer",
    "interface",
    "is",
    "keyof",
    "namespace",
    "never",
    "number",
    "object",
    "override",
    "private",
    "protected",
    "public",
    "readonly",
    "satisfies",
    "string",
    "symbol",
    "type",
    "unique",
    "unknown",
];

/// The keywords a `/` can follow and still be starting a regex. Every other keyword — `this`,
/// `true`, `null` — is a value, so what follows it is division.
const JS_VALUE_KEYWORDS: &[&str] = &["false", "null", "super", "this", "true", "undefined"];

/// JavaScript and TypeScript.
///
/// Two shapes here need more than the byte in front of them. A `/` is a regex in one position and
/// division in another, which is decided by what the last token was — the standard lexer
/// heuristic, and the reason this loop remembers it. And a `${…}` inside a template literal is
/// code, not string, so the template is cut around each one and the inside is tokenized by this
/// same function.
fn tokenize_curly(source: &str, typescript: bool) -> Vec<Span> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    // What the last token was that a `/` cares about; a comment and whitespace leave it alone.
    let mut value_before = false;

    while i < source.len() {
        let start = i;
        let c = bytes[i];

        if source[i..].starts_with("//") {
            i = source[i..]
                .find('\n')
                .map(|n| i + n)
                .unwrap_or(source.len());
            push(&mut tokens, TokenKind::Comment, source, start, i);
            continue;
        }

        if source[i..].starts_with("/*") {
            i = source[i..]
                .find("*/")
                .map(|n| i + n + 2)
                .unwrap_or(source.len());
            push(&mut tokens, TokenKind::Comment, source, start, i);
            continue;
        }

        // A regex only where a value could begin, and only when it closes on its own line: an
        // unterminated one is division after all, and reading it as a literal would swallow the
        // rest of the snippet.
        if c == b'/'
            && !value_before
            && let Some(end) = regex_literal_end(source, i)
        {
            i = end;
            push(&mut tokens, TokenKind::Str, source, start, i);
            value_before = true;
            continue;
        }

        if c == b'`' {
            i = scan_template(source, i, typescript, &mut tokens);
            value_before = true;
            continue;
        }

        if c == b'"' || c == b'\'' {
            i = string_end(source, i);
            push(&mut tokens, TokenKind::Str, source, start, i);
            value_before = true;
            continue;
        }

        if c.is_ascii_digit() {
            while i < source.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
            {
                i += 1;
            }
            push(&mut tokens, TokenKind::Number, source, start, i);
            value_before = true;
            continue;
        }

        if is_ident_start(bytes[i]) || c == b'$' {
            while i < source.len() && (is_ident_byte(bytes[i]) || bytes[i] == b'$') {
                i += 1;
            }
            let word = &source[start..i];
            let keyword =
                JS_KEYWORDS.contains(&word) || (typescript && TS_KEYWORDS.contains(&word));
            let kind = if keyword {
                TokenKind::Keyword
            } else if word.starts_with(|ch: char| ch.is_ascii_uppercase()) {
                TokenKind::Type
            } else if is_called(source, i) {
                TokenKind::Function
            } else {
                TokenKind::Plain
            };
            push(&mut tokens, kind, source, start, i);
            // `return /a/` is a regex and `this / 2` is division, so a keyword is only a value
            // when it names one.
            value_before = !keyword || JS_VALUE_KEYWORDS.contains(&word);
            continue;
        }

        if c == b'@' {
            i += 1;
            while i < source.len() && is_ident_byte(bytes[i]) {
                i += 1;
            }
            push(&mut tokens, TokenKind::Attribute, source, start, i);
            value_before = false;
            continue;
        }

        i += next_char_len(source, i);
        push(&mut tokens, TokenKind::Plain, source, start, i);
        // Everything that closes an expression can be divided; whitespace decides nothing.
        if !c.is_ascii_whitespace() {
            value_before = matches!(c, b')' | b']' | b'}');
        }
    }

    tokens
}

/// A template literal, cut around every `${…}` so the code inside one is tokenized as code.
/// Returns where the literal ended — past its closing backtick, or the end of the source when it
/// never closes.
fn scan_template(source: &str, start: usize, typescript: bool, tokens: &mut Vec<Span>) -> usize {
    let bytes = source.as_bytes();
    // Where the current run of string began: the backtick, then each `}` that closes a hole.
    let mut segment = start;
    let mut i = start + 1;

    while i < source.len() {
        match bytes[i] {
            b'\\' => i = (i + 2).min(source.len()),
            b'`' => {
                i += 1;
                push(tokens, TokenKind::Str, source, segment, i);
                return i;
            }
            b'$' if bytes.get(i + 1) == Some(&b'{') => {
                let Some(close) = template_hole_end(source, i + 2) else {
                    // Unterminated: the rest of the source is this string, which is what an
                    // editor shows too.
                    break;
                };

                push(tokens, TokenKind::Str, source, segment, i);
                push(tokens, TokenKind::Plain, source, i, i + 2);
                for (kind, from, to) in tokenize_curly(&source[i + 2..close], typescript) {
                    push(tokens, kind, source, i + 2 + from, i + 2 + to);
                }
                push(tokens, TokenKind::Plain, source, close, close + 1);

                i = close + 1;
                segment = i;
            }
            _ => i += next_char_len(source, i),
        }
    }

    push(tokens, TokenKind::Str, source, segment, source.len());
    source.len()
}

/// The `}` closing a `${…}` that opens at `start`, or `None` if it never closes. Braces nest, and
/// a string or a template inside one may hold braces of its own that do not count.
fn template_hole_end(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut i = start;
    let mut depth = 0usize;

    while i < source.len() {
        match bytes[i] {
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' if depth == 0 => return Some(i),
            b'}' => {
                depth -= 1;
                i += 1;
            }
            b'"' | b'\'' => i = string_end(source, i),
            b'`' => i = template_end(source, i),
            _ => i += next_char_len(source, i),
        }
    }

    None
}

/// Past the closing quote of the string opening at `start`, or the end of the source.
fn string_end(source: &str, start: usize) -> usize {
    let bytes = source.as_bytes();
    let quote = bytes[start];
    let mut i = start + 1;

    while i < source.len() {
        match bytes[i] {
            b'\\' => i = (i + 2).min(source.len()),
            b'\n' => break,
            q if q == quote => return i + 1,
            _ => i += next_char_len(source, i),
        }
    }

    i.min(source.len())
}

/// Past the closing backtick of the template opening at `start`. Only used to skip one, so it
/// tokenizes nothing — but it still has to walk its holes, since a `}` inside one is not the end
/// of anything.
fn template_end(source: &str, start: usize) -> usize {
    let bytes = source.as_bytes();
    let mut i = start + 1;

    while i < source.len() {
        match bytes[i] {
            b'\\' => i = (i + 2).min(source.len()),
            b'`' => return i + 1,
            b'$' if bytes.get(i + 1) == Some(&b'{') => match template_hole_end(source, i + 2) {
                Some(close) => i = close + 1,
                None => break,
            },
            _ => i += next_char_len(source, i),
        }
    }

    source.len()
}

/// Past the closing `/` and flags of a regex literal opening at `start`, or `None` when what is
/// there is not one: an unterminated body, or a newline inside it.
fn regex_literal_end(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    // An empty regex is not a thing, and `//` is a comment — which is checked before this.
    let mut i = start + 1;
    let mut in_class = false;

    while i < source.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b'\n' => return None,
            b'[' => {
                in_class = true;
                i += 1;
            }
            b']' => {
                in_class = false;
                i += 1;
            }
            b'/' if !in_class => {
                i += 1;
                // Flags, and only the letters that are ones — so `/a/ g` does not eat the space.
                while i < source.len() && bytes[i].is_ascii_lowercase() {
                    i += 1;
                }
                return (i > start + 2).then_some(i);
            }
            _ => i += next_char_len(source, i),
        }
    }

    None
}

/// Whether the identifier ending at `at` is being called: a `(` after it, or a turbofish and then
/// a `(`. Whitespace between the two counts — `foo ()` is still a call — but a newline does not,
/// since that is usually a line ending in a name and the next one opening a group.
fn is_called(source: &str, at: usize) -> bool {
    let rest = source[at..].trim_start_matches([' ', '\t']);

    rest.starts_with('(')
        || rest
            .strip_prefix("::<")
            .and_then(|turbofish| turbofish.split_once('('))
            // Up to the `(` and no further, since `Vec<_>>` closes on the last `>` and not the
            // first: what makes this a call is that the arguments open where the generics end.
            .is_some_and(|(generics, _)| generics.trim_end().ends_with('>'))
}

fn tokenize_html(source: &str) -> Vec<Span> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    // Inside a tag, the first word is the element name and the rest are attributes.
    let mut in_tag = false;
    let mut seen_name = false;

    while i < source.len() {
        let start = i;
        let c = bytes[i];

        if source[i..].starts_with("<!--") {
            i = source[i..]
                .find("-->")
                .map(|n| i + n + 3)
                .unwrap_or(source.len());
            push(&mut tokens, TokenKind::Comment, source, start, i);
            continue;
        }

        if source[i..].starts_with("<!") {
            i = source[i..]
                .find('>')
                .map(|n| i + n + 1)
                .unwrap_or(source.len());
            push(&mut tokens, TokenKind::Keyword, source, start, i);
            continue;
        }

        if c == b'<' {
            in_tag = true;
            seen_name = false;
            i += 1;
            if bytes.get(i) == Some(&b'/') {
                i += 1;
            }
            push(&mut tokens, TokenKind::Plain, source, start, i);
            continue;
        }

        if c == b'>' {
            in_tag = false;
            i += 1;
            push(&mut tokens, TokenKind::Plain, source, start, i);
            continue;
        }

        if in_tag && (c == b'"' || c == b'\'') {
            i += 1;
            while i < source.len() && bytes[i] != c {
                i += next_char_len(source, i);
            }
            i = (i + 1).min(source.len());
            push(&mut tokens, TokenKind::Str, source, start, i);
            continue;
        }

        if in_tag && (is_ident_start(bytes[i]) || c == b':' || c == b'-') {
            while i < source.len()
                && (is_ident_byte(bytes[i]) || bytes[i] == b'-' || bytes[i] == b':')
            {
                i += 1;
            }
            let kind = if seen_name {
                TokenKind::Attribute
            } else {
                seen_name = true;
                TokenKind::Type
            };
            push(&mut tokens, kind, source, start, i);
            continue;
        }

        i += next_char_len(source, i);
        push(&mut tokens, TokenKind::Plain, source, start, i);
    }

    tokens
}

fn tokenize_css(source: &str) -> Vec<Span> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    // Identifiers read as properties inside a declaration block and as selectors outside one.
    let mut depth = 0usize;

    while i < source.len() {
        let start = i;
        let c = bytes[i];

        if source[i..].starts_with("/*") {
            i = source[i..]
                .find("*/")
                .map(|n| i + n + 2)
                .unwrap_or(source.len());
            push(&mut tokens, TokenKind::Comment, source, start, i);
            continue;
        }

        if c == b'"' || c == b'\'' {
            i += 1;
            while i < source.len() && bytes[i] != c {
                i += if bytes[i] == b'\\' {
                    2
                } else {
                    next_char_len(source, i)
                };
            }
            i = (i + 1).min(source.len());
            push(&mut tokens, TokenKind::Str, source, start, i);
            continue;
        }

        if c == b'@' {
            i += 1;
            while i < source.len() && is_ident_byte(bytes[i]) {
                i += 1;
            }
            push(&mut tokens, TokenKind::Keyword, source, start, i);
            continue;
        }

        // Hex colour, or an id selector
        if c == b'#' {
            i += 1;
            while i < source.len() && (is_ident_byte(bytes[i]) || bytes[i] == b'-') {
                i += 1;
            }
            let kind = if depth > 0 {
                TokenKind::Number
            } else {
                TokenKind::Type
            };
            push(&mut tokens, kind, source, start, i);
            continue;
        }

        if c.is_ascii_digit() || (c == b'.' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit)) {
            i += 1;
            while i < source.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.' || bytes[i] == b'%')
            {
                i += 1;
            }
            push(&mut tokens, TokenKind::Number, source, start, i);
            continue;
        }

        if is_ident_start(bytes[i]) || c == b'.' || c == b'-' {
            i += 1;
            while i < source.len()
                && (is_ident_byte(bytes[i]) || bytes[i] == b'-' || bytes[i] == b'.')
            {
                i += 1;
            }
            let kind = if depth > 0 {
                let rest = source[i..].trim_start();
                if rest.starts_with(':') {
                    TokenKind::Attribute
                } else {
                    TokenKind::Plain
                }
            } else {
                TokenKind::Type
            };
            push(&mut tokens, kind, source, start, i);
            continue;
        }

        if c == b'{' {
            depth += 1;
        } else if c == b'}' {
            depth = depth.saturating_sub(1);
        }

        i += next_char_len(source, i);
        push(&mut tokens, TokenKind::Plain, source, start, i);
    }

    tokens
}

fn push(spans: &mut Vec<Span>, kind: TokenKind, _source: &str, start: usize, end: usize) {
    if end > start {
        spans.push((kind, start, end));
    }
}

fn next_char_len(source: &str, i: usize) -> usize {
    source[i..].chars().next().map(char::len_utf8).unwrap_or(1)
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// End index of a raw string starting at `i`, or `None` if it is unterminated.
fn raw_string_end(source: &str, i: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut j = i + 1;
    let mut hashes = 0;
    while bytes.get(j) == Some(&b'#') {
        hashes += 1;
        j += 1;
    }
    if bytes.get(j) != Some(&b'"') {
        return None;
    }
    j += 1;

    let terminator = format!("\"{}", "#".repeat(hashes));
    source[j..]
        .find(&terminator)
        .map(|n| j + n + terminator.len())
}

/// End index of a char literal starting at `i`, or `None` when the quote opens a lifetime.
fn char_literal_end(source: &str, i: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut j = i + 1;

    if bytes.get(j) == Some(&b'\\') {
        j += 2;
        while bytes.get(j).is_some_and(|b| *b != b'\'') {
            j += 1;
        }
        return (bytes.get(j) == Some(&b'\'')).then_some(j + 1);
    }

    let len = next_char_len(source, j);
    (bytes.get(j + len) == Some(&b'\'')).then_some(j + len + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds<'a>(src: &'a str, lang: Language) -> Vec<(TokenKind, &'a str)> {
        tokenize(src, lang)
            .into_iter()
            .map(|t| (t.kind, t.text))
            .collect()
    }

    /// The invariant that matters: whatever we decide a token is, no character may be dropped,
    /// duplicated or reordered.
    fn assert_round_trip(src: &str, lang: Language) {
        let joined: String = tokenize(src, lang).iter().map(|t| t.text).collect();
        assert_eq!(joined, src, "round trip failed for {src:?}");
    }

    #[test]
    fn round_trips_every_sample() {
        let samples = [
            "let x = 1;",
            "// comment\nfn main() {}",
            "/* nested /* block */ still */ let y = 2;",
            r#"let s = "with \"escapes\" inside";"#,
            "let r = r#\"raw \"quoted\" string\"#;",
            "let c = 'x'; let n = '\\n';",
            "fn f<'a>(v: &'a str) -> &'a str { v }",
            "#[derive(Clone, Debug)]\npub struct S;",
            "view! { <Toggle variant=ToggleVariant::Outline>\"Outline\"</Toggle> }",
            "for i in 0..3 { println!(\"{i}\"); }",
            "let emoji = \"⌘ K\"; // unicode must survive",
            "const x: Array<string> = [`a ${b}`, 'c'];",
            "const re = /ab+[/]c/gi.test(s) ? a / b : `${ f({ x: `}` }) }`;",
            "let s = `outer ${ `inner ${deep}` } end`;",
            "`unterminated ${ hole",
            "<button class=\"x\" data-state='on'>Go</button><!-- note -->",
            ".card { color: #fff; padding: 1.5rem } @media (hover: hover) { a { color: red } }",
            "export default function App() { return null; }",
            "",
        ];
        let languages = [
            Language::Rust,
            Language::Shell,
            Language::JavaScript,
            Language::TypeScript,
            Language::Html,
            Language::Css,
            Language::Plain,
        ];
        for s in samples {
            for lang in languages {
                assert_round_trip(s, lang);
            }
        }
    }

    #[test]
    fn classifies_typescript() {
        let t = kinds("const n: number = 1;", Language::TypeScript);
        assert!(t.contains(&(TokenKind::Keyword, "const")));
        assert!(t.contains(&(TokenKind::Keyword, "number")));
        // the same word is not a keyword in plain JS (plain runs are merged, so check the kind)
        let js = kinds("const n: number = 1;", Language::JavaScript);
        assert!(
            !js.iter()
                .any(|(kind, text)| *kind == TokenKind::Keyword && *text == "number")
        );
    }

    #[test]
    fn template_literals_break_out_their_holes() {
        let t = kinds("const s = `a ${b} c`;", Language::JavaScript);
        assert!(t.contains(&(TokenKind::Str, "`a ")));
        assert!(t.contains(&(TokenKind::Str, " c`")));
        // The hole is code: `b` is not part of the string.
        assert!(
            !t.iter()
                .any(|(kind, text)| *kind == TokenKind::Str && text.contains('b'))
        );

        // A brace inside a nested string does not close the hole, and a nested template is walked
        // rather than counted.
        let t = kinds("`${ f({ a: `}` }) } tail`", Language::JavaScript);
        assert!(t.contains(&(TokenKind::Function, "f")));
        assert!(t.contains(&(TokenKind::Str, " tail`")));

        // An unterminated hole leaves the rest a string rather than losing it.
        assert_round_trip("`a ${ b", Language::JavaScript);
        assert_round_trip("`a ${ b } c", Language::JavaScript);
    }

    #[test]
    fn regex_literals_are_told_from_division() {
        let t = kinds("const re = /ab+[/]c/gi;", Language::JavaScript);
        assert!(t.contains(&(TokenKind::Str, "/ab+[/]c/gi")));

        // After a value, a slash is division — and the tokens after it stay themselves.
        let t = kinds("const half = width / 2;", Language::JavaScript);
        assert!(!t.iter().any(|(kind, _)| *kind == TokenKind::Str));
        assert!(t.contains(&(TokenKind::Number, "2")));

        // `return` is not a value, so what follows it can be one.
        assert!(
            kinds("return /x/.test(s);", Language::JavaScript).contains(&(TokenKind::Str, "/x/"))
        );
        // ...but `this` is.
        assert!(
            !kinds("this / that / other", Language::JavaScript)
                .iter()
                .any(|(kind, _)| *kind == TokenKind::Str)
        );

        // Nothing that fails to close on its line is a regex.
        let t = kinds("a = b / c;\nd = e / f;", Language::JavaScript);
        assert!(!t.iter().any(|(kind, _)| *kind == TokenKind::Str));
    }

    #[test]
    fn a_call_is_not_a_plain_identifier() {
        let t = kinds("fn main() { helper(2) }", Language::Rust);
        assert!(t.contains(&(TokenKind::Function, "main")));
        assert!(t.contains(&(TokenKind::Function, "helper")));

        // A name that is not called stays plain — merged into the run around it, so what is
        // checked is that it is not a function — and a turbofish is still a call.
        let t = kinds("let f = helper; xs.collect::<Vec<_>>()", Language::Rust);
        assert!(!t.contains(&(TokenKind::Function, "helper")));
        assert!(t.contains(&(TokenKind::Function, "collect")));

        // The macro and the type keep winning over the call ahead of them.
        assert!(kinds("println!(\"x\")", Language::Rust).contains(&(TokenKind::Macro, "println!")));
        assert!(kinds("Some(1)", Language::Rust).contains(&(TokenKind::Type, "Some")));

        let t = kinds("export function run() { count(); }", Language::JavaScript);
        assert!(t.contains(&(TokenKind::Function, "run")));
        assert!(t.contains(&(TokenKind::Function, "count")));
    }

    #[test]
    fn html_separates_tag_from_attributes() {
        let t = kinds(r#"<button class="x">Go</button>"#, Language::Html);
        assert!(t.contains(&(TokenKind::Type, "button")));
        assert!(t.contains(&(TokenKind::Attribute, "class")));
        assert!(t.contains(&(TokenKind::Str, "\"x\"")));
        // text between tags stays plain, merged with the surrounding punctuation
        assert!(
            t.iter()
                .any(|(kind, text)| *kind == TokenKind::Plain && text.contains("Go"))
        );
    }

    #[test]
    fn css_splits_selectors_from_properties() {
        let t = kinds(".card { color: #fff }", Language::Css);
        assert!(t.contains(&(TokenKind::Type, ".card")));
        assert!(t.contains(&(TokenKind::Attribute, "color")));
        assert!(t.contains(&(TokenKind::Number, "#fff")));
        assert!(kinds("@media print { }", Language::Css).contains(&(TokenKind::Keyword, "@media")));
    }

    #[test]
    fn classifies_rust() {
        let t = kinds("let n: u8 = 3;", Language::Rust);
        assert!(t.contains(&(TokenKind::Keyword, "let")));
        assert!(t.contains(&(TokenKind::Type, "u8")));
        assert!(t.contains(&(TokenKind::Number, "3")));
    }

    #[test]
    fn macro_takes_its_bang() {
        let t = kinds("view! { }", Language::Rust);
        assert_eq!(t[0], (TokenKind::Macro, "view!"));
    }

    #[test]
    fn lifetime_is_not_a_char_literal() {
        let t = kinds("&'a str", Language::Rust);
        assert!(t.contains(&(TokenKind::Lifetime, "'a")));
        assert!(kinds("'x'", Language::Rust).contains(&(TokenKind::Str, "'x'")));
    }

    #[test]
    fn range_is_not_swallowed_by_a_number() {
        let t = kinds("0..3", Language::Rust);
        assert_eq!(t[0], (TokenKind::Number, "0"));
        assert!(t.contains(&(TokenKind::Number, "3")));
    }

    #[test]
    fn uppercase_identifiers_read_as_types() {
        let t = kinds("ToggleVariant::Outline", Language::Rust);
        assert_eq!(t[0], (TokenKind::Type, "ToggleVariant"));
    }

    #[test]
    fn shell_marks_command_and_flags() {
        let t = kinds("cargo add nocomponents --features full", Language::Shell);
        assert_eq!(t[0], (TokenKind::Keyword, "cargo"));
        assert!(t.contains(&(TokenKind::Attribute, "--features")));
    }
}
