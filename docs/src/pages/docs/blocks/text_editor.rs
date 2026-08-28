//! Text editor blocks.

use super::Block;
use leptos::web_sys::HtmlTextAreaElement;
use leptos::{ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use nocomponents::{
    components::{
        badge::{Badge, BadgeVariant},
        button::{Button, ButtonSize, ButtonVariant},
        kbd::Kbd,
        separator::Separator,
        tabs::{Tabs, TabsContent, TabsList, TabsTrigger},
        textarea::Textarea,
        toggle::{ToggleSize, ToggleVariant},
        toggle_group::{ToggleGroup, ToggleGroupItem},
        tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    },
    icons::IconRoot,
    primitives::toggle_group::ToggleGroupType,
    utils::types::Orientation,
};
use std::time::Duration;

const EDITOR: &str = r##"// A toggle group is a toggle group: the buttons stay pressed for as long as
// what they describe is true. That is only honest if the toolbar reads the
// selection rather than remembering what was last pressed — so the marks are
// derived from the text under the caret, and pressing one rewrites the text.

let text = RwSignal::new(String::new());
// The selection is the one piece of a textarea's state that lives nowhere but
// the DOM, which is why `Textarea` takes a `node_ref`.
let area = AnyNodeRef::new();
let selection = RwSignal::new((0usize, 0usize));

// What is true right now. Nothing remembers it; it is read back every time
// either the text or the selection moves.
let marks = RwSignal::new(Vec::<String>::new());
Effect::new(move |_| {
    let (text, (start, end)) = (text.get(), selection.get());
    marks.set(
        MARKS.iter()
            .filter(|mark| is_marked(&text, start, end, mark.marker))
            .map(|mark| mark.slug.to_string())
            .collect(),
    );
});

// And pressing one is the same arithmetic backwards: wrap the selection, or
// take the wrapper off if it is already there — including when the reader
// selected the word rather than the markers around it.
let apply = move |marker: &'static str| {
    let (start, end) = selection.get_untracked();
    let (next, start, end) = toggle_wrap(&text.get_untracked(), start, end, marker);
    text.set(next);
    restore(start, end);
};

<ToggleGroup kind=ToggleGroupType::Multiple value=marks>
    {MARKS.iter().map(|mark| view! {
        <Tooltip>
            <TooltipTrigger>
                <ToggleGroupItem value=mark.slug on:click=move |_| apply(mark.marker)>
                    {icon(mark.slug)}
                </ToggleGroupItem>
            </TooltipTrigger>
            <TooltipContent>{mark.label} <Kbd>{mark.key}</Kbd></TooltipContent>
        </Tooltip>
    }).collect_view()}
</ToggleGroup>

<Textarea model=text node_ref=area on:select=sync on:keyup=sync on:pointerup=sync />

// The offsets a browser reports are UTF-16 code units and Rust indexes bytes,
// so every crossing goes through a conversion. One `é` in the document is all
// it takes for "close enough" to start slicing in the middle of a character.
fn byte_offset(text: &str, utf16: usize) -> usize {
    let mut units = 0;
    for (at, ch) in text.char_indices() {
        if units >= utf16 { return at }
        units += ch.len_utf16();
    }
    text.len()
}"##;

/// One inline mark: what the toggle group calls it, what it is written as, and the key that also
/// does it.
struct Mark {
    slug: &'static str,
    marker: &'static str,
    label: &'static str,
    key: &'static str,
}

const MARKS: &[Mark] = &[
    Mark {
        slug: "bold",
        marker: "**",
        label: "Bold",
        key: "⌘B",
    },
    Mark {
        slug: "italic",
        marker: "_",
        label: "Italic",
        key: "⌘I",
    },
    Mark {
        slug: "strike",
        marker: "~~",
        label: "Strikethrough",
        key: "",
    },
    Mark {
        slug: "code",
        marker: "`",
        label: "Code",
        key: "",
    },
];

/// One block style, which is a prefix on a line and nothing more.
struct Style {
    slug: &'static str,
    prefix: &'static str,
    label: &'static str,
}

const STYLES: &[Style] = &[
    Style {
        slug: "h1",
        prefix: "# ",
        label: "Heading",
    },
    Style {
        slug: "h2",
        prefix: "## ",
        label: "Subheading",
    },
    Style {
        slug: "quote",
        prefix: "> ",
        label: "Quote",
    },
    Style {
        slug: "bullet",
        prefix: "- ",
        label: "Bulleted list",
    },
    Style {
        slug: "number",
        prefix: "1. ",
        label: "Numbered list",
    },
];

/// The path data is Lucide's, the same as the library's own icons.
fn icon(slug: &str) -> AnyView {
    match slug {
        "bold" => view! {
            <IconRoot class="size-4">
                <path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8" />
            </IconRoot>
        }
        .into_any(),
        "italic" => view! {
            <IconRoot class="size-4">
                <path d="M19 4h-9M14 20H5M15 4 9 20" />
            </IconRoot>
        }
        .into_any(),
        "strike" => view! {
            <IconRoot class="size-4">
                <path d="M16 4H9a3 3 0 0 0-2.83 4" />
                <path d="M14 12a4 4 0 0 1 0 8H6" />
                <path d="M4 12h16" />
            </IconRoot>
        }
        .into_any(),
        "code" => view! {
            <IconRoot class="size-4">
                <path d="m16 18 6-6-6-6M8 6l-6 6 6 6" />
            </IconRoot>
        }
        .into_any(),
        "h1" => view! {
            <IconRoot class="size-4">
                <path d="M4 12h8M4 18V6M12 18V6" />
                <path d="m17 12 3-2v8" />
            </IconRoot>
        }
        .into_any(),
        "h2" => view! {
            <IconRoot class="size-4">
                <path d="M4 12h8M4 18V6M12 18V6" />
                <path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1" />
            </IconRoot>
        }
        .into_any(),
        "quote" => view! {
            <IconRoot class="size-4">
                <path d="M17 6H3M21 12H8M21 18H8M3 12v6" />
            </IconRoot>
        }
        .into_any(),
        "bullet" => view! {
            <IconRoot class="size-4">
                <path d="M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01" />
            </IconRoot>
        }
        .into_any(),
        _ => view! {
            <IconRoot class="size-4">
                <path d="M10 6h11M10 12h11M10 18h11" />
                <path d="M4 6h1v4M4 10h2" />
                <path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" />
            </IconRoot>
        }
        .into_any(),
    }
}

// ---------------------------------------------------------------------------
// The arithmetic. A browser counts a textarea's selection in UTF-16 code units
// and Rust indexes bytes, so every crossing between the two goes through a
// conversion — one `é` in the document is all it takes for "close enough" to
// start slicing in the middle of a character.
// ---------------------------------------------------------------------------

fn byte_offset(text: &str, utf16: usize) -> usize {
    let mut units = 0;
    for (at, ch) in text.char_indices() {
        if units >= utf16 {
            return at;
        }
        units += ch.len_utf16();
    }
    text.len()
}

fn utf16_offset(text: &str, byte: usize) -> usize {
    text[..byte.min(text.len())]
        .chars()
        .map(char::len_utf16)
        .sum()
}

fn utf16_len(text: &str) -> usize {
    text.chars().map(char::len_utf16).sum()
}

/// The pair of markers the selection is sitting between, if it is sitting between one.
///
/// Scoped to the line, and decided by counting rather than by looking at what is adjacent: a
/// marker that has been opened and not closed before the caret is what "inside a bold run" means,
/// and it is the only reading under which pressing the button in the middle of a word can do
/// anything sensible. It is also the rule the preview reads by, so the two cannot come to
/// disagree about what is bold.
fn enclosing(text: &str, start: usize, end: usize, marker: &str) -> Option<(usize, usize)> {
    let (line_start, line_end) = line_bounds(text, start);
    // A selection that runs past the end of the line is not one run.
    if end > line_end {
        return None;
    }
    let opened = text[line_start..start].matches(marker).count();
    let inside = text[start..end].matches(marker).count();
    // Opened before the selection, and not closed inside it.
    if opened.is_multiple_of(2) || !inside.is_multiple_of(2) {
        return None;
    }
    let open = line_start + text[line_start..start].rfind(marker)?;
    let close = end + text[end..line_end].find(marker)?;
    Some((open, close))
}

/// Whether the selection already wears `marker` — either because the reader selected the markers
/// along with the text, or because the caret is anywhere inside a run wearing it.
fn is_marked(text: &str, start: usize, end: usize, marker: &str) -> bool {
    let (start, end) = (byte_offset(text, start), byte_offset(text, end));
    let inner = &text[start..end];
    let width = marker.len();

    (inner.len() >= 2 * width && inner.starts_with(marker) && inner.ends_with(marker))
        || enclosing(text, start, end, marker).is_some()
}

/// Puts `marker` around the selection, or takes it off. Returns the new text and where the
/// selection has moved to, in the units the DOM will want it back in.
fn toggle_wrap(text: &str, start: usize, end: usize, marker: &str) -> (String, usize, usize) {
    let (start, end) = (byte_offset(text, start), byte_offset(text, end));
    let inner = &text[start..end];
    let width = marker.len();

    if inner.len() >= 2 * width && inner.starts_with(marker) && inner.ends_with(marker) {
        let stripped = &inner[width..inner.len() - width];
        let next = format!("{}{stripped}{}", &text[..start], &text[end..]);
        let at = utf16_offset(&next, start);
        return (next, at, at + utf16_len(stripped));
    }

    // Inside a run: the pair comes off wherever on the line it happens to be, which is what makes
    // the button work with the caret in the middle of a word rather than only around the whole of
    // one.
    if let Some((open, close)) = enclosing(text, start, end, marker) {
        let next = format!(
            "{}{}{}",
            &text[..open],
            &text[open + width..close],
            &text[close + width..],
        );
        return (
            next.clone(),
            utf16_offset(&next, start - width),
            utf16_offset(&next, end - width),
        );
    }

    let next = format!("{}{marker}{inner}{marker}{}", &text[..start], &text[end..]);
    let at = utf16_offset(&next, start + width);
    (next, at, at + utf16_len(inner))
}

fn line_bounds(text: &str, at: usize) -> (usize, usize) {
    let start = text[..at].rfind('\n').map_or(0, |at| at + 1);
    let end = text[at..]
        .find('\n')
        .map_or(text.len(), |offset| at + offset);
    (start, end)
}

/// A line with its prefix taken off, and which style that prefix was. `## ` before `# `, or every
/// subheading reads as a heading with a stray hash on the front.
fn classify(line: &str) -> (&str, Option<&'static str>) {
    if let Some(rest) = line.strip_prefix("## ") {
        return (rest, Some("h2"));
    }
    if let Some(rest) = line.strip_prefix("# ") {
        return (rest, Some("h1"));
    }
    if let Some(rest) = line.strip_prefix("> ") {
        return (rest, Some("quote"));
    }
    if let Some(rest) = line.strip_prefix("- ") {
        return (rest, Some("bullet"));
    }
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    if digits > 0 && line[digits..].starts_with(". ") {
        return (&line[digits + 2..], Some("number"));
    }
    (line, None)
}

/// The style of the line the caret is on.
fn style_at(text: &str, at: usize) -> Option<&'static str> {
    let at = byte_offset(text, at);
    let (start, end) = line_bounds(text, at);
    classify(&text[start..end]).1
}

/// Sets the line's prefix, or clears it when it is already the one asked for. The caret keeps its
/// place in the words rather than in the line, so re-styling does not move it about.
fn set_style(text: &str, at: usize, slug: &str) -> (String, usize, usize) {
    let at = byte_offset(text, at);
    let (start, end) = line_bounds(text, at);
    let line = &text[start..end];
    let (rest, current) = classify(line);
    let prefix = if current == Some(slug) {
        ""
    } else {
        STYLES
            .iter()
            .find(|style| style.slug == slug)
            .map_or("", |style| style.prefix)
    };

    let removed = line.len() - rest.len();
    let next = format!("{}{prefix}{rest}{}", &text[..start], &text[end..]);
    let within = (at - start).saturating_sub(removed);
    let caret = utf16_offset(&next, start + prefix.len() + within);
    (next, caret, caret)
}

// ---------------------------------------------------------------------------
// The preview. Rendered as views rather than as a string handed to `inner_html`
// — a demo that pastes the reader's own text back into the document as markup
// teaches the wrong lesson, and the runs are what the parser produces anyway.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default, PartialEq)]
struct Runs {
    bold: bool,
    italic: bool,
    strike: bool,
    code: bool,
}

impl Runs {
    fn class(self) -> String {
        let mut class = String::new();
        if self.bold {
            class.push_str("font-semibold ");
        }
        if self.italic {
            class.push_str("italic ");
        }
        if self.strike {
            class.push_str("line-through ");
        }
        if self.code {
            class.push_str("rounded bg-muted px-1 py-0.5 font-mono text-[0.85em] ");
        }
        class
    }
}

fn flush(out: &mut Vec<(Runs, String)>, buffer: &mut String, marks: Runs) {
    if !buffer.is_empty() {
        out.push((marks, std::mem::take(buffer)));
    }
}

/// Splits a line into runs of text that wear the same marks. Inside a code span nothing else
/// counts, which is the whole point of a code span.
fn runs(line: &str) -> Vec<(Runs, String)> {
    let chars: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut buffer = String::new();
    let mut marks = Runs::default();
    let mut at = 0;

    while at < chars.len() {
        let pair = chars[at] == chars.get(at + 1).copied().unwrap_or('\0');
        if chars[at] == '`' {
            flush(&mut out, &mut buffer, marks);
            marks.code = !marks.code;
            at += 1;
        } else if !marks.code && chars[at] == '*' && pair {
            flush(&mut out, &mut buffer, marks);
            marks.bold = !marks.bold;
            at += 2;
        } else if !marks.code && chars[at] == '~' && pair {
            flush(&mut out, &mut buffer, marks);
            marks.strike = !marks.strike;
            at += 2;
        } else if !marks.code && chars[at] == '_' {
            flush(&mut out, &mut buffer, marks);
            marks.italic = !marks.italic;
            at += 1;
        } else {
            buffer.push(chars[at]);
            at += 1;
        }
    }

    flush(&mut out, &mut buffer, marks);
    out
}

fn inline(line: &str) -> AnyView {
    runs(line)
        .into_iter()
        .map(|(marks, text)| view! { <span class=marks.class()>{text}</span> })
        .collect_view()
        .into_any()
}

enum Chunk {
    Heading(&'static str, String),
    Quote(String),
    List(bool, Vec<String>),
    Para(Vec<String>),
}

fn chunks(text: &str) -> Vec<Chunk> {
    let mut out: Vec<Chunk> = Vec::new();

    for line in text.lines() {
        let (rest, style) = classify(line);
        let rest = rest.to_string();
        match style {
            Some(slug @ ("h1" | "h2")) => out.push(Chunk::Heading(slug, rest)),
            Some("quote") => out.push(Chunk::Quote(rest)),
            Some(slug @ ("bullet" | "number")) => {
                let ordered = slug == "number";
                match out.last_mut() {
                    Some(Chunk::List(kind, items)) if *kind == ordered => items.push(rest),
                    _ => out.push(Chunk::List(ordered, vec![rest])),
                }
            }
            // A blank line is what ends a paragraph; two plain lines in a row are one.
            _ if rest.trim().is_empty() => out.push(Chunk::Para(Vec::new())),
            _ => match out.last_mut() {
                Some(Chunk::Para(lines)) if !lines.is_empty() => lines.push(rest),
                _ => out.push(Chunk::Para(vec![rest])),
            },
        }
    }

    out.into_iter()
        .filter(|chunk| !matches!(chunk, Chunk::Para(lines) if lines.is_empty()))
        .collect()
}

fn preview(text: &str) -> AnyView {
    chunks(text)
        .into_iter()
        .map(|chunk| match chunk {
            Chunk::Heading("h1", line) => {
                view! { <h3 class="text-xl font-semibold tracking-tight">{inline(&line)}</h3> }
                    .into_any()
            }
            Chunk::Heading(_, line) => {
                view! { <h4 class="text-base font-semibold tracking-tight">{inline(&line)}</h4> }
                    .into_any()
            }
            Chunk::Quote(line) => view! {
                <blockquote class="border-l-2 pl-3 text-muted-foreground italic">
                    {inline(&line)}
                </blockquote>
            }
            .into_any(),
            Chunk::List(ordered, items) => {
                let rendered = items
                    .into_iter()
                    .map(|item| view! { <li>{inline(&item)}</li> })
                    .collect_view();
                if ordered {
                    view! { <ol class="list-decimal pl-5">{rendered}</ol> }.into_any()
                } else {
                    view! { <ul class="list-disc pl-5">{rendered}</ul> }.into_any()
                }
            }
            Chunk::Para(lines) => view! { <p>{inline(&lines.join(" "))}</p> }.into_any(),
        })
        .collect_view()
        .into_any()
}

const SEED: &str = "# Release notes\n\nThe canvas no longer **pans on the primary button**, which is what a board \
needs to have a click of its own. Hold `Space` and drag instead.\n\n## What changed\n\n\
- `pan_on_drag` is off by default\n- `on_surface_click` reports a _world_ coordinate\n\
- ~~Drag anywhere~~ is now the hand tool, opt in\n\n> Every gesture a surface takes is one the \
caller cannot have.";

fn editor() -> AnyView {
    let text = RwSignal::new(SEED.to_string());
    let area = AnyNodeRef::new();
    let selection = RwSignal::new((0usize, 0usize));
    let marks = RwSignal::new(Vec::<String>::new());
    let style = RwSignal::new(Vec::<String>::new());

    let element = move || {
        area.get_untracked()
            .and_then(|node| node.dyn_into::<HtmlTextAreaElement>().ok())
    };

    let sync = move || {
        if let Some(element) = element() {
            let start = element.selection_start().ok().flatten().unwrap_or(0) as usize;
            let end = element.selection_end().ok().flatten().unwrap_or(0) as usize;
            selection.set((start, end));
        }
    };

    // Put the caret back where the edit left it. A frame later, because the value the textarea is
    // holding is the one from before the signal changed, and a selection set against it lands in
    // the wrong place.
    let restore = move |start: usize, end: usize| {
        selection.set((start, end));
        set_timeout(
            move || {
                if let Some(element) = element() {
                    let _ = element.focus();
                    let _ = element.set_selection_range(start as u32, end as u32);
                }
            },
            Duration::from_millis(0),
        );
    };

    // Nothing remembers what is pressed. It is read back out of the text every time either the
    // text or the selection moves, which is the only way a toggle can claim to describe something
    // it does not own.
    Effect::new(move |_| {
        let text = text.get();
        let (start, end) = selection.get();
        marks.set(
            MARKS
                .iter()
                .filter(|mark| is_marked(&text, start, end, mark.marker))
                .map(|mark| mark.slug.to_string())
                .collect(),
        );
        style.set(
            style_at(&text, start)
                .map(str::to_string)
                .into_iter()
                .collect(),
        );
    });

    let apply_mark = move |marker: &'static str| {
        let (start, end) = selection.get_untracked();
        let (next, start, end) = toggle_wrap(&text.get_untracked(), start, end, marker);
        text.set(next);
        restore(start, end);
    };

    let apply_style = move |slug: &'static str| {
        let (start, _) = selection.get_untracked();
        let (next, start, end) = set_style(&text.get_untracked(), start, slug);
        text.set(next);
        restore(start, end);
    };

    let words = Signal::derive(move || text.with(|text| text.split_whitespace().count()));

    view! {
        <div class="flex w-full flex-col gap-3">
            <Tabs default_value="write" class="flex flex-col gap-3">
                <div class="flex flex-wrap items-center gap-2">
                    <ToggleGroup
                        kind=ToggleGroupType::Multiple
                        variant=ToggleVariant::Outline
                        size=ToggleSize::Sm
                        value=marks
                        class="gap-1"
                    >
                        {MARKS
                            .iter()
                            .map(|mark| {
                                view! {
                                    <Tooltip>
                                        <TooltipTrigger>
                                            <ToggleGroupItem
                                                value=mark.slug
                                                class="rounded-lg"
                                                on:click=move |_| apply_mark(mark.marker)
                                            >
                                                {icon(mark.slug)}
                                            </ToggleGroupItem>
                                        </TooltipTrigger>
                                        <TooltipContent class="flex items-center gap-1.5">
                                            {mark.label}
                                            {(!mark.key.is_empty())
                                                .then(|| view! { <Kbd>{mark.key}</Kbd> })}
                                        </TooltipContent>
                                    </Tooltip>
                                }
                            })
                            .collect_view()}
                    </ToggleGroup>

                    // The height has to carry the same variant the component wrote it under, or
                    // `tw_merge` keeps both and `h-full` on an auto-height row resolves to nothing.
                    <Separator
                        orientation=Orientation::Vertical
                        class="data-[orientation=vertical]:h-6"
                    />

                    <ToggleGroup
                        kind=ToggleGroupType::Single
                        variant=ToggleVariant::Outline
                        size=ToggleSize::Sm
                        value=style
                        class="gap-1"
                    >
                        {STYLES
                            .iter()
                            .map(|item| {
                                view! {
                                    <Tooltip>
                                        <TooltipTrigger>
                                            <ToggleGroupItem
                                                value=item.slug
                                                class="rounded-lg"
                                                on:click=move |_| apply_style(item.slug)
                                            >
                                                {icon(item.slug)}
                                            </ToggleGroupItem>
                                        </TooltipTrigger>
                                        <TooltipContent>{item.label}</TooltipContent>
                                    </Tooltip>
                                }
                            })
                            .collect_view()}
                    </ToggleGroup>

                    <div class="ml-auto flex items-center gap-2">
                        <Badge variant=BadgeVariant::Secondary>
                            {move || format!("{} words", words.get())}
                        </Badge>
                        <TabsList>
                            <TabsTrigger value="write">"Write"</TabsTrigger>
                            <TabsTrigger value="preview">"Preview"</TabsTrigger>
                        </TabsList>
                    </div>
                </div>

                <TabsContent value="write">
                    <Textarea
                        model=text
                        node_ref=area
                        class="min-h-56 font-mono text-sm leading-relaxed"
                        on:select=move |_| sync()
                        on:keyup=move |_| sync()
                        on:pointerup=move |_| sync()
                        on:focus=move |_| sync()
                        on:keydown=move |e: ev::KeyboardEvent| {
                            if e.ctrl_key() || e.meta_key() {
                                match e.key().to_lowercase().as_str() {
                                    "b" => {
                                        e.prevent_default();
                                        apply_mark("**");
                                    }
                                    "i" => {
                                        e.prevent_default();
                                        apply_mark("_");
                                    }
                                    _ => {}
                                }
                            }
                        }
                    />
                </TabsContent>

                <TabsContent value="preview">
                    <div class="flex min-h-56 flex-col gap-3 rounded-lg border p-4 text-sm leading-relaxed">
                        {move || preview(&text.get())}
                    </div>
                </TabsContent>
            </Tabs>

            <div class="flex items-center justify-between gap-2">
                <p class="text-xs text-muted-foreground">
                    "Select something and press a mark, or use " <Kbd>"⌘B"</Kbd> " and "
                    <Kbd>"⌘I"</Kbd>
                    ". The toolbar reads the selection back, so it stays right when
                    you move the caret."
                </p>
                <Button
                    variant=ButtonVariant::Outline
                    size=ButtonSize::Sm
                    on:click=move |_| {
                        text.set(SEED.to_string());
                        restore(0, 0);
                    }
                >
                    "Reset"
                </Button>
            </div>
        </div>
    }
    .into_any()
}

pub const BLOCKS: &[Block] = &[Block {
    slug: "a-toolbar-that-reads-the-selection",
    title: "A toolbar that reads the selection",
    uses: &[
        "badge",
        "button",
        "kbd",
        "separator",
        "tabs",
        "textarea",
        "toggle-group",
        "tooltip",
    ],
    description: "A small markdown editor, and an answer to the question a formatting toolbar always \
                  raises: is a bold button a toggle or an action? It is a toggle, but only if it \
                  tells the truth — so nothing here remembers what was last pressed. The marks are \
                  read back out of the text under the caret every time either the text or the \
                  selection moves, and pressing one rewrites the text so that the next read agrees. \
                  Move the caret into a bold word and the button lights up on its own. Two things \
                  make that possible: `Textarea` takes a `node_ref`, because the selection is the \
                  one piece of a textarea's state that lives nowhere but the DOM; and every \
                  crossing between the two coordinate systems is converted, since a browser counts \
                  the selection in UTF-16 code units and Rust indexes bytes — one accented \
                  character is all it takes for \"close enough\" to slice through the middle of one. \
                  The preview is built as views rather than handed to `inner_html`, which a demo \
                  that echoes the reader's own text back into the document has no business doing.",
    code: EDITOR,
    view: editor,
}];
