use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuIndicator, NavigationMenuItem,
    NavigationMenuLink, NavigationMenuList, NavigationMenuTrigger,
};

const DEFAULT: &str = r#"<NavigationMenu>
    <NavigationMenuList>
        <NavigationMenuItem value="components">
            <NavigationMenuTrigger>"Components"</NavigationMenuTrigger>
            <NavigationMenuContent>
                <ul class="grid w-[26rem] grid-cols-2 gap-1">
                    <li>
                        <NavigationMenuLink href="/docs/dialog">
                            <span class="font-medium">"Dialog"</span>
                            <p>"A modal that interrupts with a choice."</p>
                        </NavigationMenuLink>
                    </li>
                    …
                </ul>
            </NavigationMenuContent>
        </NavigationMenuItem>

        <NavigationMenuItem value="docs">
            <NavigationMenuLink href="/docs" class="h-9 px-3 py-2">"Docs"</NavigationMenuLink>
        </NavigationMenuItem>
    </NavigationMenuList>
    <NavigationMenuIndicator />
</NavigationMenu>"#;

const CONTROLLED: &str = r#"{
    let value = RwSignal::new(None::<String>);

    view! {
        <NavigationMenu value=value>…</NavigationMenu>
        <p>{move || value.get().unwrap_or_else(|| "closed".to_string())}</p>
    }
}"#;

const LINKS: &[(&str, &str, &str)] = &[
    (
        "Dialog",
        "/docs/dialog",
        "A modal that interrupts with a choice.",
    ),
    (
        "Popover",
        "/docs/popover",
        "A layer anchored to what opened it.",
    ),
    (
        "Command",
        "/docs/command",
        "A list filtered by what you type.",
    ),
    (
        "Combobox",
        "/docs/combobox",
        "A button that comes back with a value.",
    ),
];

const RESOURCES: &[(&str, &str, &str)] = &[
    (
        "Primitives",
        "/docs",
        "Behaviour, ARIA and data attributes, unstyled.",
    ),
    ("Components", "/docs", "The shadcn-styled layer over them."),
];

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "NavigationMenu",
        description: "The menu. Owns which panel is open and the delays that make it usable with a pointer.",
        props: &[
            Prop {
                name: "value",
                ty: "RwSignal<Option<String>>",
                default: "None",
                description: "The value of the item whose panel is open. Pass one to drive the menu or to watch it.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the menu's classes. It is the positioning context the panels are anchored to.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A list, and optionally an indicator.",
            },
        ],
    },
    ApiEntry {
        name: "NavigationMenuList",
        description: "The row. A roving-focus group, so Left and Right walk it — but every trigger and link is tabbable too, because these are links, not commands.",
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
                description: "The items.",
            },
        ],
    },
    ApiEntry {
        name: "NavigationMenuItem",
        description: "One entry of the row: a trigger with a panel, or a bare link.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "What this item's panel is called in the menu's `value`. Unique within the menu.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A trigger and a content, or a link.",
            },
        ],
    },
    ApiEntry {
        name: "NavigationMenuTrigger",
        description: "Opens a panel. On hover after a beat, on click at once, and on Down, Enter or Space — but never on focus alone, which would throw panels open as you tab past.",
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
                description: "The label; the chevron is added.",
            },
        ],
    },
    ApiEntry {
        name: "NavigationMenuContent",
        description: "A panel, anchored to the menu rather than to its item so that every panel opens in the same place. Stays mounted for 150ms after closing so its exit animation can run.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the panel's classes; where to change its width or where it hangs.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The panel's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
    ApiEntry {
        name: "NavigationMenuLink",
        description: "A link, in a panel or straight on the row. Closes the menu when followed.",
        props: &[
            Prop {
                name: "href",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Where it goes.",
            },
            Prop {
                name: "active",
                ty: "Signal<bool>",
                default: "false",
                description: "Whether this is the page the reader is on: `aria-current=\"page\"` and `data-active`.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the link's classes.",
            },
            Prop {
                name: "render",
                ty: "Option<Callback<Callback<MouseEvent>, AnyView>>",
                default: "None",
                description: "Render as something else — a router's `A`, which owns its own href. Handed the click handler that closes the menu.",
            },
            Prop {
                name: "children",
                ty: "Option<Children>",
                default: "None",
                description: "The link's contents. A `<p>` inside is styled as the description.",
            },
        ],
    },
    ApiEntry {
        name: "NavigationMenuIndicator",
        description: "The mark that slides under whichever trigger is open. Positioned by measuring that trigger, since the row is laid out by your classes.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the indicator's classes.",
        }],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let value = RwSignal::new(None::<String>);

    view! {
        <DocLayout
            title="Navigation Menu"
            description="A row of links, some of which open a panel."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Not a menubar, however alike the markup looks: these are links, so every one of them is tabbable, and the panels open on a delay and survive the pointer crossing the gap into them. One panel is open at a time, and they all open in the same place."
                    code=DEFAULT
                >
                    <div class="flex min-h-80 items-start justify-center pt-2">
                        <NavigationMenu value=value>
                            <NavigationMenuList>
                                <NavigationMenuItem value="components">
                                    <NavigationMenuTrigger>"Components"</NavigationMenuTrigger>
                                    <NavigationMenuContent>
                                        <ul class="grid w-[26rem] grid-cols-2 gap-1">
                                            {LINKS
                                                .iter()
                                                .map(|(label, href, blurb)| {
                                                    view! {
                                                        <li>
                                                            <NavigationMenuLink href=*href>
                                                                <span class="font-medium">{*label}</span>
                                                                <p>{*blurb}</p>
                                                            </NavigationMenuLink>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </ul>
                                    </NavigationMenuContent>
                                </NavigationMenuItem>

                                <NavigationMenuItem value="resources">
                                    <NavigationMenuTrigger>"Resources"</NavigationMenuTrigger>
                                    <NavigationMenuContent>
                                        <ul class="grid w-64 gap-1">
                                            {RESOURCES
                                                .iter()
                                                .map(|(label, href, blurb)| {
                                                    view! {
                                                        <li>
                                                            <NavigationMenuLink href=*href>
                                                                <span class="font-medium">{*label}</span>
                                                                <p>{*blurb}</p>
                                                            </NavigationMenuLink>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </ul>
                                    </NavigationMenuContent>
                                </NavigationMenuItem>

                                <NavigationMenuItem value="docs">
                                    <NavigationMenuLink
                                        href="/docs"
                                        class="inline-flex h-9 items-center px-3 py-2 font-medium"
                                    >
                                        "Docs"
                                    </NavigationMenuLink>
                                </NavigationMenuItem>
                            </NavigationMenuList>

                            <NavigationMenuIndicator />
                        </NavigationMenu>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Which panel is open is one signal, named by the item's value — the same shape the menubar uses."
                    code=CONTROLLED
                >
                    <p class="text-sm text-muted-foreground">
                        "The menu above is showing: "
                        <span class="font-mono text-foreground">
                            {move || value.get().unwrap_or_else(|| "nothing".to_string())}
                        </span>
                    </p>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
