//! The icons page: the one slot every component takes an icon through, the `Icon` wrapper, and
//! the fallback glyphs a project installs and owns.

use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{
        accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
        button::{Button, ButtonSize, ButtonVariant},
        code::CodeBlock,
        separator::Separator,
        spinner::Spinner,
    },
    icons::Icon,
    utils::types::Language,
};

const WITHOUT: &str = r#"<AccordionItem value="defaults">
    <AccordionTrigger>"Draws its own chevron"</AccordionTrigger>
    <AccordionContent>"Nothing passed, so the installed fallback."</AccordionContent>
</AccordionItem>"#;

const WITH: &str = r#"<AccordionItem value="custom">
    <AccordionTrigger icon=|| view! { <Caret /> }>"Draws yours"</AccordionTrigger>
    <AccordionContent>"Sized, coloured and turned by the trigger."</AccordionContent>
</AccordionItem>"#;

const CARET: &str = r#"// A filled icon on a narrow canvas: nothing about it is 24 units or stroked.
#[component]
pub fn Caret(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 320 512" class=class>
            <path d="M40 192h240L160 352z" />
        </Icon>
    }
}"#;

const DUOTONE: &str = r#"// Two layers and an opacity on one of them — any SVG children come through.
#[component]
pub fn CircleCheckDuotone(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 512 512" class=class>
            <circle cx="256" cy="256" r="240" opacity="0.4" />
            <path d="M226 346 130 250l34-34 62 62 122-122 34 34z" />
        </Icon>
    }
}

<Spinner icon=|| view! { <CircleCheckDuotone /> } />"#;

const ICON_ONLY: &str = r#"// The button is named, the icon is decoration inside it.
<Button size=ButtonSize::Icon variant=ButtonVariant::Outline aria_label="Mark done">
    <CircleCheckDuotone />
</Button>

// An icon standing on its own, with nothing to name it, names itself.
<CircleCheckDuotone label="Done" />"#;

const INSTALLED: &str = r#"cargo nocli components add select

#   added     src/components/nc/select.rs
#   needed by src/icons/nc/check.rs
#   needed by src/icons/nc/chevron.rs
#   needed by src/icons/nc/icon.rs

cargo nocli icons list           # every glyph there is, * marks the installed ones
cargo nocli icons add x search   # install more, for your own use"#;

const REDRAW: &str = r#"// src/icons/nc/chevron.rs — yours. Redraw it and every component
// that falls back on a chevron draws the new one.
#[component]
pub fn ChevronDown(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 448 512" class=move || cn!("size-4", class.get())>
            <path d="…your own artwork…" />
        </Icon>
    }
}"#;

const OWN_SET: &str = r#"// src/icons/mod.rs — the nocli block declares `nc`; the rest is yours.
// nocli:begin
pub mod nc;
// nocli:end

pub mod app;   // generated from your icon set, one `view!` function per icon

// Anywhere:
<SelectTrigger icon=|| view! { <app::ChevronDown /> }>…</SelectTrigger>"#;

/// The demo's narrow, filled caret. Original artwork, drawn for this page.
#[component]
fn Caret(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <Icon view_box="0 0 320 512" class=class>
            <path d="M40 192h240L160 352z" />
        </Icon>
    }
}

/// The demo's two-layer icon. Original artwork, drawn for this page.
#[component]
fn CircleCheckDuotone(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] label: Signal<String>,
) -> impl IntoView {
    view! {
        <Icon view_box="0 0 512 512" class=class label=label>
            <circle cx="256" cy="256" r="240" opacity="0.4" />
            <path d="M226 346 130 250l34-34 62 62 122-122 34 34z" />
        </Icon>
    }
}

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Icon",
        description: "The wrapper every icon is drawn with, the fallbacks and yours alike. Lives in the installed icon directory (`icons::icon::Icon`).",
        props: &[
            Prop {
                name: "view_box",
                ty: "Signal<String>",
                default: "",
                description: "The artwork's own viewBox, copied from its `<svg>`. Required — there is no viewBox right for every set.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "On the `<svg>`. The icon is `1em` square until a class or the component around it sizes it.",
            },
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The accessible name. Empty makes the icon decorative (`aria-hidden`); set, it is `role=\"img\"` with that `aria-label`.",
            },
            Prop {
                name: "stroke_width",
                ty: "Option<f64>",
                default: "None",
                description: "Draw the children as outlines of this width (Lucide, Feather, Tabler). Unset they are filled with `currentColor`.",
            },
        ],
    },
    ApiEntry {
        name: "icon slots",
        description: "The prop every component with a glyph takes. A part with one glyph calls it `icon`; a part that swaps glyphs names each after what it shows (`play_icon`, `pause_icon`, `copied_icon`). A component built from other parts passes their slots through: plain props of the same names up to three glyphs (`DatePicker` takes `prev_icon` and `next_icon`), and beyond that an `<Name>Icons` struct such as `VideoPlayerIcons`.",
        props: &[Prop {
            name: "icon",
            ty: "Option<ViewFn>",
            default: "the installed glyph",
            description: "Any view: a closure such as `|| view! { <MyIcon /> }`. Unset draws the component's fallback from your icon directory.",
        }],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Icons"
            description="Every component takes its icons from you, and draws a glyph you own when you pass nothing."
        >
            <div class="flex flex-col gap-8">
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "There is no icon set to depend on. A component that shows a chevron, a check,
                    a close cross or a play button takes it through an "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">"icon"</code>
                    " prop. Pass nothing and it draws its fallback: a Lucide glyph the CLI installed
                    into your project beside the components, which is yours to keep, redraw or
                    replace."
                </p>

                <DemoSection
                    title="Without an icon"
                    description="The fallback, from src/icons/nc."
                    code=WITHOUT
                >
                    <Accordion class="w-full max-w-md">
                        <AccordionItem value="defaults">
                            <AccordionTrigger>"Draws its own chevron"</AccordionTrigger>
                            <AccordionContent>
                                "Nothing passed, so the installed fallback."
                            </AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </DemoSection>

                <DemoSection
                    title="With one"
                    description="The slot is sized, coloured and rotated by the component, so the icon carries no classes of its own."
                    code=WITH
                >
                    <Accordion class="w-full max-w-md">
                        <AccordionItem value="custom">
                            <AccordionTrigger icon=|| {
                                view! { <Caret /> }
                            }>"Draws yours"</AccordionTrigger>
                            <AccordionContent>
                                "Sized, coloured and turned by the trigger."
                            </AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </DemoSection>

                <Separator />

                <DemoSection
                    title="Any artwork"
                    description="Icon takes the icon's own viewBox and fills with currentColor, so a filled icon on a 320-wide canvas looks right. A non-square viewBox is centred in the square the component gives it."
                >
                    <CodeBlock code=CARET language=Language::Rust />
                </DemoSection>

                <DemoSection
                    title="Any children"
                    description="A duotone icon is two shapes and an opacity, and the slot takes it as written. Here it stands in for the spinner's glyph, which the spinner turns."
                    code=DUOTONE
                >
                    <div class="flex items-center gap-4">
                        <Spinner icon=|| view! { <CircleCheckDuotone /> } />
                        <Spinner
                            class="size-6 text-primary"
                            icon=|| view! { <CircleCheckDuotone /> }
                        />
                    </div>
                </DemoSection>

                <DemoSection
                    title="Icon-only, and on its own"
                    description="An icon is aria-hidden unless it has a label. Name a button that holds nothing but an icon with aria_label; give label to an icon that stands alone."
                    code=ICON_ONLY
                >
                    <div class="flex items-center gap-4">
                        <Button
                            size=ButtonSize::Icon
                            variant=ButtonVariant::Outline
                            aria_label="Mark done"
                        >
                            <CircleCheckDuotone />
                        </Button>
                        <CircleCheckDuotone class="size-5 text-primary" label="Done" />
                    </div>
                </DemoSection>

                <Separator />

                <DemoSection
                    title="The fallbacks are installed, not depended on"
                    description="A component brings the glyphs it falls back on, and Icon with them, into paths.icons (src/icons/nc by default). The lock records them, so components status lists the ones you have redrawn, and a component's --force never overwrites one you edited."
                >
                    <CodeBlock code=INSTALLED language=Language::Shell />
                </DemoSection>

                <DemoSection
                    title="Redraw one for every component"
                    description="Change the artwork in the installed file and every component that falls back on it follows. Keep the function's name and its class prop."
                >
                    <CodeBlock code=REDRAW language=Language::Rust />
                </DemoSection>

                <DemoSection
                    title="Bring your own set"
                    description="Generate your icons into your own module as view! functions over Icon, beside nc rather than inside it, and pass them where you want them. Nothing in the library needs to know about them — and an icon set under a licence of its own stays in your repository."
                >
                    <CodeBlock code=OWN_SET language=Language::Rust />
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
