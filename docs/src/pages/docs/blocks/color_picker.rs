//! Colour picker blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        button::ButtonVariant,
        color_picker::ColorPicker,
        popover::{Popover, PopoverContent, PopoverPortal, PopoverTrigger},
    },
    utils::types::Align,
};

const SWATCH: &str = r##"let value = RwSignal::new("#3b82f6".to_string());

view! {
    <Popover>
        <PopoverTrigger variant=ButtonVariant::Outline class="gap-2 font-mono text-xs">
            <span
                class="size-4 rounded-sm border"
                style=move || format!("background: {}", value.get())
            />
            {move || value.get()}
        </PopoverTrigger>
        <PopoverPortal>
            // The picker brings its own popover shell — a border, a background and padding —
            // which is one shell too many once it is inside a real one.
            <PopoverContent align=Align::Start class="w-64 p-3">
                <ColorPicker
                    value=value
                    presets=SWATCHES.to_vec()
                    class="w-full border-0 bg-transparent p-0"
                />
            </PopoverContent>
        </PopoverPortal>
    </Popover>
}"##;

const SWATCHES: &[&str] = &[
    "#ef4444", "#f97316", "#eab308", "#22c55e", "#14b8a6", "#3b82f6", "#a855f7", "#ec4899",
];

pub const BLOCKS: &[Block] = &[Block {
    title: "A swatch that opens a picker",
    description: "The trigger is the colour: a swatch and the value it currently holds, which is \
                  what a colour field looks like everywhere else. The seam is the drag — press \
                  inside the saturation square, keep going past the edge of the popover and let go \
                  out on the page. The popover stays, because dismissal is decided on the \
                  *pointerdown* that opened the gesture, and that one landed inside.",
    code: SWATCH,
    view: || view! { <SwatchPopover /> }.into_any(),
}];

#[component]
fn SwatchPopover() -> impl IntoView {
    let value = RwSignal::new("#3b82f6".to_string());

    view! {
        <div class="flex flex-wrap items-center gap-4">
            <Popover>
                <PopoverTrigger variant=ButtonVariant::Outline class="gap-2 font-mono text-xs">
                    <span
                        class="size-4 rounded-sm border"
                        style=move || format!("background: {}", value.get())
                    />
                    {move || value.get()}
                </PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent align=Align::Start class="w-64 p-3">
                        <ColorPicker
                            value=value
                            presets=SWATCHES.to_vec()
                            class="w-full border-0 bg-transparent p-0"
                        />
                    </PopoverContent>
                </PopoverPortal>
            </Popover>

            <div
                class="h-10 min-w-32 flex-1 rounded-lg border"
                style=move || format!("background: {}", value.get())
            />
        </div>
    }
}
