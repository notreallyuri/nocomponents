use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
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

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Popover",
        description: "A panel anchored to what opened it, for content that needs room a tooltip does not have.",
        props: &[Prop {
            name: "children",
            ty: "ChildrenFn",
            default: "",
            description: "A trigger and a portal.",
        }],
    },
    ApiEntry {
        name: "PopoverTrigger",
        description: "Opens it. A `Button` unless `as_child` says otherwise.",
        props: &[
            Prop {
                name: "size",
                ty: "ButtonSize",
                default: "Default",
                description: "One of: Xs, Sm, Default, Lg, Icon, IconSm, IconXs, IconLg.",
            },
            Prop {
                name: "variant",
                ty: "ButtonVariant",
                default: "Default",
                description: "One of: Default, Outline, Ghost, Secondary, Link, Destructive.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's own classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Rendered as the `disabled` attribute.",
            },
            Prop {
                name: "as_child",
                ty: "Option<Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>>",
                default: "None",
                description: "Render the trigger as something else. Handed the node ref and the click handler, both of which must reach the element.",
            },
            Prop {
                name: "children",
                ty: "Option<ChildrenFn>",
                default: "None",
                description: "The label.",
            },
        ],
    },
    ApiEntry {
        name: "PopoverPortal",
        description: "The panel, positioned against the trigger and flipped when there is no room.",
        props: &[Prop {
            name: "children",
            ty: "ChildrenFn",
            default: "",
            description: "Distance from the trigger, in pixels.",
        }],
    },
    ApiEntry {
        name: "PopoverContent",
        description: "Merged over the panel's classes.",
        props: &[
            Prop {
                name: "side",
                ty: "Side",
                default: "Bottom",
                description: "One of: Left, Right, Bottom, Top.",
            },
            Prop {
                name: "align",
                ty: "Align",
                default: "Start",
                description: "One of: Start, Center, End.",
            },
            Prop {
                name: "side_offset",
                ty: "SideOffset",
                default: "SideOffset(4.0)",
                description: "The panel's contents.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Stacks the title over the description.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Merged over the layout classes.",
            },
        ],
    },
    ApiEntry {
        name: "PopoverHeader",
        description: "Usually a title and a description.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The panel's name.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Merged over the heading classes.",
            },
        ],
    },
    ApiEntry {
        name: "PopoverTitle",
        description: "The title text.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The line under the title.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Merged over the muted text classes.",
            },
        ],
    },
    ApiEntry {
        name: "PopoverDescription",
        description: "The description text.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The bottom strip, for actions.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Merged over the layout classes.",
            },
        ],
    },
    ApiEntry {
        name: "PopoverFooter",
        description: "Usually buttons.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Renders the panel at the end of the document, so it escapes any clipping ancestor.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The panel. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
];

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

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
