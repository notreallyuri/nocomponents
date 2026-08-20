use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use leptos_router::components::A;
use nocomponents::components::{button::ButtonVariant, prelude::Button};

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
    view! {
        <div class="flex min-h-screen bg-background text-foreground font-sans">
            <aside class="hidden lg:flex flex-col w-60 shrink-0 border-r bg-background sticky top-0 h-screen overflow-y-auto">
                <div class="flex h-16 bg-background sticky top-0 items-center justify-between px-6 border-b shrink-0">
                    <A
                        href="/"
                        attr:class="flex items-center gap-2 font-bold tracking-tight text-sm"
                    >
                        <div class="size-5 bg-primary text-primary-foreground flex items-center justify-center">
                            <span class="text-[10px]">"NC"</span>
                        </div>
                        "nocomponents"
                    </A>
                </div>

                <nav class="flex-1 px-3 py-4 flex flex-col gap-6">
                    {NAV
                        .iter()
                        .map(|group| {
                            let mut sorted_items = group.items.to_vec();
                            if group.label == "Components" {
                                sorted_items.sort_by_key(|item| item.label.to_lowercase());
                            }

                            view! {
                                <div class="flex flex-col gap-1">
                                    <span class="px-3 text-xs font-medium text-muted-foreground mb-1">
                                        {group.label}
                                    </span>
                                    {sorted_items
                                        .into_iter()
                                        .map(|item| {
                                            view! {
                                                <Button
                                                    class="justify-start"
                                                    variant=ButtonVariant::Ghost
                                                    render=Callback::new(move |(class, _node_ref)| {
                                                        view! {
                                                            <A href=item.href attr:class=class>
                                                                {item.label}
                                                            </A>
                                                        }
                                                            .into_any()
                                                    })
                                                />
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                        })
                        .collect_view()}
                </nav>
            </aside>

            <div class="flex-1 flex flex-col min-w-0">
                <header class="sticky top-0 z-40 h-16 flex items-center justify-between px-6 border-b bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60 shrink-0">
                    <div class="flex flex-col">
                        <h1 class="text-sm font-semibold leading-none">{title}</h1>
                        <p class="text-xs text-muted-foreground mt-1">{description}</p>
                    </div>
                    <ThemeSwitcher />
                </header>

                <main class="flex-1 px-6 py-8 max-w-4xl mx-auto w-full">{children()}</main>
            </div>
        </div>
    }
}
