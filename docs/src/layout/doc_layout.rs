use crate::{
    app::href,
    components::{doc_search::DocSearch, page_nav::PageNav, theme_switcher::ThemeSwitcherPopover},
};
use leptos::{html::Main, prelude::*};
use leptos_router::{
    components::{A, Outlet},
    hooks::use_location,
};
use nocomponents::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader,
    SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail,
    SidebarTrigger,
};
use nocomponents::utils::types::StyledRender;

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
                label: "Blocks",
                href: "/blocks",
            },
            NavItem {
                label: "Primitives",
                href: "/primitives",
            },
            NavItem {
                label: "Installation",
                href: "/docs/installation",
            },
            NavItem {
                label: "Theme",
                href: "/docs/theme",
            },
            NavItem {
                label: "Utilities",
                href: "/docs/utility",
            },
        ],
    },
    NavGroup {
        label: "Forms",
        items: &[NavItem {
            label: "Form State",
            href: "/docs/form",
        }],
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
                label: "Canvas",
                href: "/docs/canvas",
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
                label: "Dropzone",
                href: "/docs/dropzone",
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
                label: "Floating Menu",
                href: "/docs/floating-menu",
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
                label: "Kanban",
                href: "/docs/kanban",
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
                label: "Timeline",
                href: "/docs/timeline",
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
                label: "Tree",
                href: "/docs/tree",
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
pub fn DocShell() -> impl IntoView {
    // The docs nav is the library's own sidebar, dogfooded: the collapse, the rail and the
    // stored preference are the real ones.
    //
    // It hangs off the `/docs` parent route rather than off each page, so moving between pages
    // swaps only what is inside `SidebarInset`. The sidebar element itself is never rebuilt,
    // which is what keeps its scroll position.
    let location = use_location();

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
                                                                render=Callback::new(move |styled: StyledRender| {
                                                                    view! {
                                                                        <A
                                                                            href=target.clone()
                                                                            // Or "Components"
                                                                            // reads as current
                                                                            // on every page.
                                                                            exact=true
                                                                            attr:class=styled.class
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
                <Outlet />
            </SidebarInset>
        </SidebarProvider>
    }
}

#[component]
pub fn DocLayout(
    title: &'static str,
    description: &'static str,
    children: Children,
) -> impl IntoView {
    let location = use_location();
    let content = NodeRef::<Main>::new();

    // Every component page has a blocks page beside it and the other way round, so the link
    // between them is worked out from the path rather than passed in by each of the sixty pages.
    // The pathname already carries `BASE_PATH`, so it is appended to rather than resolved again.
    // A component page offers the blocks that use it — the index filtered rather than a page of
    // its own, since a block belongs to every component in it and not just to this one.
    let companion = Signal::derive(move || {
        let path = location.pathname.get();

        let slug = NAV
            .iter()
            .find(|group| group.label == "Components")?
            .items
            .iter()
            .find(|item| href(item.href) == path)?
            .href
            .rsplit('/')
            .next()?;

        Some((format!("{}?uses={slug}", href("/blocks")), "Blocks"))
    });

    view! {
        // `min-h`, not `h`: a long description wraps on a narrow screen, and a fixed
        // height spills it out under the border instead of growing.
        <header class="sticky top-0 z-40 flex min-h-14 shrink-0 items-center justify-between gap-3 border-b bg-background/95 px-4 py-2 backdrop-blur sm:px-6 supports-backdrop-filter:bg-background/60">
            <div class="flex min-w-0 items-center gap-3">
                <SidebarTrigger />
                <div class="flex min-w-0 flex-col">
                    <h1 class="truncate text-sm leading-none font-semibold">{title}</h1>
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
                }} <DocSearch /> <ThemeSwitcherPopover />
            </div>
        </header>

        // The rail is a third column, so the content stops being what is centred in the inset and
        // starts being what is centred in the pair. Below `xl` the rail is gone and `mx-auto` on
        // the main column puts it back where it was.
        <div class="mx-auto flex w-full max-w-7xl flex-1 gap-10 px-4 sm:px-6">
            <main node_ref=content class="mx-auto w-full min-w-0 max-w-4xl flex-1 py-8">
                {children()}
            </main>
            <PageNav content=content />
        </div>
    }
}
