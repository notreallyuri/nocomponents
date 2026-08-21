use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::{
    components::sidebar::{
        Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
        SidebarGroupLabel, SidebarHeader, SidebarInput, SidebarInset, SidebarMenu,
        SidebarMenuAction, SidebarMenuBadge, SidebarMenuButton, SidebarMenuItem,
        SidebarMenuSkeleton, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem,
        SidebarProvider, SidebarRail, SidebarSeparator, SidebarTrigger, SidebarVariant,
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

/// A box that holds a whole sidebar layout.
///
/// `transform-gpu` is the point: a transformed ancestor becomes the containing block for the
/// `fixed` panel inside it, so the demo stays in its box instead of pinning itself to the window.
/// `md:` is what the sidebar's own breakpoint keys off, so the box also has to be wide enough for
/// the desktop presentation to be the one on show.
#[component]
fn Frame(children: Children) -> impl IntoView {
    view! {
        <div class="relative h-96 w-full transform-gpu overflow-hidden rounded-lg border">
            {children()}
        </div>
    }
}

/// The rows every demo shares, so each section is about the one thing it demonstrates.
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

/// The page beside the sidebar: a bar with the trigger in it, and something to look at.
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

#[component]
pub fn Page() -> impl IntoView {
    // Controlled, so the demos on this page do not all write the one stored preference between
    // them. An uncontrolled sidebar is the common case and persists on its own.
    let default_open = RwSignal::new(true);
    let icon_open = RwSignal::new(true);
    let right_open = RwSignal::new(true);
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
            </div>
        </DocLayout>
    }
}
