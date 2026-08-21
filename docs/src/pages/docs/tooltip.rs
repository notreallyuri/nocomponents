use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    },
    utils::types::Side,
};

const DEFAULT: &str = r#"<Tooltip>
    <TooltipTrigger render=Callback::new(move |node_ref| {
        view! {
            <Button node_ref=node_ref variant=ButtonVariant::Outline>
                "Hover me"
            </Button>
        }
            .into_any()
    }) />
    <TooltipContent>"Tooltips label; they do not explain."</TooltipContent>
</Tooltip>"#;

const SIDES: &str = r#"<Tooltip>
    <TooltipTrigger render=… />
    <TooltipContent side=Side::Right>"On the right"</TooltipContent>
</Tooltip>"#;

const DELAY: &str = r#"<Tooltip delay=0u64>
    <TooltipTrigger render=… />
    <TooltipContent>"No wait at all"</TooltipContent>
</Tooltip>
<Tooltip delay=1000u64>
    <TooltipTrigger render=… />
    <TooltipContent>"A full second of hovering"</TooltipContent>
</Tooltip>"#;

const ON_TEXT: &str = r#"<p class="text-sm">
    "The tokens live in "
    <Tooltip>
        <TooltipTrigger>
            <span class="underline decoration-dotted underline-offset-4">"globals.css"</span>
        </TooltipTrigger>
        <TooltipContent>"docs/styles/globals.css"</TooltipContent>
    </Tooltip> "."
</p>"#;

#[component]
pub fn Page() -> impl IntoView {
    let trigger = |label: &'static str| {
        Callback::new(move |node_ref| {
            view! {
                <Button node_ref=node_ref variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    {label}
                </Button>
            }
            .into_any()
        })
    };

    view! {
        <DocLayout title="Tooltip" description="A short label that appears on hover or focus.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Opens after the pointer rests for a moment, and on focus too — a tooltip only the mouse can reach is no tooltip at all."
                    code=DEFAULT
                >
                    <Tooltip>
                        <TooltipTrigger render=trigger("Hover me") />
                        <TooltipContent>"Tooltips label; they do not explain."</TooltipContent>
                    </Tooltip>
                </DemoSection>

                <DemoSection
                    title="Sides"
                    description="Like the popover, it flips and shifts to stay in the viewport whatever side you ask for."
                    code=SIDES
                >
                    <div class="flex flex-wrap gap-3">
                        <Tooltip>
                            <TooltipTrigger render=trigger("Top") />
                            <TooltipContent side=Side::Top>"On the top"</TooltipContent>
                        </Tooltip>
                        <Tooltip>
                            <TooltipTrigger render=trigger("Right") />
                            <TooltipContent side=Side::Right>"On the right"</TooltipContent>
                        </Tooltip>
                        <Tooltip>
                            <TooltipTrigger render=trigger("Bottom") />
                            <TooltipContent side=Side::Bottom>"On the bottom"</TooltipContent>
                        </Tooltip>
                        <Tooltip>
                            <TooltipTrigger render=trigger("Left") />
                            <TooltipContent side=Side::Left>"On the left"</TooltipContent>
                        </Tooltip>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Delay"
                    description="The default 400ms keeps a sweep across a toolbar from flashing a row of tooltips. Drop it to zero where the tooltip is the only label a control has."
                    code=DELAY
                >
                    <div class="flex flex-wrap gap-3">
                        <Tooltip delay=0u64>
                            <TooltipTrigger render=trigger("Instant") />
                            <TooltipContent>"No wait at all"</TooltipContent>
                        </Tooltip>
                        <Tooltip delay=1000u64>
                            <TooltipTrigger render=trigger("Patient") />
                            <TooltipContent>"A full second of hovering"</TooltipContent>
                        </Tooltip>
                    </div>
                </DemoSection>

                <DemoSection
                    title="On text"
                    description="Without render the trigger wraps its children in a span, for a word in a sentence rather than a control."
                    code=ON_TEXT
                >
                    <p class="text-sm">
                        "The tokens live in " <Tooltip>
                            <TooltipTrigger>
                                <span class="underline decoration-dotted underline-offset-4">
                                    "globals.css"
                                </span>
                            </TooltipTrigger>
                            <TooltipContent>"docs/styles/globals.css"</TooltipContent>
                        </Tooltip> "."
                    </p>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
