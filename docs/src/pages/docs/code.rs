use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::code::{Code, CodeBlock},
    utils::types::Language,
};

const LANGUAGES: &str = r#"<CodeBlock label="toast.ts" code=TS_SAMPLE language=Language::TypeScript />
<CodeBlock label="index.html" code=HTML_SAMPLE language=Language::Html />
<CodeBlock label="globals.css" code=CSS_SAMPLE language=Language::Css />"#;

const LONG_LINES: &str = r#"<CodeBlock code="let placement = get_placement(Side::Bottom, Align::Start); // a deliberately long line to force the block to scroll sideways instead of stretching its container" />"#;

// Snippets live outside `view!` on purpose: leptosfmt reindents raw strings that sit inside the
// macro, which would silently rewrite what the reader copies.
const INLINE: &str = r#"<p>
    "Wrap the tree in " <Code>"ThemeProvider"</Code> " to enable dark mode."
</p>"#;

const BLOCK: &str =
    r#"<CodeBlock code="cargo add nocomponents --features full" language=Language::Shell />"#;

const LABELLED: &str = r#"<CodeBlock
    label="src/main.rs"
    code="fn main() {\n    leptos::mount::mount_to_body(App)\n}"
/>"#;

const NO_COPY: &str = r#"<CodeBlock
    code="rustup target add wasm32-unknown-unknown"
    language=Language::Shell
    copyable=false
/>"#;

const TS_SAMPLE: &str = r#"interface Toast {
  title: string;
  duration?: number;
}

export function notify({ title, duration = 4000 }: Toast) {
  // strings, numbers and types all get their own colour
  return `${title} for ${duration}ms`;
}"#;

const HTML_SAMPLE: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <link data-trunk rel="tailwind-css" href="styles/globals.css" />
  </head>
  <body class=""></body>
</html>"#;

const CSS_SAMPLE: &str = r#"@custom-variant dark (&:is(.dark *));

.card {
  background-color: #fff;
  padding: 1.5rem; /* comments are dimmed */
}"#;

const IN_DEMO: &str = r##"<DemoSection
    title="Variants"
    code=r#"<Toggle>"Default"</Toggle>"#
>
    <Toggle>"Default"</Toggle>
</DemoSection>"##;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Code",
        description: "Inline code, for a symbol or a fragment mentioned in running text.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the inline code classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The symbol or fragment.",
            },
        ],
    },
    ApiEntry {
        name: "CodeBlock",
        description: "A block of code with an optional filename header and a copy button.",
        props: &[
            Prop {
                name: "code",
                ty: "String",
                default: "",
                description: "The snippet, rendered as it is written.",
            },
            Prop {
                name: "label",
                ty: "Option<String>",
                default: "None",
                description: "Shown in the header — usually a file name or a short caption.",
            },
            Prop {
                name: "language",
                ty: "Language",
                default: "Rust",
                description: "One of: Rust, Shell, JavaScript, TypeScript, Html, Css, Plain.",
            },
            Prop {
                name: "copyable",
                ty: "bool",
                default: "true",
                description: "Draw the copy button.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the block's classes.",
            },
        ],
    },
    ApiEntry {
        name: "CodeCopyButton",
        description: "The copy trigger for the enclosing block. Swaps to a check for a moment after copying.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the button's classes.",
        }],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Code"
            description="Displays code inline or as a block, with a copy button."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Inline"
                    description="For a symbol or fragment mentioned in running text."
                    code=INLINE
                >
                    <p class="text-sm">
                        "Wrap the tree in " <Code>"ThemeProvider"</Code> " to enable dark mode."
                    </p>
                </DemoSection>

                <DemoSection
                    title="Block"
                    description="Hover the block to reveal the copy button; it acknowledges with a check."
                    code=BLOCK
                >
                    <CodeBlock
                        code="cargo add nocomponents --features full"
                        language=Language::Shell
                    />
                </DemoSection>

                <DemoSection
                    title="With a label"
                    description="The header usually carries a file name."
                    code=LABELLED
                >
                    <CodeBlock
                        label="src/main.rs"
                        code="fn main() {\n    leptos::mount::mount_to_body(App)\n}"
                    />
                </DemoSection>

                <DemoSection
                    title="Without the copy button"
                    description="Pass copyable=false when the snippet is not meant to be lifted."
                    code=NO_COPY
                >
                    <CodeBlock
                        code="rustup target add wasm32-unknown-unknown"
                        language=Language::Shell
                        copyable=false
                    />
                </DemoSection>

                <DemoSection
                    title="Long lines"
                    description="A block scrolls horizontally rather than widening the page."
                    code=LONG_LINES
                >
                    <CodeBlock code="let placement = get_placement(Side::Bottom, Align::Start); // a deliberately long line to force the block to scroll sideways instead of stretching its container" />
                </DemoSection>

                <DemoSection
                    title="Languages"
                    description="Rust and shell plus TypeScript, JavaScript, HTML and CSS. Language::Plain opts out."
                    code=LANGUAGES
                >
                    <div class="flex flex-col gap-3">
                        <CodeBlock label="toast.ts" code=TS_SAMPLE language=Language::TypeScript />
                        <CodeBlock label="index.html" code=HTML_SAMPLE language=Language::Html />
                        <CodeBlock label="globals.css" code=CSS_SAMPLE language=Language::Css />
                    </div>
                </DemoSection>

                <DemoSection
                    title="In the docs"
                    description="Every DemoSection takes an optional code prop, so a demo and its snippet stay together."
                    code=IN_DEMO
                >
                    <p class="text-sm text-muted-foreground">
                        "The snippet under each demo on these pages comes from that prop."
                    </p>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
