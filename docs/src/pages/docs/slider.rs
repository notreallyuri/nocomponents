use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{components::slider::Slider, utils::types::Orientation};

const DEFAULT: &str = r#"<Slider default_value=vec![40.0] />"#;

const CONTROLLED: &str = r#"let volume = RwSignal::new(vec![60.0]);

view! {
    <Slider value=volume />
    <span class="text-sm text-muted-foreground">
        {move || format!("{}%", volume.get()[0])}
    </span>
}"#;

const RANGE: &str = r#"// Two values means two thumbs, and neither can be dragged past the other.
<Slider value=price min=0.0 max=500.0 step=10.0 />"#;

const STEP: &str = r#"<Slider value=rating min=0.0 max=5.0 step=0.5 />"#;

const VERTICAL: &str = r#"<Slider
    orientation=Orientation::Vertical
    default_value=vec![70.0]
/>"#;

const DISABLED: &str = r#"<Slider default_value=vec![30.0] disabled=true />"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Slider",
        description: "A value chosen by dragging along a track. A range is the two-thumb case of the same component.",
        props: &[
            Prop {
                name: "min",
                ty: "Signal<f64>",
                default: "0.0",
                description: "The value at the start of the track.",
            },
            Prop {
                name: "max",
                ty: "Signal<f64>",
                default: "100.0",
                description: "The value at the end of the track.",
            },
            Prop {
                name: "step",
                ty: "Signal<f64>",
                default: "1.0",
                description: "What the value snaps to, rounded at the step's own precision.",
            },
            Prop {
                name: "orientation",
                ty: "Orientation",
                default: "Horizontal",
                description: "One of: Horizontal, Vertical.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Stops it taking pointer events and marks it disabled.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "id",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Given to the first thumb, so a `FieldLabel` can name it. Inside a `Field` this is filled in, along with `aria-describedby` and `aria-invalid` on every thumb.",
            },
            Prop {
                name: "name",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The name the values are submitted under, through one `<input type=\"hidden\">` per thumb: `name` for a single thumb, `name[]` once per thumb, lowest first, for a range. Empty leaves it out.",
            },
            Prop {
                name: "form",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The `id` of a `<form>` it belongs to when it is not inside one. Empty means the one around it.",
            },
            Prop {
                name: "value",
                ty: "Option<RwSignal<Vec<f64>>>",
                default: "None",
                description: "Omit for an uncontrolled slider. The length decides how many thumbs there are.",
            },
            Prop {
                name: "default_value",
                ty: "Option<Vec<f64>>",
                default: "None",
                description: "Where an uncontrolled slider starts. Its length decides how many thumbs there are.",
            },
            Prop {
                name: "on_value_change",
                ty: "Option<Callback<Vec<f64>>>",
                default: "None",
                description: "Called with every thumb's value whenever one moves, including when the signal passed as `value` is written from outside.",
            },
        ],
    },
    ApiEntry {
        name: "SliderTrack",
        description: "The groove the thumbs run along. A press on it moves the nearest thumb and picks it up.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the track's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A range and one thumb per value.",
            },
        ],
    },
    ApiEntry {
        name: "SliderRange",
        description: "The filled stretch: from the start to the only thumb, or between the outer two.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the range's classes.",
        }],
    },
    ApiEntry {
        name: "SliderThumb",
        description: "One handle. Announces its own bounds, which are its neighbours rather than the track's.",
        props: &[
            Prop {
                name: "index",
                ty: "usize",
                default: "0",
                description: "Which value this thumb owns.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the thumb's classes.",
            },
            Prop {
                name: "aria_label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "What the thumb is called. Empty inside a `Field` leaves it to the field's label: a `<label for>` cannot name a `<span role=\"slider\">`, so the thumb points `aria-labelledby` at the label instead.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let volume = RwSignal::new(vec![60.0]);
    let price = RwSignal::new(vec![120.0, 380.0]);
    let rating = RwSignal::new(vec![3.5]);

    view! {
        <DocLayout title="Slider" description="A value chosen by dragging along a track.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Press anywhere on the track and the nearest thumb comes to meet the pointer, then keeps following it — a click and a drag are the same gesture."
                    code=DEFAULT
                >
                    <div class="w-full max-w-sm">
                        <Slider default_value=vec![40.0] />
                    </div>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="The value is a Vec<f64>, so the same signal shape covers one thumb and many."
                    code=CONTROLLED
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <Slider value=volume />
                        <span class="text-sm text-muted-foreground">
                            {move || format!("{}%", volume.get()[0])}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Range"
                    description="Two values means two thumbs. Each is clamped to its neighbour, so they cannot cross however hard they are dragged, and each announces its own bounds rather than the track's."
                    code=RANGE
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <Slider value=price min=0.0 max=500.0 step=10.0 />
                        <span class="text-sm text-muted-foreground">
                            {move || {
                                let p = price.get();
                                format!("${} – ${}", p[0], p[1])
                            }}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Step"
                    description="A fractional step snaps at its own precision, so a half-star stays 3.5 rather than becoming 3.5000000000000004."
                    code=STEP
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <Slider value=rating min=0.0 max=5.0 step=0.5 />
                        <span class="text-sm text-muted-foreground">
                            {move || format!("{} / 5", rating.get()[0])}
                        </span>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Vertical"
                    description="A vertical track runs bottom to top, the way a fader does. Up and Right still raise the value; Home and End jump to the ends."
                    code=VERTICAL
                >
                    <div class="flex h-48 items-center">
                        <Slider orientation=Orientation::Vertical default_value=vec![70.0] />
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled slider takes neither the pointer nor the keyboard, and its thumbs leave the tab order."
                    code=DISABLED
                >
                    <div class="w-full max-w-sm">
                        <Slider default_value=vec![30.0] disabled=true />
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
