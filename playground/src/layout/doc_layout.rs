use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};
use nocomponents::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader,
    SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail,
    SidebarTrigger,
};

#[derive(Clone)]
struct NavGroup {
    label: &'static str,
    items: &'static [NavItem],
}

#[derive(Clone)]
struct NavItem {
    label: &'static str,
    href: &'static str,
}

const NAV: &[NavGroup] = &[
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
                label: "Native Select",
                href: "/docs/native-select",
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

    view! {
        <SidebarProvider class="bg-background font-sans text-foreground">
            <Sidebar>
                <SidebarHeader class="border-b h-14">
                    <A
                        href="/"
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
                                                    let is_current = Signal::derive(move || {
                                                        location.pathname.get() == item.href
                                                    });

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
                                                                            href=item.href
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
                <header class="sticky top-0 z-40 flex h-14 shrink-0 items-center justify-between gap-3 border-b bg-background/95 px-6 backdrop-blur supports-backdrop-filter:bg-background/60">
                    <div class="flex items-center gap-3">
                        <SidebarTrigger />
                        <div class="flex flex-col">
                            <h1 class="text-sm leading-none font-semibold">{title}</h1>
                            <p class="mt-1 text-xs text-muted-foreground">{description}</p>
                        </div>
                    </div>
                    <ThemeSwitcher />
                </header>

                <main class="mx-auto w-full max-w-4xl flex-1 px-6 py-8">{children()}</main>
            </SidebarInset>
        </SidebarProvider>
    }
}
