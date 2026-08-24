use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        avatar::{Avatar, AvatarFallback},
        breadcrumb::{
            Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
            BreadcrumbSeparator,
        },
        button::ButtonVariant,
        dialog::{Dialog, DialogDescription, DialogPortal, DialogTitle, DialogTrigger},
        dropdown_menu::{
            DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
            DropdownMenuPortal, DropdownMenuSeparator, DropdownMenuTrigger,
        },
        field::{Field, FieldDescription, FieldGroup, FieldLabel},
        separator::Separator,
        sidebar::{
            Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
            SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuBadge,
            SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail, SidebarTrigger,
        },
        switch::Switch,
    },
    icons::{
        calendar::Calendar, chevron::ChevronDown, copy::Copy, ellipsis::Ellipsis, search::Search,
    },
    primitives::sidebar::{SidebarCollapsible, use_sidebar},
    utils::types::{Align, Orientation, Side},
};

#[derive(Clone, Copy, PartialEq)]
struct Page {
    slug: &'static str,
    label: &'static str,
    section: &'static str,
    badge: Option<&'static str>,
}

#[rustfmt::skip]
const PAGES: &[Page] = &[
    Page { slug: "overview",  label: "Overview",  section: "Workspace", badge: None },
    Page { slug: "documents", label: "Documents", section: "Workspace", badge: Some("12") },
    Page { slug: "schedule",  label: "Schedule",  section: "Workspace", badge: None },
    Page { slug: "members",   label: "Members",   section: "Account",   badge: Some("4") },
    Page { slug: "billing",   label: "Billing",   section: "Account",   badge: None },
];

/// One switch in the settings dialog. `on` is where it starts, not where it stays.
#[derive(Clone, Copy, PartialEq)]
struct Setting {
    label: &'static str,
    description: &'static str,
    on: bool,
}

/// One pane of the settings dialog: what the row in the panel says, and what the pane holds.
#[derive(Clone, Copy, PartialEq)]
struct Section {
    slug: &'static str,
    label: &'static str,
    group: &'static str,
    blurb: &'static str,
    settings: &'static [Setting],
}

/// The order the groups come in. Read off `SECTIONS` rather than declared twice.
const GROUPS: &[&str] = &["Settings", "Workspace"];

#[rustfmt::skip]
const SECTIONS: &[Section] = &[
    Section {
        slug: "general", label: "General", group: "Settings",
        blurb: "How the workspace behaves, for everyone in it.",
        settings: &[
            Setting { label: "Autosave", description: "Write every change as it is made.", on: true },
            Setting { label: "Confirm before leaving", description: "Ask when a page still has unsaved work.", on: false },
        ],
    },
    Section {
        slug: "appearance", label: "Appearance", group: "Settings",
        blurb: "Only for you, and only on this device.",
        settings: &[
            Setting { label: "Reduce motion", description: "Cut the panel and dialog animations.", on: false },
            Setting { label: "Compact rows", description: "Tighter line height in every list.", on: true },
        ],
    },
    Section {
        slug: "notifications", label: "Notifications", group: "Settings",
        blurb: "What is worth an email, and what is not.",
        settings: &[
            Setting { label: "Mentions", description: "Someone names you in a comment.", on: true },
            Setting { label: "Weekly digest", description: "One summary, on Monday morning.", on: true },
            Setting { label: "Everything else", description: "Every change anyone makes.", on: false },
        ],
    },
    Section {
        slug: "members", label: "Members", group: "Workspace",
        blurb: "Who gets in, and who decides.",
        settings: &[
            Setting { label: "Anyone can invite", description: "Members may add members.", on: false },
            Setting { label: "Approve requests", description: "An admin signs off on each join.", on: true },
        ],
    },
    Section {
        slug: "billing", label: "Billing", group: "Workspace",
        blurb: "The plan, and where the receipts go.",
        settings: &[
            Setting { label: "Annual billing", description: "One invoice a year, two months off.", on: false },
            Setting { label: "Email receipts", description: "Send a copy to the workspace owner.", on: true },
        ],
    },
];

const SHELL: &str = r##"// One list of pages, read three times over: the menu renders it, the
// breadcrumb looks the current one up, and the body shows it. The sidebar
// is not "navigation beside a page" — it holds the state the page is
// drawn from, so there is nothing to keep in step.
struct Page { slug: &'static str, label: &'static str, section: &'static str }

let current = RwSignal::new("overview");
let showing = Signal::derive(move || page(current.get()));

view! {
    // Only because this one lives in a box on a documentation page: the panel is
    // `position: fixed`, and a transform on an ancestor is what gives a fixed element
    // a containing block. A real app wants the panel pinned to the window, and drops
    // both this wrapper and the `open` signal — uncontrolled, it remembers its own
    // state in local storage.
    <div class="relative h-[30rem] w-full transform-gpu overflow-hidden rounded-lg border">
    <SidebarProvider open=open collapsible=SidebarCollapsible::Icon class="min-h-full">
        <Sidebar>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>"Workspace"</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            <SidebarMenuItem>
                                // `tooltip` stands in for the label once the panel has
                                // collapsed to icons, and renders only in that state —
                                // a tooltip repeating a visible label is noise.
                                <SidebarMenuButton
                                    active=Signal::derive(move || current.get() == slug)
                                    tooltip=Signal::derive(move || label.to_string())
                                    on:click=move |_| current.set(slug)
                                >
                                    <Copy />
                                    <span>{label}</span>
                                </SidebarMenuButton>
                                <SidebarMenuBadge>"12"</SidebarMenuBadge>
                            </SidebarMenuItem>
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>

            <SidebarFooter>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <DropdownMenu>
                            // The row *is* the trigger. Both want to reach the same element —
                            // the menu to measure what it hangs off, the row to carry its own
                            // state — and leptos allows one `node_ref` per element, so the one
                            // the trigger hands over goes into the row.
                            <DropdownMenuTrigger render=Callback::new(|(node, click)| {
                                view! {
                                    <SidebarMenuButton
                                        node_ref=node
                                        on:click=move |e| click.run(e)
                                    >
                                        <Avatar class="size-6">
                                            <AvatarFallback>"YO"</AvatarFallback>
                                        </Avatar>
                                        <span class="truncate">"Yuri"</span>
                                        <ChevronDown class="ml-auto size-4 opacity-60" />
                                    </SidebarMenuButton>
                                }.into_any()
                            }) />
                            <DropdownMenuPortal>
                                <DropdownMenuContent side=Side::Top align=Align::Start class="w-48">
                                    <DropdownMenuItem>"Account settings"</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem>"Sign out"</DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenuPortal>
                        </DropdownMenu>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarFooter>
            <SidebarRail />
        </Sidebar>

        <SidebarInset>
            <header class="flex h-12 shrink-0 items-center gap-2 border-b px-3">
                <SidebarTrigger />
                <Separator orientation=Orientation::Vertical class="h-4" />
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="#">
                                {move || showing.get().section}
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage>{move || showing.get().label}</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </header>

            <h3 class="text-base font-semibold">{move || showing.get().label}</h3>
        </SidebarInset>
    </SidebarProvider>
    </div>
}"##;

const SETTINGS: &str = r##"// The panel is not navigation here either: it picks which pane of settings
// the dialog is showing, and the dialog is titled by whatever it picked.
struct Section { slug: &'static str, label: &'static str, group: &'static str }

let current = RwSignal::new("general");
let showing = Signal::derive(move || section(current.get()));

// A sidebar inside a dialog is not the application's sidebar. Left uncontrolled
// it reads — and writes — the same `ui-sidebar` key the page's own sidebar
// remembers itself in, so this one is handed a signal of its own.
let open = RwSignal::new(true);

// One signal per switch, minted out here. The pane is rebuilt whenever another
// section is picked, so anything held inside it would snap back to its default.
let switches: Vec<(&'static str, RwSignal<bool>)> = ...;

view! {
    <Dialog>
        <DialogTrigger variant=ButtonVariant::Outline>"Workspace settings"</DialogTrigger>

        // `p-0` because the panel goes to the edges, and a fixed height because the
        // panel is `position: fixed` and this is the transformed ancestor that pins
        // it. `sm:max-w-3xl` widens the box a dialog usually gets without touching
        // the `max-w-[calc(100%-2rem)]` that keeps it on a phone screen.
        <DialogPortal class="h-[24rem] max-h-[calc(100dvh-2rem)] gap-0 overflow-hidden p-0 sm:max-w-3xl">
            <SidebarProvider
                open=open
                collapsible=SidebarCollapsible::None
                class="min-h-0"
                style="--sidebar-width: 12rem;"
            >
                <Sidebar>
                    <SidebarHeader class="h-12 shrink-0 justify-center border-b">
                        <span class="truncate px-2 text-sm font-medium">"Northwind"</span>
                    </SidebarHeader>
                    <SidebarContent>
                        <SidebarGroup>
                            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
                            <SidebarGroupContent>
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton
                                            active=Signal::derive(move || current.get() == slug)
                                            // `let sidebar = use_sidebar()` in the component
                                            // that renders these rows — the context only
                                            // exists under the provider that gave it. Below
                                            // `md` the panel is a sheet lying over the pane
                                            // it just changed, so picking a section is also
                                            // done with the panel.
                                            on:click=move |_| {
                                                current.set(slug);
                                                if sidebar.is_mobile.get() {
                                                    sidebar.set_open(false);
                                                }
                                            }
                                        >
                                            <span>{label}</span>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarGroupContent>
                        </SidebarGroup>
                    </SidebarContent>
                </Sidebar>

                <SidebarInset>
                    // The same height as the panel's own header, so the two rules meet.
                    // `pr-12` leaves the corner to the dialog's close button.
                    <header class="flex h-12 shrink-0 items-center gap-2 border-b px-4 pr-12">
                        // Below `md` the panel is a sheet over the dialog, and this is the
                        // only way back to it. Above `md` it is always there, so the trigger
                        // would have nothing to say.
                        <SidebarTrigger class="md:hidden" />
                        // Mounted once and re-read, not re-mounted per section: the title
                        // is what `aria-labelledby` points at, and a new node is a new id.
                        <DialogTitle class="text-sm">{move || showing.get().label}</DialogTitle>
                    </header>

                    <div class="flex min-h-0 flex-1 flex-col gap-4 overflow-auto p-4">
                        <DialogDescription>{move || showing.get().blurb}</DialogDescription>
                        <FieldGroup class="gap-4">
                            <Field orientation=Orientation::Horizontal class="justify-between gap-6">
                                <div class="flex flex-col gap-1">
                                    <FieldLabel>{setting.label}</FieldLabel>
                                    <FieldDescription>{setting.description}</FieldDescription>
                                </div>
                                // The field mints the id and the switch wears it, so the
                                // label points at the control without either being told.
                                <Switch checked=checked />
                            </Field>
                        </FieldGroup>
                    </div>
                </SidebarInset>
            </SidebarProvider>
        </DialogPortal>
    </Dialog>
}"##;

pub const BLOCKS: &[Block] = &[
    Block {
        title: "An application shell",
        description: "The whole frame of an app in one composition, and the seam is that there is no \
                  seam: the sidebar does not sit *beside* the page, it holds the state the page is \
                  drawn from. One list of pages feeds the menu, the breadcrumb and the body, so \
                  the three cannot drift. Collapse it with the trigger — or ⌘B from anywhere — and \
                  the labels go, which is why every row carries a `tooltip` that is rendered only \
                  once there is no label left to repeat. The footer's account menu is the other \
                  seam: a dropdown anchored inside the panel, which on a narrow viewport is itself \
                  a sheet, so it is a menu in a portal over a dialog over the page. Escape closes \
                  them one at a time, innermost first, because each layer keeps its own dismissal \
                  rather than the outermost swallowing everything.",
        code: SHELL,
        view: || view! { <AppShell /> }.into_any(),
    },
    Block {
        title: "Settings, with the sidebar inside the dialog",
        description: "The same panel, doing a smaller job: a sidebar is not page-level navigation, \
                  it is a panel over the state something else is drawn from — so it fits inside a \
                  dialog on a page that already has one of its own. Two things make that work. The \
                  panel is `position: fixed`, and the dialog is `-translate-x-1/2 -translate-y-1/2`, \
                  which is what gives a fixed element its containing block: the panel is pinned to \
                  the dialog rather than to the window, and the dialog's `overflow-hidden` is what \
                  rounds its corners off. And the provider is handed an `open` signal of its own, \
                  because an uncontrolled one reads *and writes* the `ui-sidebar` key the page's \
                  own sidebar remembers itself in — the dialog would quietly close the sidebar \
                  behind it. Every switch's signal is minted above the pane, since picking another \
                  section rebuilds the pane and anything held inside would snap back to its \
                  default. There is no footer and nothing to save: a switch is the change. Below \
                  `md` the panel is a sheet portalled over the dialog, which is why one trigger \
                  appears at that width and nowhere else — above it the panel never leaves.",
        code: SETTINGS,
        view: || view! { <SettingsDialog /> }.into_any(),
    },
];

fn page(slug: &str) -> Page {
    PAGES
        .iter()
        .copied()
        .find(|page| page.slug == slug)
        .unwrap_or(PAGES[0])
}

fn section(slug: &str) -> Section {
    SECTIONS
        .iter()
        .copied()
        .find(|section| section.slug == slug)
        .unwrap_or(SECTIONS[0])
}

#[component]
fn NavGroup(section: &'static str, current: RwSignal<&'static str>) -> impl IntoView {
    view! {
        <SidebarGroup>
            <SidebarGroupLabel>{section}</SidebarGroupLabel>
            <SidebarGroupContent>
                <SidebarMenu>
                    {PAGES
                        .iter()
                        .filter(|page| page.section == section)
                        .map(|page| {
                            let Page { slug, label, badge, .. } = *page;

                            view! {
                                <SidebarMenuItem>
                                    <SidebarMenuButton
                                        active=Signal::derive(move || current.get() == slug)
                                        tooltip=Signal::derive(move || label.to_string())
                                        on:click=move |_| current.set(slug)
                                    >
                                        {match section {
                                            "Workspace" => view! { <Copy /> }.into_any(),
                                            _ => view! { <Search /> }.into_any(),
                                        }}
                                        <span>{label}</span>
                                    </SidebarMenuButton>
                                    {badge
                                        .map(|badge| {
                                            view! { <SidebarMenuBadge>{badge}</SidebarMenuBadge> }
                                        })}
                                </SidebarMenuItem>
                            }
                        })
                        .collect_view()}
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
    }
}

#[component]
fn AccountMenu() -> impl IntoView {
    view! {
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger render=Callback::new(|(node, click): (_, Callback<_>)| {
                        view! {
                            <SidebarMenuButton
                                node_ref=node
                                tooltip=Signal::derive(|| "Yuri".to_string())
                                on:click=move |e| click.run(e)
                            >
                                <Avatar class="size-6">
                                    <AvatarFallback class="text-[10px]">"YO"</AvatarFallback>
                                </Avatar>
                                <span class="truncate">"Yuri"</span>
                                <ChevronDown class="ml-auto size-4 opacity-60" />
                            </SidebarMenuButton>
                        }
                            .into_any()
                    }) />
                    <DropdownMenuPortal>
                        <DropdownMenuContent side=Side::Top align=Align::Start class="w-48">
                            <DropdownMenuLabel>"yuri@example.com"</DropdownMenuLabel>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem>"Account settings"</DropdownMenuItem>
                            <DropdownMenuItem>"Billing"</DropdownMenuItem>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem>"Sign out"</DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenuPortal>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    }
}

/// One group of section rows in the dialog's panel.
#[component]
fn SettingsNav(group: &'static str, current: RwSignal<&'static str>) -> impl IntoView {
    // Only for `is_mobile`: below `md` the panel is a sheet lying over the pane it has just
    // changed, so picking a section is also done with the panel.
    let ctx = use_sidebar();

    view! {
        <SidebarGroup>
            <SidebarGroupLabel>{group}</SidebarGroupLabel>
            <SidebarGroupContent>
                <SidebarMenu>
                    {SECTIONS
                        .iter()
                        .filter(|section| section.group == group)
                        .map(|section| {
                            let Section { slug, label, .. } = *section;
                            let pick = move |_| {
                                current.set(slug);
                                if ctx.is_mobile.get() {
                                    ctx.set_open(false);
                                }
                            };

                            view! {
                                <SidebarMenuItem>
                                    <SidebarMenuButton
                                        active=Signal::derive(move || current.get() == slug)
                                        on:click=pick
                                    >
                                        <span>{label}</span>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            }
                        })
                        .collect_view()}
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
    }
}

/// A label, what it does, and the switch. The field mints the id and the switch wears it, so
/// nothing here has to name one.
#[component]
fn SettingRow(setting: Setting, checked: RwSignal<bool>) -> impl IntoView {
    view! {
        <Field orientation=Orientation::Horizontal class="justify-between gap-6">
            <div class="flex flex-col gap-1">
                <FieldLabel>{setting.label}</FieldLabel>
                <FieldDescription>{setting.description}</FieldDescription>
            </div>
            <Switch checked=checked />
        </Field>
    }
}

#[component]
fn SettingsDialog() -> impl IntoView {
    let current = RwSignal::new("general");
    let showing = Signal::derive(move || section(current.get()));

    // A sidebar inside a dialog is not the application's sidebar. Left uncontrolled it reads —
    // and writes — the same `ui-sidebar` key the page's own sidebar remembers itself in, so
    // opening this would close that one behind it.
    let open = RwSignal::new(true);

    // One signal per switch, minted out here. The pane is rebuilt whenever another section is
    // picked, so anything held inside it would snap back to its default.
    let switches: StoredValue<Vec<(&'static str, RwSignal<bool>)>> = StoredValue::new(
        SECTIONS
            .iter()
            .flat_map(|section| section.settings.iter())
            .map(|setting| (setting.label, RwSignal::new(setting.on)))
            .collect(),
    );

    view! {
        <Dialog>
            <DialogTrigger variant=ButtonVariant::Outline>"Workspace settings"</DialogTrigger>

            // `p-0` because the panel goes to the edges, and a height because the panel is
            // `position: fixed` and this is the transformed ancestor that pins it — a box that
            // grew with its content would leave the panel measuring the last pane. `sm:max-w-3xl`
            // widens it without touching the `max-w-[calc(100%-2rem)]` underneath, which is what
            // keeps the whole thing on a phone screen.
            <DialogPortal class="h-[24rem] max-h-[calc(100dvh-2rem)] gap-0 overflow-hidden p-0 sm:max-w-3xl">
                <SidebarProvider
                    open=open
                    collapsible=SidebarCollapsible::None
                    class="min-h-0"
                    style="--sidebar-width: 12rem;"
                >
                    <Sidebar>
                        <SidebarHeader class="h-12 shrink-0 justify-center border-b">
                            <span class="truncate px-2 text-sm font-medium">"Northwind"</span>
                        </SidebarHeader>
                        <SidebarContent>
                            {GROUPS
                                .iter()
                                .map(|group| view! { <SettingsNav group=group current=current /> })
                                .collect_view()}
                        </SidebarContent>
                    </Sidebar>

                    <SidebarInset>
                        // The same height as the panel's own header, so the two rules meet.
                        // `pr-12` leaves the corner to the dialog's close button.
                        <header class="flex h-12 shrink-0 items-center gap-2 border-b px-4 pr-12">
                            // Below `md` the panel is a sheet over the dialog, and this is the
                            // only way back to it. Above `md` it never leaves, so the trigger
                            // would have nothing to say.
                            <SidebarTrigger class="md:hidden" />
                            // Mounted once and re-read, rather than re-mounted per section:
                            // the title is what `aria-labelledby` points at, and a fresh node
                            // would mint a fresh id every time.
                            <DialogTitle class="text-sm">{move || showing.get().label}</DialogTitle>
                        </header>

                        <div class="flex min-h-0 flex-1 flex-col gap-4 overflow-auto p-4">
                            <DialogDescription>{move || showing.get().blurb}</DialogDescription>
                            <FieldGroup class="gap-4">
                                {move || {
                                    showing
                                        .get()
                                        .settings
                                        .iter()
                                        .map(|setting| {
                                            let checked = switches
                                                .with_value(|switches| {
                                                    switches
                                                        .iter()
                                                        .find(|(label, _)| *label == setting.label)
                                                        .map(|(_, checked)| *checked)
                                                })
                                                .unwrap_or_else(|| RwSignal::new(setting.on));

                                            view! { <SettingRow setting=*setting checked=checked /> }
                                        })
                                        .collect_view()
                                }}
                            </FieldGroup>
                        </div>
                    </SidebarInset>
                </SidebarProvider>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
fn AppShell() -> impl IntoView {
    let current = RwSignal::new("overview");
    let showing = Signal::derive(move || page(current.get()));

    let open = RwSignal::new(true);

    view! {
        <div class="relative h-[30rem] w-full transform-gpu overflow-hidden rounded-lg border">
            <SidebarProvider
                open=open
                collapsible=SidebarCollapsible::Icon
                class="min-h-full"
                style="--sidebar-width: 13rem;"
            >
                <Sidebar>
                    <SidebarHeader>
                        <div class="flex items-center gap-2 px-1 py-1.5">
                            <div class="flex size-7 shrink-0 items-center justify-center rounded-md bg-sidebar-primary text-sidebar-primary-foreground">
                                <Calendar class="size-4" />
                            </div>
                            <div class="flex min-w-0 flex-col group-data-[state=collapsed]:hidden">
                                <span class="truncate text-sm font-semibold">"Northwind"</span>
                                <span class="truncate text-xs text-muted-foreground">
                                    "Free plan"
                                </span>
                            </div>
                        </div>
                    </SidebarHeader>

                    <SidebarContent>
                        <NavGroup section="Workspace" current=current />
                        <NavGroup section="Account" current=current />
                    </SidebarContent>

                    <SidebarFooter>
                        <AccountMenu />
                    </SidebarFooter>
                    <SidebarRail />
                </Sidebar>

                <SidebarInset>
                    <header class="flex h-12 shrink-0 items-center gap-2 border-b px-3">
                        <SidebarTrigger />
                        <Separator orientation=Orientation::Vertical class="h-4" />
                        <Breadcrumb>
                            <BreadcrumbList>
                                <BreadcrumbItem>
                                    <BreadcrumbLink href="#">
                                        {move || showing.get().section}
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbPage>{move || showing.get().label}</BreadcrumbPage>
                                </BreadcrumbItem>
                            </BreadcrumbList>
                        </Breadcrumb>
                        <Ellipsis class="ml-auto size-4 text-muted-foreground" />
                    </header>

                    <div class="flex flex-1 flex-col gap-3 overflow-auto p-4">
                        <h3 class="text-base font-semibold">{move || showing.get().label}</h3>
                        <p class="max-w-prose text-sm text-muted-foreground">
                            "The heading, the breadcrumb and the highlighted row are the same
                            value read three times. Pick another page and all three move at
                            once, because there is nothing to keep in step."
                        </p>
                        <div class="grid grid-cols-2 gap-3">
                            <div class="h-16 rounded-lg bg-muted/60" />
                            <div class="h-16 rounded-lg bg-muted/60" />
                        </div>
                    </div>
                </SidebarInset>
            </SidebarProvider>
        </div>
    }
}
