use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    button::ButtonVariant,
    input::Input,
    label::Label,
    popover::{
        Popover, PopoverContent, PopoverDescription, PopoverHeader, PopoverPortal, PopoverTitle,
        PopoverTrigger,
    },
};
use nocomponents::utils::types::Side;

const DEFAULT: &str = r#"<Popover>
    <PopoverTrigger variant=ButtonVariant::Outline>
        "Open popover"
    </PopoverTrigger>
    <PopoverPortal>
        <PopoverContent class="w-72">
            <PopoverHeader>
                <PopoverTitle>"Quick info"</PopoverTitle>
                <PopoverDescription>
                    "Popovers are great for contextual details that don't warrant a full dialog."
                </PopoverDescription>
            </PopoverHeader>
        </PopoverContent>
    </PopoverPortal>
</Popover>"#;

const WITH_FORM: &str = r#"<Popover>
    <PopoverTrigger variant=ButtonVariant::Secondary>
        "Adjust dimensions"
    </PopoverTrigger>
    <PopoverPortal>
        <PopoverContent class="w-72">
            <PopoverHeader>
                <PopoverTitle>"Dimensions"</PopoverTitle>
            </PopoverHeader>
            <div class="px-4 pb-2 flex flex-col gap-3">
                <div class="flex items-center justify-between gap-4">
                    <Label class="text-muted-foreground">"Width"</Label>
                    <Input class="w-36 h-7" attr:placeholder="100%" />
                </div>
                <div class="flex items-center justify-between gap-4">
                    <Label class="text-muted-foreground">"Height"</Label>
                    <Input class="w-36 h-7" attr:placeholder="auto" />
                </div>
            </div>
        </PopoverContent>
    </PopoverPortal>
</Popover>"#;

const PLACEMENT: &str = r#"<Popover>
    <PopoverTrigger variant=ButtonVariant::Outline>"Top"</PopoverTrigger>
    <PopoverPortal>
        <PopoverContent side=Side::Top class="w-48">
            <PopoverHeader>
                <PopoverTitle>"Placed on top"</PopoverTitle>
            </PopoverHeader>
        </PopoverContent>
    </PopoverPortal>
</Popover>
<Popover>
    <PopoverTrigger variant=ButtonVariant::Outline>"Right"</PopoverTrigger>
    <PopoverPortal>
        <PopoverContent side=Side::Right class="w-48">
            <PopoverHeader>
                <PopoverTitle>"Placed on right"</PopoverTitle>
            </PopoverHeader>
        </PopoverContent>
    </PopoverPortal>
</Popover>
<Popover>
    <PopoverTrigger variant=ButtonVariant::Outline>"Bottom"</PopoverTrigger>
    <PopoverPortal>
        <PopoverContent side=Side::Bottom class="w-48">
            <PopoverHeader>
                <PopoverTitle>"Placed on bottom"</PopoverTitle>
            </PopoverHeader>
        </PopoverContent>
    </PopoverPortal>
</Popover>
<Popover>
    <PopoverTrigger variant=ButtonVariant::Outline>"Left"</PopoverTrigger>
    <PopoverPortal>
        <PopoverContent side=Side::Left class="w-48">
            <PopoverHeader>
                <PopoverTitle>"Placed on left"</PopoverTitle>
            </PopoverHeader>
        </PopoverContent>
    </PopoverPortal>
</Popover>"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Popover" description="Floating content anchored to a trigger element.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The trigger anchors the content, which flips and shifts to stay in the viewport."
                    code=DEFAULT
                >
                    <Popover>
                        <PopoverTrigger variant=ButtonVariant::Outline>
                            "Open popover"
                        </PopoverTrigger>
                        <PopoverPortal>
                            <PopoverContent class="w-72">
                                <PopoverHeader>
                                    <PopoverTitle>"Quick info"</PopoverTitle>
                                    <PopoverDescription>
                                        "Popovers are great for contextual details that don't warrant a full dialog."
                                    </PopoverDescription>
                                </PopoverHeader>
                            </PopoverContent>
                        </PopoverPortal>
                    </Popover>
                </DemoSection>

                <DemoSection
                    title="With form"
                    description="Popovers can hold interactive content, not just text."
                    code=WITH_FORM
                >
                    <Popover>
                        <PopoverTrigger variant=ButtonVariant::Secondary>
                            "Adjust dimensions"
                        </PopoverTrigger>
                        <PopoverPortal>
                            <PopoverContent class="w-72">
                                <PopoverHeader>
                                    <PopoverTitle>"Dimensions"</PopoverTitle>
                                </PopoverHeader>
                                <div class="px-4 pb-2 flex flex-col gap-3">
                                    <div class="flex items-center justify-between gap-4">
                                        <Label class="text-muted-foreground">"Width"</Label>
                                        <Input class="w-36 h-7" attr:placeholder="100%" />
                                    </div>
                                    <div class="flex items-center justify-between gap-4">
                                        <Label class="text-muted-foreground">"Height"</Label>
                                        <Input class="w-36 h-7" attr:placeholder="auto" />
                                    </div>
                                </div>
                            </PopoverContent>
                        </PopoverPortal>
                    </Popover>
                </DemoSection>

                <DemoSection
                    title="Placement"
                    description="Side and alignment can be configured independently."
                    code=PLACEMENT
                >
                    <div class="flex flex-wrap gap-3">
                        <Popover>
                            <PopoverTrigger variant=ButtonVariant::Outline>"Top"</PopoverTrigger>
                            <PopoverPortal>
                                <PopoverContent side=Side::Top class="w-48">
                                    <PopoverHeader>
                                        <PopoverTitle>"Placed on top"</PopoverTitle>
                                    </PopoverHeader>
                                </PopoverContent>
                            </PopoverPortal>
                        </Popover>
                        <Popover>
                            <PopoverTrigger variant=ButtonVariant::Outline>"Right"</PopoverTrigger>
                            <PopoverPortal>
                                <PopoverContent side=Side::Right class="w-48">
                                    <PopoverHeader>
                                        <PopoverTitle>"Placed on right"</PopoverTitle>
                                    </PopoverHeader>
                                </PopoverContent>
                            </PopoverPortal>
                        </Popover>
                        <Popover>
                            <PopoverTrigger variant=ButtonVariant::Outline>"Bottom"</PopoverTrigger>
                            <PopoverPortal>
                                <PopoverContent side=Side::Bottom class="w-48">
                                    <PopoverHeader>
                                        <PopoverTitle>"Placed on bottom"</PopoverTitle>
                                    </PopoverHeader>
                                </PopoverContent>
                            </PopoverPortal>
                        </Popover>
                        <Popover>
                            <PopoverTrigger variant=ButtonVariant::Outline>"Left"</PopoverTrigger>
                            <PopoverPortal>
                                <PopoverContent side=Side::Left class="w-48">
                                    <PopoverHeader>
                                        <PopoverTitle>"Placed on left"</PopoverTitle>
                                    </PopoverHeader>
                                </PopoverContent>
                            </PopoverPortal>
                        </Popover>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
