use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarLabel, MenubarMenu, MenubarPortal,
    MenubarSeparator, MenubarShortcut, MenubarSub, MenubarSubContent, MenubarSubTrigger,
    MenubarTrigger,
};

const DEFAULT: &str = r#"<Menubar>
    <MenubarMenu value="file">
        <MenubarTrigger>"File"</MenubarTrigger>
        <MenubarPortal>
            <MenubarContent>
                <MenubarItem>"New tab" <MenubarShortcut>"⌘T"</MenubarShortcut></MenubarItem>
                <MenubarItem>"New window" <MenubarShortcut>"⌘N"</MenubarShortcut></MenubarItem>
                <MenubarSeparator />
                <MenubarSub>
                    <MenubarSubTrigger>"Share"</MenubarSubTrigger>
                    <MenubarSubContent>
                        <MenubarItem>"Email link"</MenubarItem>
                        <MenubarItem>"Messages"</MenubarItem>
                    </MenubarSubContent>
                </MenubarSub>
                <MenubarSeparator />
                <MenubarItem>"Print…" <MenubarShortcut>"⌘P"</MenubarShortcut></MenubarItem>
            </MenubarContent>
        </MenubarPortal>
    </MenubarMenu>
    <MenubarMenu value="edit">
        <MenubarTrigger>"Edit"</MenubarTrigger>
        <MenubarPortal>
            <MenubarContent>
                <MenubarItem>"Undo" <MenubarShortcut>"⌘Z"</MenubarShortcut></MenubarItem>
                <MenubarItem>"Redo" <MenubarShortcut>"⇧⌘Z"</MenubarShortcut></MenubarItem>
            </MenubarContent>
        </MenubarPortal>
    </MenubarMenu>
</Menubar>"#;

const CONTROLLED: &str = r#"{
    // One signal holds the whole bar: the value of the menu that is open, or `None`.
    let open = RwSignal::new(None::<String>);

    view! {
        <Menubar open=open>…</Menubar>
        <Button on:click=move |_| open.set(Some("view".to_string()))>"Open View"</Button>
        <p>{move || open.get().unwrap_or_else(|| "closed".to_string())}</p>
    }
}"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Menubar",
        description: "The bar. One tab stop, arrows along it, and one menu open at a time.",
        props: &[
            Prop {
                name: "open",
                ty: "RwSignal<Option<String>>",
                default: "None",
                description: "The value of the menu that is open. Pass one to drive the bar or to watch it; this is the whole of its state.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the bar's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The menus.",
            },
        ],
    },
    ApiEntry {
        name: "MenubarMenu",
        description: "One menu of the bar. Provides the floating context the parts below it read, and follows the bar's value in both directions.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "What this menu is called in the bar's `open` signal. Unique within the bar.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the menu wrapper's classes.",
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
        name: "MenubarTrigger",
        description: "The name on the bar. A `role=menuitem` button: clicking toggles its menu, and once any menu is open, moving the pointer onto another trigger switches to it.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the trigger's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Rendered as the `disabled` attribute, and skipped by the arrows.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The menu's name.",
            },
        ],
    },
    ApiEntry {
        name: "MenubarPortal",
        description: "Renders the menu at the end of the document, mounted only while it is open — plus 150ms for the exit animation.",
        props: &[Prop {
            name: "children",
            ty: "ChildrenFn",
            default: "",
            description: "The content. Re-run on each mount, hence `ChildrenFn`.",
        }],
    },
    ApiEntry {
        name: "MenubarContent",
        description: "The menu surface: the dropdown menu's, with Left and Right rebound to walk the bar.",
        props: &[
            Prop {
                name: "side",
                ty: "Side",
                default: "Side::Bottom",
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
                description: "Merged over the surface's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items, labels, separators and submenus.",
            },
        ],
    },
    ApiEntry {
        name: "MenubarItem",
        description: "One command. Closes the whole bar when chosen, not just the panel it is in.",
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
                name: "on_click",
                ty: "Option<Callback<MouseEvent>>",
                default: "None",
                description: "Run before the bar closes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The item's contents.",
            },
        ],
    },
    ApiEntry {
        name: "MenubarLabel",
        description: "A heading over a run of items.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the label's classes.",
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
        name: "MenubarSeparator",
        description: "A rule between runs of items.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rule's classes.",
        }],
    },
    ApiEntry {
        name: "MenubarShortcut",
        description: "The keyboard hint on the right of an item. Presentational — it binds nothing.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the hint's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The keys, written out.",
            },
        ],
    },
    ApiEntry {
        name: "MenubarSub",
        description: "A nested menu. Its own floating layer, which is why choosing an item has to close the bar as well as the panel.",
        props: &[Prop {
            name: "children",
            ty: "ChildrenFn",
            default: "",
            description: "A sub-trigger and a sub-content.",
        }],
    },
    ApiEntry {
        name: "MenubarSubTrigger",
        description: "The item that opens a submenu. Opens on hover, on Right, and on Enter or Space.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the trigger's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The label; the chevron is added.",
            },
        ],
    },
    ApiEntry {
        name: "MenubarSubContent",
        description: "The nested surface. Portalled like the parent, and dismissed by Left as well as Escape.",
        props: &[
            Prop {
                name: "side_offset",
                ty: "SideOffset",
                default: "SideOffset(4.0)",
                description: "Distance from the sub-trigger, in pixels.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the surface's classes.",
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
    let open = RwSignal::new(None::<String>);

    view! {
        <DocLayout
            title="Menubar"
            description="A row of menus that behaves as one control, the way an application's menu bar does."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Each menu is a dropdown menu; the bar is what makes them one control. Open one and the bar is armed: the pointer switches menus as it crosses the triggers, and Left and Right do the same from inside an open menu. The bar is a single tab stop."
                    code=DEFAULT
                >
                    <Menubar open=open>
                        <MenubarMenu value="file">
                            <MenubarTrigger>"File"</MenubarTrigger>
                            <MenubarPortal>
                                <MenubarContent>
                                    <MenubarItem>
                                        "New tab" <MenubarShortcut>"⌘T"</MenubarShortcut>
                                    </MenubarItem>
                                    <MenubarItem>
                                        "New window" <MenubarShortcut>"⌘N"</MenubarShortcut>
                                    </MenubarItem>
                                    <MenubarSeparator />
                                    <MenubarSub>
                                        <MenubarSubTrigger>"Share"</MenubarSubTrigger>
                                        <MenubarSubContent>
                                            <MenubarItem>"Email link"</MenubarItem>
                                            <MenubarItem>"Messages"</MenubarItem>
                                            <MenubarItem>"Notes"</MenubarItem>
                                        </MenubarSubContent>
                                    </MenubarSub>
                                    <MenubarSeparator />
                                    <MenubarItem>
                                        "Print…" <MenubarShortcut>"⌘P"</MenubarShortcut>
                                    </MenubarItem>
                                </MenubarContent>
                            </MenubarPortal>
                        </MenubarMenu>

                        <MenubarMenu value="edit">
                            <MenubarTrigger>"Edit"</MenubarTrigger>
                            <MenubarPortal>
                                <MenubarContent>
                                    <MenubarItem>
                                        "Undo" <MenubarShortcut>"⌘Z"</MenubarShortcut>
                                    </MenubarItem>
                                    <MenubarItem>
                                        "Redo" <MenubarShortcut>"⇧⌘Z"</MenubarShortcut>
                                    </MenubarItem>
                                    <MenubarSeparator />
                                    <MenubarLabel>"Find"</MenubarLabel>
                                    <MenubarItem>"Search the web"</MenubarItem>
                                    <MenubarItem>"Find…"</MenubarItem>
                                </MenubarContent>
                            </MenubarPortal>
                        </MenubarMenu>

                        <MenubarMenu value="view">
                            <MenubarTrigger>"View"</MenubarTrigger>
                            <MenubarPortal>
                                <MenubarContent>
                                    <MenubarItem>"Always show bookmarks"</MenubarItem>
                                    <MenubarItem>"Always show full URLs"</MenubarItem>
                                    <MenubarSeparator />
                                    <MenubarItem>
                                        "Reload" <MenubarShortcut>"⌘R"</MenubarShortcut>
                                    </MenubarItem>
                                    <MenubarItem>"Toggle fullscreen"</MenubarItem>
                                </MenubarContent>
                            </MenubarPortal>
                        </MenubarMenu>

                        <MenubarMenu value="profiles">
                            <MenubarTrigger disabled=true>"Profiles"</MenubarTrigger>
                            <MenubarPortal>
                                <MenubarContent>
                                    <MenubarItem>"Add profile"</MenubarItem>
                                </MenubarContent>
                            </MenubarPortal>
                        </MenubarMenu>
                    </Menubar>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="The bar's whole state is one signal: the value of the menu that is open, or nothing. Reading it is how you know what is showing; writing it is how you open a menu from elsewhere."
                    code=CONTROLLED
                >
                    <p class="text-sm text-muted-foreground">
                        "The bar above is open on: "
                        <span class="font-mono text-foreground">
                            {move || open.get().unwrap_or_else(|| "nothing".to_string())}
                        </span>
                    </p>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
