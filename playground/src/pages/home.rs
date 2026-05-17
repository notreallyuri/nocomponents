use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use leptos_router::components::A;

const COMPONENTS: &[(&str, &str)] = &[
    ("Avatar", "/docs/avatar"),
    ("Badge", "/docs/badge"),
    ("Button", "/docs/button"),
    ("Card", "/docs/card"),
    ("Checkbox", "/docs/checkbox"),
    ("Dialog", "/docs/dialog"),
    ("Dropdown Menu", "/docs/dropdown-menu"),
    ("Input", "/docs/input"),
    ("Popover", "/docs/popover"),
    ("Select", "/docs/select"),
    ("Switch", "/docs/switch"),
    ("Tabs", "/docs/tabs"),
    ("Toast", "/docs/toast"),
];

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex min-h-screen flex-col bg-background text-foreground font-sans">
            <header class="sticky top-0 z-40 h-16 flex items-center justify-between px-6 border-b bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60">
                <div class="flex items-center select-none gap-2 font-bold tracking-tight text-sm">
                    <div class="size-5 bg-primary text-primary-foreground flex items-center justify-center">
                        <span class="text-[10px]">"NC"</span>
                    </div>
                    "nocomponents"
                </div>
                <ThemeSwitcher />
            </header>

            <main class="flex-1 flex flex-col items-center justify-center px-8 py-24 gap-16">
                <div class="flex flex-col items-center gap-4 text-center max-w-xl">
                    <h1 class="text-4xl font-bold tracking-tight">"nocomponents"</h1>
                    <p class="text-muted-foreground text-lg">
                        "Headless + styled Leptos UI primitives. Pick what you need."
                    </p>
                    <A
                        href="/docs/button"
                        attr:class="mt-2 inline-flex h-9 items-center justify-center rounded-lg bg-primary px-4 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
                    >
                        "Browse components →"
                    </A>
                </div>

                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2 w-full max-w-3xl">
                    {COMPONENTS
                        .iter()
                        .map(|&(label, href)| {
                            view! {
                                <A
                                    href=href
                                    attr:class="rounded-lg border bg-card px-3 py-2.5 text-sm font-medium text-card-foreground hover:bg-muted transition-colors text-center"
                                >
                                    {label}
                                </A>
                            }
                        })
                        .collect_view()}
                </div>
            </main>
        </div>
    }
}
