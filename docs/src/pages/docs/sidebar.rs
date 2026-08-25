use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::sidebar::{
        Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
        SidebarGroupLabel, SidebarHeader, SidebarInput, SidebarInset, SidebarMenu,
        SidebarMenuAction, SidebarMenuBadge, SidebarMenuButton, SidebarMenuItem,
        SidebarMenuSkeleton, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem,
        SidebarProvider, SidebarRail, SidebarSeparator, SidebarTree, SidebarTreeGroup,
        SidebarTreeItem, SidebarTreeRow, SidebarTrigger, SidebarVariant,
    },
    icons::{copy::Copy, ellipsis::Ellipsis, search::Search},
    primitives::sidebar::SidebarCollapsible,
    utils::types::Side,
};

const DEFAULT: &str = r#"// The provider owns the state; everything under it reads the same
// context. Uncontrolled, it restores its last state from local
// storage, and Ctrl/Cmd+B toggles it from anywhere on the page.
<SidebarProvider>
    <Sidebar>
        <SidebarHeader>
            <SidebarInput placeholder="Search" />
        </SidebarHeader>
        <SidebarContent>
            <SidebarGroup>
                <SidebarGroupLabel>"Workspace"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton active=true>
                                <Search />
                                <span>"Overview"</span>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroupContent>
            </SidebarGroup>
        </SidebarContent>
        <SidebarRail />
    </Sidebar>
    <SidebarInset>
        <header class="flex h-12 items-center gap-2 border-b px-3">
            <SidebarTrigger />
        </header>
    </SidebarInset>
</SidebarProvider>"#;

const ICON: &str = r#"// Collapsed to a rail of icons instead of off the edge. The labels
// go, so each row takes a `tooltip` — rendered only while it is
// collapsed, since a tooltip repeating a visible label is noise.
<SidebarProvider collapsible=SidebarCollapsible::Icon>
    <Sidebar>
        <SidebarContent>
            <SidebarGroup>
                // Folds up under the group above rather than hiding, so
                // the icons keep their rhythm.
                <SidebarGroupLabel>"Workspace"</SidebarGroupLabel>
                <SidebarGroupContent>
                    <SidebarMenu>
                        <SidebarMenuItem>
                            <SidebarMenuButton tooltip="Overview" active=true>
                                <Search />
                                <span>"Overview"</span>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroupContent>
            </SidebarGroup>
        </SidebarContent>
    </Sidebar>
    // ...
</SidebarProvider>"#;

const SIDES: &str = r#"// The edge is the provider's, since the trigger and the rail need it
// as much as the panel does.
<SidebarProvider side=Side::Right>
    <Sidebar>// ...</Sidebar>
    <SidebarInset>// ...</SidebarInset>
</SidebarProvider>"#;

const VARIANTS: &str = r#"// `Floating` lifts the panel off the window; `Inset` leaves the panel
// flat and floats the content beside it instead.
<SidebarProvider>
    <Sidebar variant=SidebarVariant::Floating>// ...</Sidebar>
    <SidebarInset>// ...</SidebarInset>
</SidebarProvider>"#;

const MENU: &str = r#"// A row is a button, a badge, an action and a nested list. The action
// and the badge line up with whichever `size` the row asked for.
<SidebarMenuItem>
    <SidebarMenuButton>
        <Copy />
        <span>"Documents"</span>
    </SidebarMenuButton>
    <SidebarMenuBadge>"12"</SidebarMenuBadge>
</SidebarMenuItem>

<SidebarMenuItem>
    <SidebarMenuButton>
        <Search />
        <span>"Reports"</span>
    </SidebarMenuButton>
    <SidebarMenuAction show_on_hover=true>
        <Ellipsis />
    </SidebarMenuAction>
    <SidebarMenuSub>
        <SidebarMenuSubItem>
            <SidebarMenuSubButton active=true>
                <span>"Weekly"</span>
            </SidebarMenuSubButton>
        </SidebarMenuSubItem>
    </SidebarMenuSub>
</SidebarMenuItem>

// While the rows are still loading:
<SidebarMenuSkeleton width=70 />"#;

/// A box that holds a whole sidebar layout. `transform-gpu` is the point: a transformed ancestor
/// becomes the containing block for the `fixed` panel, so the demo stays in its box.
#[component]
fn Frame(children: Children) -> impl IntoView {
    view! {
        <div class="relative h-96 w-full transform-gpu overflow-hidden rounded-lg border">
            {children()}
        </div>
    }
}

/// The rows every demo shares, so each section is about one thing.
#[component]
fn Nav(#[prop(default = false)] tooltips: bool) -> impl IntoView {
    let tooltip =
        move |label: &'static str| tooltips.then(|| Signal::derive(move || label.to_string()));

    view! {
        <SidebarGroup>
            <SidebarGroupLabel>"Workspace"</SidebarGroupLabel>
            <SidebarGroupContent>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton active=true tooltip=tooltip("Overview")>
                            <Search />
                            <span>"Overview"</span>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                    <SidebarMenuItem>
                        <SidebarMenuButton tooltip=tooltip("Documents")>
                            <Copy />
                            <span>"Documents"</span>
                        </SidebarMenuButton>
                        <SidebarMenuBadge>"12"</SidebarMenuBadge>
                    </SidebarMenuItem>
                    <SidebarMenuItem>
                        <SidebarMenuButton tooltip=tooltip("Settings")>
                            <Ellipsis />
                            <span>"Settings"</span>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
    }
}

/// The page beside the sidebar: a bar with the trigger, and something to look at.
#[component]
fn Body(#[prop(default = "Overview")] title: &'static str) -> impl IntoView {
    view! {
        <SidebarInset>
            <header class="flex h-12 shrink-0 items-center gap-2 border-b px-3">
                <SidebarTrigger />
                <span class="text-sm font-medium">{title}</span>
            </header>
            <div class="flex flex-1 flex-col gap-2 p-3">
                <div class="h-16 rounded-lg bg-muted/50" />
                <div class="h-16 rounded-lg bg-muted/50" />
            </div>
        </SidebarInset>
    }
}

const TREE: &str = r#"// The tree element, wearing the sidebar's rows. One behaviour, two sets
// of classes.
<SidebarGroup>
    <SidebarGroupLabel>"Files"</SidebarGroupLabel>
    <SidebarGroupContent>
        <SidebarTree label="Files">
            <SidebarTreeItem default_open=true>
                <SidebarTreeRow>"src"</SidebarTreeRow>
                <SidebarTreeGroup>
                    <SidebarTreeItem>
                        <SidebarTreeRow>"components"</SidebarTreeRow>
                        <SidebarTreeGroup>
                            <SidebarTreeItem>
                                <SidebarTreeRow>"button.rs"</SidebarTreeRow>
                            </SidebarTreeItem>
                        </SidebarTreeGroup>
                    </SidebarTreeItem>
                    <SidebarTreeItem>
                        <SidebarTreeRow active=true>"lib.rs"</SidebarTreeRow>
                    </SidebarTreeItem>
                </SidebarTreeGroup>
            </SidebarTreeItem>
        </SidebarTree>
    </SidebarGroupContent>
</SidebarGroup>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "SidebarProvider",
        description: "Owns the state and provides it to every part below: the wide-viewport open state, which is persisted, and the narrow one, which is not.",
        props: &[
            Prop {
                name: "side",
                ty: "Side",
                default: "Side::Left",
                description: "One of: Left, Right, Bottom, Top.",
            },
            Prop {
                name: "collapsible",
                ty: "SidebarCollapsible",
                default: "OffCanvas",
                description: "One of: OffCanvas, Icon, None.",
            },
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Pass one to drive it from outside. A controlled sidebar is not persisted.",
            },
            Prop {
                name: "default_open",
                ty: "bool",
                default: "true",
                description: "Where an uncontrolled sidebar starts, before local storage has anything to say.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "style",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Added to the wrapper's own custom properties, for overriding the widths.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A sidebar and an inset.",
            },
        ],
    },
    ApiEntry {
        name: "Sidebar",
        description: "The panel. Two boxes on a wide viewport — one reserving the width, one fixed — and a sheet on a narrow one.",
        props: &[
            Prop {
                name: "variant",
                ty: "SidebarVariant",
                default: "Sidebar",
                description: "One of: Sidebar, Floating, Inset.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the container's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "A header, content, a footer and a rail. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarInset",
        description: "The page beside the sidebar; takes the room the sidebar is not using.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The page.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarTrigger",
        description: "The button that opens and closes it.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Rendered as the `disabled` attribute.",
            },
            Prop {
                name: "children",
                ty: "Option<Children>",
                default: "None",
                description: "Defaults to a panel icon and a screen-reader label.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarRail",
        description: "The strip along the panel's inner edge, which toggles it. Out of the tab order, since the trigger is already in it.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rail's classes.",
        }],
    },
    ApiEntry {
        name: "SidebarHeader",
        description: "The top of the panel, above the scrolling part.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a logo or a search box.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarFooter",
        description: "The bottom of the panel, below the scrolling part.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually an account row.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarContent",
        description: "Everything between the header and the footer; the part that scrolls.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Groups.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarSeparator",
        description: "A rule across the panel, inset from its edges.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rule's classes.",
        }],
    },
    ApiEntry {
        name: "SidebarInput",
        description: "A search box sized for the sidebar's header.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the control's classes.",
            },
            Prop {
                name: "model",
                ty: "Option<RwSignal<String>>",
                default: "None",
                description: "Two-way binding.",
            },
            Prop {
                name: "placeholder",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Shown while it is empty.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarGroup",
        description: "A section of the menu, with its own label and action.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A label, an action and a content.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarGroupLabel",
        description: "The heading over a group; folds away when the sidebar collapses to icons.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the heading classes.",
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
        name: "SidebarGroupAction",
        description: "A button in a group's top-right corner.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually an icon.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarGroupContent",
        description: "The group's body.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a menu.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenu",
        description: "The list of rows.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the list's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenuItem",
        description: "One row and whatever hangs off it: an action, a badge, a sub-list.",
        props: &[
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
                description: "A button, and optionally an action, a badge or a sub-menu.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenuButton",
        description: "A row in the menu: the icon, the label and the current-page state. `tooltip` stands in for the label once the sidebar is collapsed to icons, and is rendered only in that state.",
        props: &[
            Prop {
                name: "active",
                ty: "Signal<bool>",
                default: "false",
                description: "The current page's row. Announced with `aria-current`, not just painted.",
            },
            Prop {
                name: "size",
                ty: "SidebarMenuSize",
                default: "Default",
                description: "One of: Default, Sm, Lg.",
            },
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
                description: "Stops it taking pointer events and marks it disabled.",
            },
            Prop {
                name: "tooltip",
                ty: "Option<Signal<String>>",
                default: "None",
                description: "Stands in for the label once the sidebar is collapsed to icons, and is rendered only then.",
            },
            Prop {
                name: "render",
                ty: "Option<Callback<(Signal<String>, AnyNodeRef), AnyView>>",
                default: "None",
                description: "Render the row as something else — usually an `<a>`. Handed the row's class and the node ref its state attributes go on.",
            },
            Prop {
                name: "children",
                ty: "Option<ChildrenFn>",
                default: "None",
                description: "An icon and a label.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenuAction",
        description: "A second control on a menu row: a \"more\" menu, a dismiss.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
            Prop {
                name: "show_on_hover",
                ty: "bool",
                default: "false",
                description: "Keep it hidden until the row is hovered or focused.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually an icon.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenuBadge",
        description: "A count on a menu row; not interactive, so it never takes the row's click.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the badge's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a number.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenuSkeleton",
        description: "A placeholder row, for a menu that is still loading.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the row's classes.",
            },
            Prop {
                name: "show_icon",
                ty: "bool",
                default: "true",
                description: "Draw a square where the icon will be.",
            },
            Prop {
                name: "width",
                ty: "u32",
                default: "70",
                description: "Width of the text bar, as a percentage; vary it across a list.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenuSub",
        description: "The nested list under an expanded row.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the list's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Sub-items.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenuSubItem",
        description: "One row of the nested list.",
        props: &[
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
                description: "Usually a sub-button.",
            },
        ],
    },
    ApiEntry {
        name: "SidebarMenuSubButton",
        description: "A row in the nested list, shorter than a top-level one.",
        props: &[
            Prop {
                name: "active",
                ty: "Signal<bool>",
                default: "false",
                description: "The current page's row.",
            },
            Prop {
                name: "size",
                ty: "SidebarMenuSize",
                default: "SidebarMenuSize::Default",
                description: "One of: Default, Sm, Lg.",
            },
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
                description: "Stops it taking pointer events and marks it disabled.",
            },
            Prop {
                name: "render",
                ty: "Option<Callback<(Signal<String>, AnyNodeRef), AnyView>>",
                default: "None",
                description: "Render the row as something else — usually an `<a>`.",
            },
            Prop {
                name: "children",
                ty: "Option<ChildrenFn>",
                default: "None",
                description: "The label.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    // Controlled, so the demos do not all write the one stored preference between them.
    let default_open = RwSignal::new(true);
    let icon_open = RwSignal::new(true);
    let right_open = RwSignal::new(true);
    let tree_open = RwSignal::new(true);
    let floating_open = RwSignal::new(true);
    let inset_open = RwSignal::new(true);
    let menu_open = RwSignal::new(true);

    view! {
        <DocLayout
            title="Sidebar"
            description="A navigation panel that collapses, and remembers that it is collapsed."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The provider owns the state and everything under it reads the same context — the trigger, the rail, and each row. Uncontrolled it restores its last state from local storage, and Ctrl/Cmd+B toggles it from anywhere on the page. Collapsing slides the panel off the edge, and the content beside it takes the room back."
                    code=DEFAULT
                >
                    <Frame>
                        <SidebarProvider open=default_open class="min-h-full">
                            <Sidebar>
                                <SidebarHeader>
                                    <SidebarInput placeholder="Search" />
                                </SidebarHeader>
                                <SidebarContent>
                                    <Nav />
                                </SidebarContent>
                                <SidebarSeparator />
                                <SidebarFooter>
                                    <span class="px-2 text-xs text-muted-foreground">
                                        "yuri@nocomponents"
                                    </span>
                                </SidebarFooter>
                                <SidebarRail />
                            </Sidebar>
                            <Body />
                        </SidebarProvider>
                    </Frame>
                </DemoSection>

                <DemoSection
                    title="Collapsed to icons"
                    description="The other thing collapsing can mean: a rail of icons instead of nothing. The labels go with the width, so each row takes a tooltip — rendered only while the sidebar is collapsed, since a tooltip that repeats a label the reader can already see is noise. Group labels fold up under the group above rather than hiding, which keeps the icons from shifting."
                    code=ICON
                >
                    <Frame>
                        <SidebarProvider
                            open=icon_open
                            collapsible=SidebarCollapsible::Icon
                            class="min-h-full"
                        >
                            <Sidebar>
                                <SidebarContent>
                                    <Nav tooltips=true />
                                </SidebarContent>
                                <SidebarRail />
                            </Sidebar>
                            <Body title="Icons" />
                        </SidebarProvider>
                    </Frame>
                </DemoSection>

                <DemoSection
                    title="A tree of rows"
                    description="The same tree element the Tree page documents, wearing the sidebar's rows instead of its own — one behaviour, two sets of classes. It folds away with the rest when the sidebar collapses to icons, since a tree of labels has nothing to show at that width."
                    code=TREE
                >
                    <Frame>
                        <SidebarProvider open=tree_open class="min-h-full">
                            <Sidebar>
                                <SidebarContent>
                                    <SidebarGroup>
                                        <SidebarGroupLabel>"Files"</SidebarGroupLabel>
                                        <SidebarGroupContent>
                                            <SidebarTree label="Files">
                                                <SidebarTreeItem default_open=true>
                                                    <SidebarTreeRow>"src"</SidebarTreeRow>
                                                    <SidebarTreeGroup>
                                                        <SidebarTreeItem>
                                                            <SidebarTreeRow>"components"</SidebarTreeRow>
                                                            <SidebarTreeGroup>
                                                                <SidebarTreeItem>
                                                                    <SidebarTreeRow>"button.rs"</SidebarTreeRow>
                                                                </SidebarTreeItem>
                                                                <SidebarTreeItem>
                                                                    <SidebarTreeRow>"tree.rs"</SidebarTreeRow>
                                                                </SidebarTreeItem>
                                                            </SidebarTreeGroup>
                                                        </SidebarTreeItem>
                                                        <SidebarTreeItem>
                                                            <SidebarTreeRow active=true>"lib.rs"</SidebarTreeRow>
                                                        </SidebarTreeItem>
                                                    </SidebarTreeGroup>
                                                </SidebarTreeItem>
                                                <SidebarTreeItem>
                                                    <SidebarTreeRow>"Cargo.toml"</SidebarTreeRow>
                                                </SidebarTreeItem>
                                            </SidebarTree>
                                        </SidebarGroupContent>
                                    </SidebarGroup>
                                </SidebarContent>
                                <SidebarRail />
                            </Sidebar>
                            <Body title="Files" />
                        </SidebarProvider>
                    </Frame>
                </DemoSection>

                <DemoSection
                    title="From the right"
                    description="The edge is set once, on the provider: the panel pins to it, the rail moves to the panel's inner edge, and the gap the layout reserves swaps sides with it."
                    code=SIDES
                >
                    <Frame>
                        <SidebarProvider open=right_open side=Side::Right class="min-h-full">
                            <Sidebar>
                                <SidebarContent>
                                    <Nav />
                                </SidebarContent>
                                <SidebarRail />
                            </Sidebar>
                            <Body title="Right" />
                        </SidebarProvider>
                    </Frame>
                </DemoSection>

                <DemoSection
                    title="Variants"
                    description="Floating lifts the panel off the window — inset, rounded, its own shadow. Inset does the opposite: the panel keeps the page's background and the content beside it becomes the card, which is why the wrapper takes the sidebar's colour."
                    code=VARIANTS
                >
                    <div class="flex flex-col gap-4">
                        <Frame>
                            <SidebarProvider open=floating_open class="min-h-full">
                                <Sidebar variant=SidebarVariant::Floating>
                                    <SidebarContent>
                                        <Nav />
                                    </SidebarContent>
                                    <SidebarRail />
                                </Sidebar>
                                <Body title="Floating" />
                            </SidebarProvider>
                        </Frame>

                        <Frame>
                            <SidebarProvider open=inset_open class="min-h-full">
                                <Sidebar variant=SidebarVariant::Inset>
                                    <SidebarContent>
                                        <Nav />
                                    </SidebarContent>
                                    <SidebarRail />
                                </Sidebar>
                                <Body title="Inset" />
                            </SidebarProvider>
                        </Frame>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Menu parts"
                    description="A row can carry a count, a second control, and a nested list. The badge and the action line up with whichever size the row asked for, and both go when the sidebar collapses to icons — there is no room, and nothing to line up with. The action can wait for a hover, which is what keeps a long list from being a wall of buttons."
                    code=MENU
                >
                    <Frame>
                        <SidebarProvider open=menu_open class="min-h-full">
                            <Sidebar>
                                <SidebarContent>
                                    <SidebarGroup>
                                        <SidebarGroupLabel>"Library"</SidebarGroupLabel>
                                        <SidebarGroupContent>
                                            <SidebarMenu>
                                                <SidebarMenuItem>
                                                    <SidebarMenuButton>
                                                        <Copy />
                                                        <span>"Documents"</span>
                                                    </SidebarMenuButton>
                                                    <SidebarMenuBadge>"12"</SidebarMenuBadge>
                                                </SidebarMenuItem>

                                                <SidebarMenuItem>
                                                    <SidebarMenuButton>
                                                        <Search />
                                                        <span>"Reports"</span>
                                                    </SidebarMenuButton>
                                                    <SidebarMenuAction show_on_hover=true>
                                                        <Ellipsis />
                                                    </SidebarMenuAction>
                                                    <SidebarMenuSub>
                                                        <SidebarMenuSubItem>
                                                            <SidebarMenuSubButton active=true>
                                                                <span>"Weekly"</span>
                                                            </SidebarMenuSubButton>
                                                        </SidebarMenuSubItem>
                                                        <SidebarMenuSubItem>
                                                            <SidebarMenuSubButton>
                                                                <span>"Monthly"</span>
                                                            </SidebarMenuSubButton>
                                                        </SidebarMenuSubItem>
                                                    </SidebarMenuSub>
                                                </SidebarMenuItem>

                                                <SidebarMenuItem>
                                                    <SidebarMenuSkeleton width=70 />
                                                </SidebarMenuItem>
                                                <SidebarMenuItem>
                                                    <SidebarMenuSkeleton width=45 />
                                                </SidebarMenuItem>
                                            </SidebarMenu>
                                        </SidebarGroupContent>
                                    </SidebarGroup>
                                </SidebarContent>
                                <SidebarRail />
                            </Sidebar>
                            <Body title="Library" />
                        </SidebarProvider>
                    </Frame>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
