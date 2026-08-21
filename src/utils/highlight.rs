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
                // lowercase primitives.
                TokenKind::Type
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

/// JavaScript and TypeScript. Regex literals are not recognised — telling `/` apart needs a
/// parser — so a regex reads as punctuation plus its contents.
fn tokenize_curly(source: &str, typescript: bool) -> Vec<Span> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;

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

        // Plain and template strings alike; `${…}` inside a template is not broken out.
        if c == b'"' || c == b'\'' || c == b'`' {
            i += 1;
            while i < source.len() {
                match bytes[i] {
                    b'\\' => i += 2,
                    q if q == c => {
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

        if c.is_ascii_digit() {
            while i < source.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
            {
                i += 1;
            }
            push(&mut tokens, TokenKind::Number, source, start, i);
            continue;
        }

        if is_ident_start(bytes[i]) || c == b'$' {
            while i < source.len() && (is_ident_byte(bytes[i]) || bytes[i] == b'$') {
                i += 1;
            }
            let word = &source[start..i];
            let kind = if JS_KEYWORDS.contains(&word) || (typescript && TS_KEYWORDS.contains(&word))
            {
                TokenKind::Keyword
            } else if word.starts_with(|ch: char| ch.is_ascii_uppercase()) {
                TokenKind::Type
            } else {
                TokenKind::Plain
            };
            push(&mut tokens, kind, source, start, i);
            continue;
        }

        if c == b'@' {
            i += 1;
            while i < source.len() && is_ident_byte(bytes[i]) {
                i += 1;
            }
            push(&mut tokens, TokenKind::Attribute, source, start, i);
            continue;
        }

        i += next_char_len(source, i);
        push(&mut tokens, TokenKind::Plain, source, start, i);
    }

    tokens
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
    fn template_literals_are_strings() {
        let t = kinds("const s = `a ${b} c`;", Language::JavaScript);
        assert!(t.contains(&(TokenKind::Str, "`a ${b} c`")));
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
