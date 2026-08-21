use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
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
            </div>
        </DocLayout>
    }
}
