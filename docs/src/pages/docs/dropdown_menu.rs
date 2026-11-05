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
        DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuContent, DropdownMenuGroup,
        DropdownMenuItem, DropdownMenuLabel, DropdownMenuPortal, DropdownMenuRadioGroup,
        DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuSub,
        DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger,
    },
};
use nocomponents::primitives::dropdown_menu::DropdownMenuItemVariant;

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
            <DropdownMenuItem disabled=true>"Billing"</DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem variant=DropdownMenuItemVariant::Destructive>
                "Log out"
            </DropdownMenuItem>
        </DropdownMenuContent>
    </DropdownMenuPortal>
</DropdownMenu>"#;

const CHOICES: &str = r#"let show_status = RwSignal::new(true);
let position = RwSignal::new("bottom".to_string());

view! {
    <DropdownMenu>
        <DropdownMenuTrigger variant=ButtonVariant::Outline>
            "View"
        </DropdownMenuTrigger>
        <DropdownMenuPortal>
            <DropdownMenuContent class="w-56">
                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        "New tab"
                        <DropdownMenuShortcut>"Ctrl+T"</DropdownMenuShortcut>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
                <DropdownMenuSeparator />
                <DropdownMenuLabel>"Appearance"</DropdownMenuLabel>
                <DropdownMenuCheckboxItem checked=show_status>
                    "Status bar"
                </DropdownMenuCheckboxItem>
                <DropdownMenuSeparator />
                <DropdownMenuLabel>"Panel position"</DropdownMenuLabel>
                <DropdownMenuRadioGroup value=position>
                    <DropdownMenuRadioItem value="top">"Top"</DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value="bottom">"Bottom"</DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value="right">"Right"</DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>
                <DropdownMenuSeparator />
                <DropdownMenuItem variant=DropdownMenuItemVariant::Destructive>
                    "Reset layout"
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenuPortal>
    </DropdownMenu>
}"#;

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
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Pass one to drive the menu from outside or to watch it; omit it and the root owns the signal.",
            },
            Prop {
                name: "default_open",
                ty: "bool",
                default: "false",
                description: "Whether the menu starts open. Ignored when `open` is given.",
            },
            Prop {
                name: "on_open_change",
                ty: "Option<Callback<bool>>",
                default: "None",
                description: "Called with the new value whenever the menu opens or closes, including when the signal passed as `open` is written from outside.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "A trigger and a portal.",
            },
        ],
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
        description: "One row. Selecting it closes the whole menu tree, not just the panel it is in, unless `close_on_select` is `false`.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Refuses the click and leaves the menu open. The arrow keys and typeahead skip it.",
            },
            Prop {
                name: "inset",
                ty: "Signal<bool>",
                default: "false",
                description: "Writes `data-inset`, which indents the row to line up with items that carry an indicator.",
            },
            Prop {
                name: "close_on_select",
                ty: "Signal<bool>",
                default: "true",
                description: "Whether choosing the item closes the menu. Set it to `false` for an action that should leave the menu open. Read at selection time.",
            },
            Prop {
                name: "variant",
                ty: "DropdownMenuItemVariant",
                default: "Default",
                description: "One of: Default, Destructive. Written as `data-variant`.",
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
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the part's classes.",
            },
            Prop {
                name: "inset",
                ty: "Signal<bool>",
                default: "false",
                description: "Writes `data-inset`.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The heading text.",
            },
        ],
    },
    ApiEntry {
        name: "DropdownMenuSeparator",
        description: "A rule between groups of items.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the part's classes.",
        }],
    },
    ApiEntry {
        name: "DropdownMenuGroup",
        description: "Items that belong together, rendered as `role=\"group\"`.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the part's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The group's items.",
            },
        ],
    },
    ApiEntry {
        name: "DropdownMenuShortcut",
        description: "A hint such as a key chord, pushed to the end of its row. Style only.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the part's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The hint text.",
            },
        ],
    },
    ApiEntry {
        name: "DropdownMenuCheckboxItem",
        description: "A row that toggles. `role=\"menuitemcheckbox\"`, with `aria-checked` and `data-state` of `checked` or `unchecked`. Walked by the arrows and typeahead like an item, but leaves the menu open unless `close_on_select` is set.",
        props: &[
            Prop {
                name: "checked",
                ty: "RwSignal<bool>",
                default: "false",
                description: "The state. Toggled on selection.",
            },
            Prop {
                name: "on_checked_change",
                ty: "Option<Callback<bool>>",
                default: "None",
                description: "Called with the new state.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Refuses the click and is skipped by the keys.",
            },
            Prop {
                name: "inset",
                ty: "Signal<bool>",
                default: "false",
                description: "Writes `data-inset`.",
            },
            Prop {
                name: "close_on_select",
                ty: "Signal<bool>",
                default: "false",
                description: "Whether toggling closes the menu. Off by default, so a menu of toggles stays open and focus stays on the row. Read at selection time.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the part's classes.",
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
        name: "DropdownMenuRadioGroup",
        description: "Radio items that share one value.",
        props: &[
            Prop {
                name: "value",
                ty: "RwSignal<String>",
                default: "\"\"",
                description: "The value of the selected item.",
            },
            Prop {
                name: "on_value_change",
                ty: "Option<Callback<String>>",
                default: "None",
                description: "Called with the chosen item's value.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the part's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The radio items.",
            },
        ],
    },
    ApiEntry {
        name: "DropdownMenuRadioItem",
        description: "One choice of a radio group. `role=\"menuitemradio\"`, with `aria-checked` and `data-state`. Leaves the menu open unless `close_on_select` is set.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "What the group's value becomes when this row is chosen.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Refuses the click and is skipped by the keys.",
            },
            Prop {
                name: "inset",
                ty: "Signal<bool>",
                default: "false",
                description: "Writes `data-inset`.",
            },
            Prop {
                name: "close_on_select",
                ty: "Signal<bool>",
                default: "false",
                description: "Whether choosing the row closes the menu. Off by default, so focus stays on the row. Read at selection time.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the part's classes.",
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
        name: "DropdownMenuSub",
        description: "A submenu: its own trigger and content, nested in a parent menu.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the part's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "A trigger and a sub-content.",
            },
        ],
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
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Rendered as the `disabled` attribute.",
            },
            Prop {
                name: "inset",
                ty: "Signal<bool>",
                default: "false",
                description: "Writes `data-inset`.",
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
    let show_status = RwSignal::new(true);
    let position = RwSignal::new("bottom".to_string());

    view! {
        <DocLayout
            title="Dropdown Menu"
            description="Displays a list of actions triggered by a button."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Labels, separators and items compose the menu; the content is portalled and positioned against the trigger. A disabled item is skipped by the arrows and the typeahead as well as by the pointer."
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
                                <DropdownMenuItem disabled=true>"Billing"</DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem variant=DropdownMenuItemVariant::Destructive>
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

                <DemoSection
                    title="Checkboxes and radios"
                    description="Checkbox and radio items keep their state in signals you own. Choosing one keeps the menu open, with focus on the row; `close_on_select` closes it instead."
                    code=CHOICES
                >
                    <DropdownMenu>
                        <DropdownMenuTrigger variant=ButtonVariant::Outline>
                            "View"
                        </DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent class="w-56">
                                <DropdownMenuGroup>
                                    <DropdownMenuItem>
                                        "New tab"
                                        <DropdownMenuShortcut>"Ctrl+T"</DropdownMenuShortcut>
                                    </DropdownMenuItem>
                                </DropdownMenuGroup>
                                <DropdownMenuSeparator />
                                <DropdownMenuLabel>"Appearance"</DropdownMenuLabel>
                                <DropdownMenuCheckboxItem checked=show_status>
                                    "Status bar"
                                </DropdownMenuCheckboxItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuLabel>"Panel position"</DropdownMenuLabel>
                                <DropdownMenuRadioGroup value=position>
                                    <DropdownMenuRadioItem value="top">"Top"</DropdownMenuRadioItem>
                                    <DropdownMenuRadioItem value="bottom">
                                        "Bottom"
                                    </DropdownMenuRadioItem>
                                    <DropdownMenuRadioItem value="right">
                                        "Right"
                                    </DropdownMenuRadioItem>
                                </DropdownMenuRadioGroup>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem variant=DropdownMenuItemVariant::Destructive>
                                    "Reset layout"
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
