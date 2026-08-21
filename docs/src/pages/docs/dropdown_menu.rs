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
    dropdown_menu::{
        DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuPortal,
        DropdownMenuSeparator, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger,
        DropdownMenuTrigger,
    },
};

const DEFAULT: &str = r#"<DropdownMenu>
    <DropdownMenuTrigger variant=ButtonVariant::Outline>
        "Options"
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
        <DropdownMenuContent class="w-48">
            <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem>"Profile"</DropdownMenuItem>
            <DropdownMenuItem>"Settings"</DropdownMenuItem>
            <DropdownMenuItem>"Billing"</DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem class="text-destructive hover:bg-destructive/10 hover:text-destructive">
                "Log out"
            </DropdownMenuItem>
        </DropdownMenuContent>
    </DropdownMenuPortal>
</DropdownMenu>"#;

const WITH_SUBMENU: &str = r#"<DropdownMenu>
    <DropdownMenuTrigger variant=ButtonVariant::Outline>
        "More"
    </DropdownMenuTrigger>
    <DropdownMenuPortal>
        <DropdownMenuContent class="w-48">
            <DropdownMenuItem>"Dashboard"</DropdownMenuItem>
            <DropdownMenuSub>
                <DropdownMenuSubTrigger>"Team"</DropdownMenuSubTrigger>
                <DropdownMenuSubContent>
                    <DropdownMenuItem>"Invite members"</DropdownMenuItem>
                    <DropdownMenuItem>"Manage roles"</DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem>"Audit log"</DropdownMenuItem>
                </DropdownMenuSubContent>
            </DropdownMenuSub>
            <DropdownMenuSeparator />
            <DropdownMenuItem>"Support"</DropdownMenuItem>
        </DropdownMenuContent>
    </DropdownMenuPortal>
</DropdownMenu>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "DropdownMenu",
        description: "A menu opened by a button. Arrows walk it, typeahead jumps, Escape closes one layer.",
        props: &[Prop {
            name: "children",
            ty: "ChildrenFn",
            default: "",
            description: "A trigger and a portal.",
        }],
    },
    ApiEntry {
        name: "DropdownMenuTrigger",
        description: "Opens it. A `Button` unless `render` says otherwise.",
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
                name: "render",
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
        name: "DropdownMenuPortal",
        description: "Renders the menu at the end of the document, so it escapes any clipping ancestor.",
        props: &[Prop {
            name: "children",
            ty: "ChildrenFn",
            default: "",
            description: "The menu. Re-run on each mount, hence `ChildrenFn`.",
        }],
    },
    ApiEntry {
        name: "DropdownMenuContent",
        description: "The menu surface, positioned against the trigger and flipped when there is no room.",
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
                description: "Distance from the trigger, in pixels.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the menu's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items, labels and separators.",
            },
        ],
    },
    ApiEntry {
        name: "DropdownMenuItem",
        description: "One row. Selecting it closes the whole menu tree, not just the panel it is in.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "on_click",
                ty: "Option<Callback<ev::MouseEvent>>",
                default: "None",
                description: "Called before the menu closes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The row's label.",
            },
        ],
    },
    ApiEntry {
        name: "DropdownMenuLabel",
        description: "A heading over a group of items. Not selectable.",
        props: &[Prop {
            name: "children",
            ty: "Children",
            default: "",
            description: "The heading text.",
        }],
    },
    ApiEntry {
        name: "DropdownMenuSeparator",
        description: "A rule between groups of items.",
        props: &[],
    },
    ApiEntry {
        name: "DropdownMenuSub",
        description: "A submenu: its own trigger and content, nested in a parent menu.",
        props: &[Prop {
            name: "children",
            ty: "ChildrenFn",
            default: "",
            description: "A trigger and a sub-content.",
        }],
    },
    ApiEntry {
        name: "DropdownMenuSubTrigger",
        description: "The row that opens a submenu. Right steps into it, Left steps back out.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the row's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The row's label.",
            },
        ],
    },
    ApiEntry {
        name: "DropdownMenuSubContent",
        description: "The submenu surface, opened to the side of its trigger.",
        props: &[
            Prop {
                name: "side_offset",
                ty: "SideOffset",
                default: "SideOffset(4.0)",
                description: "Distance from the trigger, in pixels.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the menu's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The submenu's items. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Dropdown Menu"
            description="Displays a list of actions triggered by a button."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Labels, separators and items compose the menu; the content is portalled and positioned against the trigger."
                    code=DEFAULT
                >
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline>
                            "Options"
                        </DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent class="w-48">
                                <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem>"Profile"</DropdownMenuItem>
                                <DropdownMenuItem>"Settings"</DropdownMenuItem>
                                <DropdownMenuItem>"Billing"</DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem class="text-destructive hover:bg-destructive/10 hover:text-destructive">
                                    "Log out"
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
                </DemoSection>

                <DemoSection
                    title="With submenu"
                    description="DropdownMenuSub nests a second menu that opens beside its trigger."
                    code=WITH_SUBMENU
                >
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline>
                            "More"
                        </DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent class="w-48">
                                <DropdownMenuItem>"Dashboard"</DropdownMenuItem>
                                <DropdownMenuSub>
                                    <DropdownMenuSubTrigger>"Team"</DropdownMenuSubTrigger>
                                    <DropdownMenuSubContent>
                                        <DropdownMenuItem>"Invite members"</DropdownMenuItem>
                                        <DropdownMenuItem>"Manage roles"</DropdownMenuItem>
                                        <DropdownMenuSeparator />
                                        <DropdownMenuItem>"Audit log"</DropdownMenuItem>
                                    </DropdownMenuSubContent>
                                </DropdownMenuSub>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem>"Support"</DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
