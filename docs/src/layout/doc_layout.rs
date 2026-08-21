use crate::{
    app::href,
    components::{doc_search::DocSearch, theme_switcher::ThemeSwitcher},
};
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};
use nocomponents::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader,
    SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail,
    SidebarTrigger,
};

#[derive(Clone)]
pub struct NavGroup {
    pub label: &'static str,
    pub items: &'static [NavItem],
}

#[derive(Clone)]
pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
}

/// Every page in the docs, in the order the sidebar renders them. Also what the ⌘K palette
/// searches, so the two cannot disagree about what exists.
pub const NAV: &[NavGroup] = &[
    NavGroup {
        label: "Sections",
        items: &[
            NavItem {
                label: "Introduction",
                href: "/docs/introduction",
            },
            NavItem {
                label: "Components",
                href: "/docs",
            },
            NavItem {
                label: "Installation",
                href: "/docs/installation",
            },
        ],
    },
    NavGroup {
        label: "Components",
        items: &[
            NavItem {
                label: "Accordion",
                href: "/docs/accordion",
            },
            NavItem {
                label: "Alert",
                href: "/docs/alert",
            },
            NavItem {
                label: "Alert Dialog",
                href: "/docs/alert-dialog",
            },
            NavItem {
                label: "Aspect Ratio",
                href: "/docs/aspect-ratio",
            },
            NavItem {
                label: "Avatar",
                href: "/docs/avatar",
            },
            NavItem {
                label: "Badge",
                href: "/docs/badge",
            },
            NavItem {
                label: "Breadcrumb",
                href: "/docs/breadcrumb",
            },
            NavItem {
                label: "Button",
                href: "/docs/button",
            },
            NavItem {
                label: "Button Group",
                href: "/docs/button-group",
            },
            NavItem {
                label: "Card",
                href: "/docs/card",
            },
            NavItem {
                label: "Checkbox",
                href: "/docs/checkbox",
            },
            NavItem {
                label: "Code",
                href: "/docs/code",
            },
            NavItem {
                label: "Collapsible",
                href: "/docs/collapsible",
            },
            NavItem {
                label: "Combobox",
                href: "/docs/combobox",
            },
            NavItem {
                label: "Command",
                href: "/docs/command",
            },
            NavItem {
                label: "Context Menu",
                href: "/docs/context-menu",
            },
            NavItem {
                label: "Dialog",
                href: "/docs/dialog",
            },
            NavItem {
                label: "Drawer",
                href: "/docs/drawer",
            },
            NavItem {
                label: "Dropdown Menu",
                href: "/docs/dropdown-menu",
            },
            NavItem {
                label: "Empty",
                href: "/docs/empty",
            },
            NavItem {
                label: "Field",
                href: "/docs/field",
            },
            NavItem {
                label: "Hover Card",
                href: "/docs/hover-card",
            },
            NavItem {
                label: "Input",
                href: "/docs/input",
            },
            NavItem {
                label: "Input Group",
                href: "/docs/input-group",
            },
            NavItem {
                label: "Input OTP",
                href: "/docs/input-otp",
            },
            NavItem {
                label: "Item",
                href: "/docs/item",
            },
            NavItem {
                label: "Kbd",
                href: "/docs/kbd",
            },
            NavItem {
                label: "Label",
                href: "/docs/label",
            },
            NavItem {
                label: "Menubar",
                href: "/docs/menubar",
            },
            NavItem {
                label: "Native Select",
                href: "/docs/native-select",
            },
            NavItem {
                label: "Navigation Menu",
                href: "/docs/navigation-menu",
            },
            NavItem {
                label: "Resizable",
                href: "/docs/resizable",
            },
            NavItem {
                label: "Carousel",
                href: "/docs/carousel",
            },
            NavItem {
                label: "Calendar",
                href: "/docs/calendar",
            },
            NavItem {
                label: "Date Picker",
                href: "/docs/date-picker",
            },
            NavItem {
                label: "Data Table",
                href: "/docs/data-table",
            },
            NavItem {
                label: "Chart",
                href: "/docs/chart",
            },
            NavItem {
                label: "Color Picker",
                href: "/docs/color-picker",
            },
            NavItem {
                label: "Image Cropper",
                href: "/docs/image-cropper",
            },
            NavItem {
                label: "Pagination",
                href: "/docs/pagination",
            },
            NavItem {
                label: "Popover",
                href: "/docs/popover",
            },
            NavItem {
                label: "Progress",
                href: "/docs/progress",
            },
            NavItem {
                label: "Radio Group",
                href: "/docs/radio-group",
            },
            NavItem {
                label: "Scroll Area",
                href: "/docs/scroll-area",
            },
            NavItem {
                label: "Select",
                href: "/docs/select",
            },
            NavItem {
                label: "Separator",
                href: "/docs/separator",
            },
            NavItem {
                label: "Sheet",
                href: "/docs/sheet",
            },
            NavItem {
                label: "Sidebar",
                href: "/docs/sidebar",
            },
            NavItem {
                label: "Skeleton",
                href: "/docs/skeleton",
            },
            NavItem {
                label: "Slider",
                href: "/docs/slider",
            },
            NavItem {
                label: "Spinner",
                href: "/docs/spinner",
            },
            NavItem {
                label: "Switch",
                href: "/docs/switch",
            },
            NavItem {
                label: "Table",
                href: "/docs/table",
            },
            NavItem {
                label: "Tabs",
                href: "/docs/tabs",
            },
            NavItem {
                label: "Textarea",
                href: "/docs/textarea",
            },
            NavItem {
                label: "Toast",
                href: "/docs/toast",
            },
            NavItem {
                label: "Tooltip",
                href: "/docs/tooltip",
            },
            NavItem {
                label: "Toggle",
                href: "/docs/toggle",
            },
            NavItem {
                label: "Toggle Group",
                href: "/docs/toggle-group",
            },
        ],
    },
];

#[component]
pub fn DocLayout(
    title: &'static str,
    description: &'static str,
    children: Children,
) -> impl IntoView {
    // The docs nav is the library's own sidebar, dogfooded: the collapse, the rail and the
    // stored preference are the real ones.
    let location = use_location();

    // Every component page has a blocks page beside it and the other way round, so the link
    // between them is worked out from the path rather than passed in by each of the sixty pages.
    // The pathname already carries `BASE_PATH`, so it is appended to rather than resolved again.
    let companion = Signal::derive(move || {
        let path = location.pathname.get();
        let docs = href("/docs");

        if !path.starts_with(&docs) || path == docs {
            return None;
        }

        Some(match path.strip_suffix("/blocks") {
            Some(component) => (component.to_string(), "Overview"),
            None => (format!("{path}/blocks"), "Blocks"),
        })
    });

    view! {
        <SidebarProvider class="bg-background font-sans text-foreground">
            <Sidebar>
                <SidebarHeader class="border-b h-14">
                    <A
                        href=href("/")
                        attr:class="flex h-10 items-center gap-2 px-2 text-sm font-bold tracking-tight"
                    >
                        <div class="flex size-5 items-center justify-center bg-primary text-primary-foreground">
                            <span class="text-[10px]">"NC"</span>
                        </div>
                        "nocomponents"
                    </A>
                </SidebarHeader>

                <SidebarContent>
                    {NAV
                        .iter()
                        .map(|group| {
                            let mut sorted_items = group.items.to_vec();
                            if group.label == "Components" {
                                sorted_items.sort_by_key(|item| item.label.to_lowercase());
                            }

                            view! {
                                <SidebarGroup>
                                    <SidebarGroupLabel>{group.label}</SidebarGroupLabel>
                                    <SidebarGroupContent>
                                        <SidebarMenu>
                                            {sorted_items
                                                .into_iter()
                                                .map(|item| {
                                                    let target = href(item.href);
                                                    let current = target.clone();
                                                    let is_current = Signal::derive(move || {
                                                        let path = location.pathname.get();
                                                        path == current || path == format!("{current}/blocks")
                                                    });
                                                    // A blocks page belongs to its component,
                                                    // so the row stays lit rather than the
                                                    // sidebar going blank underneath you.

                                                    view! {
                                                        <SidebarMenuItem>
                                                            <SidebarMenuButton
                                                                active=is_current
                                                                // `A` takes no node ref, so the
                                                                // state attribute goes on the
                                                                // link itself.
                                                                render=Callback::new(move |(class, _): (Signal<String>, _)| {
                                                                    view! {
                                                                        <A
                                                                            href=target.clone()
                                                                            // Or "Components"
                                                                            // reads as current
                                                                            // on every page.
                                                                            exact=true
                                                                            attr:class=class
                                                                            attr:data-active=move || {
                                                                                is_current.get().then_some("true")
                                                                            }
                                                                        >
                                                                            {item.label}
                                                                        </A>
                                                                    }
                                                                        .into_any()
                                                                })
                                                            />
                                                        </SidebarMenuItem>
                                                    }
                                                })
                                                .collect_view()}
                                        </SidebarMenu>
                                    </SidebarGroupContent>
                                </SidebarGroup>
                            }
                        })
                        .collect_view()}
                </SidebarContent>

                <SidebarRail />
            </Sidebar>

            <SidebarInset class="min-w-0">
                // `min-h`, not `h`: a long description wraps on a narrow screen, and a fixed
                // height spills it out under the border instead of growing.
                <header class="sticky top-0 z-40 flex min-h-14 shrink-0 items-center justify-between gap-3 border-b bg-background/95 px-4 py-2 backdrop-blur sm:px-6 supports-backdrop-filter:bg-background/60">
                    <div class="flex min-w-0 items-center gap-3">
                        <SidebarTrigger />
                        <div class="flex min-w-0 flex-col">
                            <h1 class="truncate text-sm leading-none font-semibold">{title}</h1>
                            // Two lines on a phone, one on anything wider: the room is there.
                            <p class="mt-1 line-clamp-2 text-xs text-muted-foreground sm:truncate">
                                {description}
                            </p>
                        </div>
                    </div>
                    <div class="flex shrink-0 items-center gap-2">
                        {move || {
                            companion
                                .get()
                                .map(|(target, label)| {
                                    view! {
                                        <A
                                            href=target
                                            attr:class="hidden h-8 items-center rounded-lg border px-2.5 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground sm:inline-flex"
                                        >
                                            {label}
                                        </A>
                                    }
                                })
                        }} <DocSearch /> <ThemeSwitcher />
                    </div>
                </header>

                <main class="mx-auto w-full max-w-4xl flex-1 px-4 py-8 sm:px-6">{children()}</main>
            </SidebarInset>
        </SidebarProvider>
    }
}
