//! The theme page: what `ThemeProvider` puts on `<html>`, and what a stylesheet does about it.

use crate::components::{
    api_table::{ApiEntry, ApiReference, Prop},
    demo_section::DemoSection,
    theme_switcher::{ThemeSwitcher, ThemeSwitcherPopover},
};
use crate::layout::doc_layout::DocLayout;
use leptos::prelude::*;
use nocomponents::{
    components::{code::CodeBlock, separator::Separator},
    utils::{
        theme::{Theme, use_theme},
        types::{Language, Orientation},
    },
};

const PROVIDER: &str = r#"use nocomponents::utils::theme::ThemeProvider;

view! {
    // Every default is for a first visit only: anything stored wins.
    <ThemeProvider default_color="ctp-mocha" default_accent="mauve" default_radius="0.5rem">
        <App />
    </ThemeProvider>
}"#;

const READING: &str = r#"let theme = use_theme();

// The three signals, all writable.
theme.current.get();   // Theme::Light | Dark | System — what was asked for
theme.resolved.get();  // Light or Dark — never System
theme.color.get();     // the palette's name, "" for the base one
theme.accent.get();    // the accent's name, "" for the palette's own
theme.radius.get();    // a CSS length, "" for the palette's own

// And the setters, which reject what the attribute or the property could not hold.
theme.set_theme(Theme::Dark);
theme.set_color("grove");
theme.set_accent("mauve");
theme.set_radius("0.75rem");"#;

const PALETTE_CSS: &str = r#"/* A palette is the tokens that differ, twice. Everything else falls
   through to :root, which is why a theme is a dozen lines rather than
   a second copy of the whole scale. */

[data-theme="grove"] {
  --radius: 0.625rem;
  --primary: oklch(0.52 0.13 158);
  --primary-foreground: oklch(0.985 0 0);
  --ring: oklch(0.52 0.13 158);
}

/* The compound is the real one — both attributes land on <html>. The
   descendant form is for a palette scoped to a subtree. */

[data-theme="grove"].dark,
.dark [data-theme="grove"] {
  --primary: oklch(0.75 0.15 158);
  --primary-foreground: oklch(0.21 0.05 158);
  --ring: oklch(0.6 0.13 158);
}"#;

const ACCENT_CSS: &str = r#"/* The ramp: one definition per name per mode, so that
   data-accent="mauve" means the same colour on every palette. A palette
   with its own opinion redefines the ramp, not the tokens under it. */

:root { --ac-mauve: oklch(0.555 0.25 297); /* … */ }
.dark { --ac-mauve: oklch(0.787 0.119 304.8); /* … */ }

/* Applied last, and at :root[data-accent] so it outweighs the
   [data-theme].dark pairs above. */

:root[data-accent] {
  --nc-accent: var(--ac-lavender);  /* an unknown name still lands somewhere */
  --primary: var(--nc-accent);
  --ring: var(--nc-accent);
  --sidebar-primary: var(--nc-accent);
  /* Which way the text on an accent goes is the accent's own lightness,
     not the mode's: Latte's yellow is lighter than Mocha's blue. */
  --primary-foreground: oklch(from var(--nc-accent) clamp(0, (0.62 - l) * 1000, 1) 0 0);

  /* And the tinted surface — hover, a focused item, the active sidebar
     row. Same hue, held at a lightness that sits under text. Without
     this a palette that hand-wrote a green one keeps every hover green
     while the brand goes orange. */
  --accent: oklch(from var(--nc-accent) 0.95 0.03 h);
  --accent-foreground: oklch(from var(--nc-accent) 0.35 0.09 h);
  --sidebar-accent: var(--accent);
}

:root[data-accent].dark {
  --accent: oklch(from var(--nc-accent) 0.32 0.05 h);
  --accent-foreground: oklch(from var(--nc-accent) 0.95 0.02 h);
}

:root[data-accent="mauve"] { --nc-accent: var(--ac-mauve); }"#;

const RADIUS: &str = r#"let theme = use_theme();

// A number and a unit. Anything else — calc(), a bare number, a
// stray declaration — is ignored rather than written.
theme.set_radius("0.75rem");
theme.set_radius("8px");

// And back to whatever the palette says its corners are.
theme.set_radius("");"#;

const SWITCHER: &str = r#"// Mode, palette and accent side by side.
<ThemeSwitcherPopover />

// Mode and palette in a menu, for a header with less room.
<ThemeSwitcher />"#;

/// The radius scale, which `@theme inline` derives from a single `--radius` — so a palette that
/// sets one number moves every corner in the library at once. Its own row rather than an
/// eleventh swatch: the thing to look at is the shape, and a colour chip cannot show it.
const RADII: [(&str, &str); 5] = [
    ("rounded-sm", "sm"),
    ("rounded-md", "md"),
    ("rounded-lg", "lg"),
    ("rounded-xl", "xl"),
    ("rounded-2xl", "2xl"),
];

/// The tokens worth showing as swatches, and what each one is for. Class names are literals
/// because Tailwind finds them by scanning the source — one assembled from `token` would not
/// generate anything.
const TOKENS: [(&str, &str, &str); 10] = [
    ("bg-background", "background", "The page."),
    ("bg-card", "card", "A raised surface."),
    ("bg-popover", "popover", "A floating one."),
    ("bg-primary", "primary", "The accent, or the palette's own."),
    ("bg-secondary", "secondary", "The quieter button."),
    ("bg-muted", "muted", "A filled-in nothing."),
    (
        "bg-accent",
        "accent",
        "Hover and focus. Not the accent hue.",
    ),
    ("bg-destructive", "destructive", "The one you cannot undo."),
    ("bg-border", "border", "Every hairline."),
    ("bg-sidebar", "sidebar", "The nav's own background."),
];

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "ThemeProvider",
        description: "Reads the three stored values, resolves System, and writes the result to `<html>`. One per app, outside the router.",
        props: &[
            Prop {
                name: "default_color",
                ty: "String",
                default: "\"\"",
                description: "The palette for a first visit with nothing stored. Empty is the base one.",
            },
            Prop {
                name: "default_accent",
                ty: "String",
                default: "\"\"",
                description: "The accent for a first visit. Empty leaves the palette's own `--primary` alone.",
            },
            Prop {
                name: "default_radius",
                ty: "String",
                default: "\"\"",
                description: "A corner radius for a first visit, as a CSS length. Empty leaves the palette's own alone.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The app. Everything inside can call `use_theme`.",
            },
        ],
    },
    ApiEntry {
        name: "ThemeContext",
        description: "What `use_theme()` returns. `Copy`, so it can be captured by any number of closures.",
        props: &[
            Prop {
                name: "current",
                ty: "RwSignal<Theme>",
                default: "System",
                description: "The mode that was asked for. Persisted under `ui-theme`.",
            },
            Prop {
                name: "color",
                ty: "RwSignal<String>",
                default: "\"\"",
                description: "The palette's name, written to `data-theme`. Persisted under `ui-color-theme`.",
            },
            Prop {
                name: "accent",
                ty: "RwSignal<String>",
                default: "\"\"",
                description: "The accent's name, written to `data-accent`. Persisted under `ui-accent`.",
            },
            Prop {
                name: "radius",
                ty: "RwSignal<String>",
                default: "\"\"",
                description: "A custom corner radius as a CSS length, written to `--radius` in `<html>`'s inline style. Persisted under `ui-radius`.",
            },
            Prop {
                name: "resolved",
                ty: "Signal<Theme>",
                default: "",
                description: "`current` with System answered — Light or Dark, and live: it follows the OS mid-session.",
            },
            Prop {
                name: "set_theme",
                ty: "fn(Theme)",
                default: "",
                description: "Sets the mode.",
            },
            Prop {
                name: "set_color",
                ty: "fn(impl Into<String>)",
                default: "",
                description: "Sets the palette. A name that is not lowercase letters, digits and hyphens is ignored.",
            },
            Prop {
                name: "set_accent",
                ty: "fn(impl Into<String>)",
                default: "",
                description: "Sets the accent, validated the same way.",
            },
            Prop {
                name: "set_radius",
                ty: "fn(impl Into<String>)",
                default: "",
                description: "Sets the radius. A number and a unit; `calc()` and the rest of the length grammar are deliberately not accepted.",
            },
            Prop {
                name: "is_dark",
                ty: "fn() -> bool",
                default: "",
                description: "`resolved() == Theme::Dark`, for a component that has to branch rather than style.",
            },
        ],
    },
    ApiEntry {
        name: "Theme",
        description: "The mode, and only the mode. `Theme::ALL` is the three of them in the order a switcher offers them.",
        props: &[
            Prop {
                name: "as_str",
                ty: "fn() -> &'static str",
                default: "",
                description: "`\"light\"`, `\"dark\"`, `\"system\"` — what goes into storage.",
            },
            Prop {
                name: "label",
                ty: "fn() -> &'static str",
                default: "",
                description: "The capitalised name, for a switcher.",
            },
        ],
    },
];

/// What `ThemeProvider` has actually written on `<html>`, read back out of the same signals the
/// switcher writes.
#[component]
fn HtmlReadout() -> impl IntoView {
    let theme = use_theme();

    let attrs = move || {
        let mut out = String::from("<html");
        if theme.resolved.get() == Theme::Dark {
            out.push_str(" class=\"dark\"");
        }
        theme.color.with(|color| {
            if !color.is_empty() {
                out.push_str(&format!(" data-theme=\"{color}\""));
            }
        });
        theme.accent.with(|accent| {
            if !accent.is_empty() {
                out.push_str(&format!(" data-accent=\"{accent}\""));
            }
        });
        theme.radius.with(|radius| {
            if !radius.is_empty() {
                out.push_str(&format!(" style=\"--radius: {radius}\""));
            }
        });
        out.push('>');
        out
    };

    view! {
        <div class="flex flex-col gap-3">
            <pre class="overflow-x-auto rounded-lg bg-muted/60 px-3 py-2 font-mono text-xs text-foreground">
                {attrs}
            </pre>
            <div class="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
                <span>"ui-theme"</span>
                <code class="rounded bg-muted px-1 py-0.5 text-foreground">
                    {move || theme.current.get().as_str()}
                </code>
                <Separator orientation=Orientation::Vertical class="h-3" />
                <span>"ui-color-theme"</span>
                <code class="rounded bg-muted px-1 py-0.5 text-foreground">
                    {move || {
                        let color = theme.color.get();
                        if color.is_empty() { "\"\"".to_string() } else { color }
                    }}
                </code>
                <Separator orientation=Orientation::Vertical class="h-3" />
                <span>"ui-accent"</span>
                <code class="rounded bg-muted px-1 py-0.5 text-foreground">
                    {move || {
                        let accent = theme.accent.get();
                        if accent.is_empty() { "\"\"".to_string() } else { accent }
                    }}
                </code>
                <Separator orientation=Orientation::Vertical class="h-3" />
                <span>"ui-radius"</span>
                <code class="rounded bg-muted px-1 py-0.5 text-foreground">
                    {move || {
                        let radius = theme.radius.get();
                        if radius.is_empty() { "\"\"".to_string() } else { radius }
                    }}
                </code>
            </div>
        </div>
    }
}

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Theme"
            description="Four axes on <html>: a mode, a palette, an accent inside it, and a corner radius."
        >
            <div class="flex flex-col gap-8">
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "Light and dark is a " <span class="font-medium text-foreground">"mode"</span>
                    " — the one axis the operating system has an opinion about. A "
                    <span class="font-medium text-foreground">"palette"</span>
                    " is a different set of tokens entirely, and it has a light and a dark of its
                    own. An " <span class="font-medium text-foreground">"accent"</span>
                    " is one hue picked inside whichever palette is in force, and leaves the rest
                    of it alone. Folding them into one enum would mean a consumer with two brands
                    could not say \"brand two, dark\" at all — so they are three names, and the
                    switcher in the header moves any one of them without touching the others. The
                    fourth is the " <span class="font-medium text-foreground">"corner radius"</span>
                    ", which is the odd one out: not a name a stylesheet defines but a length the
                    reader picks, so it is written straight into "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                        "--radius"
                    </code> " on the element itself, where it outranks every palette that set it."
                </p>

                <DemoSection
                    title="The switcher"
                    description="Both of them are in docs/src/components/theme_switcher.rs — nothing here ships in the library, because what a switcher should look like is not a library's business."
                    code=SWITCHER
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <ThemeSwitcherPopover />
                        <ThemeSwitcher />
                    </div>
                </DemoSection>

                <DemoSection
                    title="What lands on <html>"
                    description="Live. The mode is a class because that is what Tailwind's dark: variant reads; the other two are attributes because CSS matches them without a variant at all."
                >
                    <HtmlReadout />
                </DemoSection>

                <DemoSection
                    title="The tokens"
                    description="Every one of these is a CSS custom property a palette may redefine — the colours, and the one number every corner in the library is derived from. Change the palette or the accent above and watch which of them move."
                >
                    <div class="grid grid-cols-2 gap-3 sm:grid-cols-5">
                        {TOKENS
                            .into_iter()
                            .map(|(class, name, description)| {
                                view! {
                                    <div class="flex flex-col gap-1.5">
                                        <div class=format!(
                                            "h-10 w-full rounded-md ring-1 ring-inset ring-foreground/15 {class}",
                                        ) />
                                        <div class="flex flex-col">
                                            <span class="font-mono text-[0.7rem] text-foreground">
                                                {name}
                                            </span>
                                            <span class="text-[0.7rem] leading-tight text-muted-foreground">
                                                {description}
                                            </span>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>

                    <Separator class="my-5" />

                    <div class="flex flex-col gap-2">
                        <div class="flex flex-wrap items-end gap-3">
                            {RADII
                                .into_iter()
                                .map(|(class, name)| {
                                    view! {
                                        <div class="flex flex-col items-center gap-1.5">
                                            <div class=format!(
                                                "size-12 bg-muted ring-1 ring-inset ring-foreground/15 {class}",
                                            ) />
                                            <span class="font-mono text-[0.7rem] text-muted-foreground">
                                                {name}
                                            </span>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                        <div class="flex flex-col">
                            <span class="font-mono text-[0.7rem] text-foreground">"radius"</span>
                            <span class="text-[0.7rem] leading-tight text-muted-foreground">
                                "One number, and the scale is calc() off it. Default is 0, Grove 0.625rem, Ember 1rem, Catppuccin 0.5rem — and the switcher's slider overrides all of them."
                            </span>
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Setting it up"
                    description="One provider, outside the router, wrapping everything that might ask."
                >
                    <CodeBlock code=PROVIDER language=Language::Rust label="app.rs" />
                </DemoSection>

                <DemoSection
                    title="Reading it"
                    description="use_theme() anywhere inside the provider. The context is Copy, so a closure can keep it."
                >
                    <CodeBlock code=READING language=Language::Rust />
                </DemoSection>

                <DemoSection
                    title="Defining a palette"
                    description="Nothing in Rust knows what palettes exist: a name is whatever the stylesheet defines, and the empty name is :root."
                >
                    <CodeBlock code=PALETTE_CSS language=Language::Css label="globals.css" />
                </DemoSection>

                <DemoSection
                    title="A custom radius"
                    description="The one axis with nothing to enumerate. The slider writes --radius into <html>'s own style, which is the only thing a [data-theme] rule cannot outrank — and the popover's own corners are drawn from it, so the panel is its own preview."
                >
                    <CodeBlock code=RADIUS language=Language::Rust />
                </DemoSection>

                <DemoSection
                    title="Defining an accent"
                    description="The docs site ships Catppuccin's fourteen. The names are shared across every palette, so a palette may redefine what they look like in it."
                >
                    <CodeBlock code=ACCENT_CSS language=Language::Css label="globals.css" />
                </DemoSection>

                <DemoSection
                    title="Catppuccin"
                    description="Three palettes, not four. Latte is the light half of all three rather than a palette of its own — which is how the flavours are shipped everywhere else, and what keeps the mode axis meaning what it means for every other palette. Which is also why the switcher offers three rows in dark and one row saying Latte in light: under that mode they are three names for the same screen."
                >
                    <p class="text-sm leading-relaxed text-muted-foreground">
                        "Their accent ramps are in the accent grid, and each flavour retunes the
                        fourteen so that \"mauve\" sits correctly in it. The values are generated
                        from "
                        <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                            "catppuccin/palette"
                        </code> " rather than eyeballed, in oklch, which is the unit the rest of the
                        tokens are already written in."
                    </p>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
