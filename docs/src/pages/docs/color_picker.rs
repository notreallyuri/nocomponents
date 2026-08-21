use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{color_picker::ColorPicker, label::Label},
    utils::color::{ColorFormat, Rgba},
};

const DEFAULT: &str = r##"{
    let value = RwSignal::new("#3b82f6".to_string());

    view! {
        <ColorPicker value=value />
        <p>{move || value.get()}</p>
    }
}"##;

const ALPHA: &str = r##"<ColorPicker
    value=value
    with_alpha=true
    format=ColorFormat::Rgb
    presets=vec!["#ef4444", "#f97316", "#22c55e", "#3b82f6", "#a855f7"]
/>"##;

const OKLCH: &str = r#"// The theme's own tokens are written in oklch, and some of them are outside
// sRGB — reading one back lands on the nearest colour a screen can show.
<ColorPicker value=RwSignal::new("oklch(0.646 0.222 41.116)".to_string()) format=ColorFormat::Oklch />"#;

const PRESETS: &[&str] = &[
    "#ef4444", "#f97316", "#eab308", "#22c55e", "#14b8a6", "#3b82f6", "#a855f7", "#ec4899",
    "#78716c", "#0a0a0a",
];

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "ColorPicker",
        description: "A saturation/brightness square, a hue rail, a text field, and optionally opacity and presets. One component rather than a set of parts; `primitives::color_picker` has them apart.",
        props: &[
            Prop {
                name: "value",
                ty: "RwSignal<String>",
                default: "\"\"",
                description: "The colour as text, in the current format. Read it for a form; write to it to set the picker. Anything the parser reads goes in — hex, `rgb()`, `hsl()`, `oklch()`, old syntax or new.",
            },
            Prop {
                name: "format",
                ty: "ColorFormat",
                default: "Hex",
                description: "Which format the field reads and writes to start with. The button beside it cycles through all four.",
            },
            Prop {
                name: "with_alpha",
                ty: "bool",
                default: "false",
                description: "Adds the opacity rail, and lets the formats carry an alpha.",
            },
            Prop {
                name: "presets",
                ty: "Vec<&'static str>",
                default: "[]",
                description: "A row of colours under the picker. The one matching the current colour is marked.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the picker's classes.",
            },
        ],
    },
    ApiEntry {
        name: "utils::color",
        description: "The arithmetic underneath, usable on its own: `Rgba`, `Hsva`, `Hsla` and `Oklch` with conversions both ways, `Rgba::parse` and `ColorFormat::format`. No dependency — it is written and tested here, like the calendar's date type.",
        props: &[
            Prop {
                name: "Hsva",
                ty: "struct",
                default: "",
                description: "What the picker stores. The square and the rail *are* HSV, and the round trip through 8-bit RGB loses the hue wherever saturation or brightness reaches zero — which is why an RGB-backed picker springs back to red in the black corner.",
            },
            Prop {
                name: "Rgba::parse",
                ty: "fn(&str) -> Option<Rgba>",
                default: "",
                description: "Reads `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`, `rgb()`, `hsl()` and `oklch()`, in modern or comma syntax, with percentages and an alpha after a slash or in a fourth slot. Colour names are not read.",
            },
            Prop {
                name: "ColorFormat::format",
                ty: "fn(Rgba) -> String",
                default: "",
                description: "Writes one back out, mentioning alpha only when there is some.",
            },
            Prop {
                name: "Oklch",
                ty: "struct",
                default: "",
                description: "Can describe colours no screen shows, so converting to `Rgba` clips to the nearest one that fits. Clipping settles rather than drifting: reading a clipped colour back gives the same colour again.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let simple = RwSignal::new("#3b82f6".to_string());
    let translucent = RwSignal::new("rgb(34 197 94 / 0.6)".to_string());
    let token = RwSignal::new("oklch(0.646 0.222 41.116)".to_string());

    view! {
        <DocLayout
            title="Color Picker"
            description="A saturation square, a hue rail and a text field."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Drag in the square, or focus it and use the arrows — shift for larger steps. The picker stores HSV rather than RGB, which is why the hue rail stays where you put it even when you drag into the black corner and back."
                    code=DEFAULT
                >
                    <div class="flex flex-wrap items-start gap-6">
                        <ColorPicker value=simple />
                        <div class="flex flex-col gap-2">
                            <Label>"Value"</Label>
                            <code class="rounded-md bg-muted px-2 py-1 font-mono text-sm">
                                {move || simple.get()}
                            </code>
                            <div
                                class="size-16 rounded-lg border"
                                style=move || format!("background: {}", simple.get())
                            />
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Opacity and presets"
                    description="The opacity rail and the swatch both sit over a chequer, so a translucent colour reads as translucent rather than as the panel behind it. A preset is marked when it is the colour showing."
                    code=ALPHA
                >
                    <div class="flex flex-wrap items-start gap-6">
                        <ColorPicker
                            value=translucent
                            with_alpha=true
                            format=ColorFormat::Rgb
                            presets=PRESETS.to_vec()
                        />
                        <div class="flex flex-col gap-2">
                            <Label>"Value"</Label>
                            <code class="rounded-md bg-muted px-2 py-1 font-mono text-sm">
                                {move || translucent.get()}
                            </code>
                            <p class="max-w-48 text-xs text-muted-foreground">
                                "The format button cycles hex → rgb → hsl → oklch. The field takes any
                                of them, and any of the older comma syntaxes too."
                            </p>
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Out of gamut"
                    description="This is one of the theme's own chart tokens. It is written in oklch, which describes more colours than a screen can show — and this one does not fit, so the picker lands on the nearest that does and says so. Clipping settles rather than drifting: whatever it shows now, it will still show after another trip through the field."
                    code=OKLCH
                >
                    <div class="flex flex-wrap items-start gap-6">
                        <ColorPicker value=token format=ColorFormat::Oklch />
                        <div class="flex flex-col gap-2 text-sm">
                            <Label>"Asked for"</Label>
                            <code class="rounded-md bg-muted px-2 py-1 font-mono text-xs">
                                "oklch(0.646 0.222 41.116)"
                            </code>
                            <Label>"Showing"</Label>
                            <code class="rounded-md bg-muted px-2 py-1 font-mono text-xs">
                                {move || token.get()}
                            </code>
                            <p class="max-w-56 text-xs text-muted-foreground">
                                {move || match Rgba::parse(&token.get()) {
                                    Some(rgba) => {
                                        format!("As bytes: {}", ColorFormat::Rgb.format(rgba))
                                    }
                                    None => "Not a colour.".to_string(),
                                }}
                            </p>
                        </div>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
