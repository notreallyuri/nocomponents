use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use leptos_router::components::A;

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

const NAV: &[NavGroup] = &[NavGroup {
    label: "Components",
    items: &[
        NavItem {
            label: "Avatar",
            href: "/docs/avatar",
        },
        NavItem {
            label: "Badge",
            href: "/docs/badge",
        },
        NavItem {
            label: "Button",
            href: "/docs/button",
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
            label: "Dialog",
            href: "/docs/dialog",
        },
        NavItem {
            label: "Dropdown Menu",
            href: "/docs/dropdown-menu",
        },
        NavItem {
            label: "Input",
            href: "/docs/input",
        },
        NavItem {
            label: "Popover",
            href: "/docs/popover",
        },
        NavItem {
            label: "Select",
            href: "/docs/select",
        },
        NavItem {
            label: "Switch",
            href: "/docs/switch",
        },
        NavItem {
            label: "Tabs",
            href: "/docs/tabs",
        },
        NavItem {
            label: "Toast",
            href: "/docs/toast",
        },
    ],
}];

#[component]
pub fn DocLayout(
    title: &'static str,
    description: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex min-h-screen bg-background text-foreground font-sans">
            <aside class="hidden lg:flex flex-col w-60 shrink-0 border-r bg-background sticky top-0 h-screen overflow-y-auto">
                <div class="flex h-16 items-center justify-between px-6 border-b shrink-0">
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
                            view! {
                                <div class="flex flex-col gap-1">
                                    <span class="px-3 text-[11px] font-semibold uppercase tracking-widest text-muted-foreground mb-1">
                                        {group.label}
                                    </span>
                                    {group
                                        .items
                                        .iter()
                                        .map(|item| {
                                            view! {
                                                <A
                                                    href=item.href
                                                    attr:class="rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground hover:bg-muted transition-colors [&.active]:bg-muted [&.active]:text-foreground [&.active]:font-medium"
                                                >
                                                    {item.label}
                                                </A>
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
