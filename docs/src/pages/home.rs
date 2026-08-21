use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use leptos_router::components::A;
use nocomponents::{
    components::prelude::{Button, ButtonSize, ButtonVariant},
    icons::x::X,
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex min-h-screen flex-col bg-background text-foreground font-sans">
            <header class="sticky top-0 justify-end z-40 h-16 flex items-center px-6">
                <ThemeSwitcher />
            </header>

            <main class="flex-1 flex flex-col items-center justify-center px-8 py-24 gap-16">
                <div class="flex border flex-col items-center text-center max-w-xl">
                    <div class="flex items-center bg-border h-8 w-full px-2">
                        <div class="flex items-center select-none gap-2">
                            <div class="size-5 bg-primary font-bold text-primary-foreground flex items-center justify-center">
                                <span class="text-[10px]">"NC"</span>
                            </div>
                            <p class="text-muted-foreground font-medium text-sm">"nocomponents"</p>
                        </div>
                        <div class="rounded-full flex items-center ml-auto justify-center bg-muted size-4">
                            <X class="text-foreground size-3" />
                        </div>
                    </div>
                    <div class="p-4 space-y-1">
                        <h1 class="text-4xl font-bold tracking-tight">"nocomponents"</h1>
                        <p class="text-muted-foreground text-sm">
                            "Headless + styled Leptos UI primitives. Pick what you need."
                        </p>
                        <Button
                            variant=ButtonVariant::Outline
                            size=ButtonSize::Lg
                            class="mt-4"
                            render=Callback::new(move |(class, _node_ref)| {
                                view! {
                                    <A href=crate::app::href("/docs") attr:class=class>
                                        "Go to docs"
                                    </A>
                                }
                                    .into_any()
                            })
                        />
                    </div>
                </div>
            </main>
        </div>
    }
}
