use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use leptos_router::components::A;
use nocomponents::utils::types::StyledRender;
use nocomponents::{
    components::prelude::{
        Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Checkbox, CodeBlock, Label, Select,
        SelectContent, SelectItem, SelectPortal, SelectTrigger, SelectValue, Separator, Slider,
        Switch,
    },
    icons::x::X,
    utils::types::Language,
};

const INSTALL: &str = r#"[dependencies.nocomponents]
git = "https://github.com/notreallyuri/nocomponents"
features = ["components", "all-elements"]"#;

#[component]
fn Specimens() -> impl IntoView {
    let notifications = RwSignal::new(true);
    let remember = RwSignal::new(false);
    let fruit = RwSignal::new(Some("Banana".to_string()));
    let volume = RwSignal::new(vec![62.0]);

    view! {
        <div class="flex flex-col gap-4">
            <div class="flex flex-wrap items-center gap-x-6 gap-y-4">
                <div class="flex items-center gap-2">
                    <Switch checked=notifications id="home-notifications" />
                    <Label class="text-sm" attr:r#for="home-notifications">
                        "Notifications"
                    </Label>
                </div>

                <div class="flex items-center gap-2">
                    <Checkbox checked=remember id="home-remember" />
                    <Label class="text-sm" attr:r#for="home-remember">
                        "Remember me"
                    </Label>
                </div>

                <Select value=fruit class="w-36">
                    <SelectTrigger>
                        <SelectValue placeholder="Pick one" />
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent>
                            <SelectItem value="Apple">"Apple"</SelectItem>
                            <SelectItem value="Banana">"Banana"</SelectItem>
                            <SelectItem value="Cherry">"Cherry"</SelectItem>
                        </SelectContent>
                    </SelectPortal>
                </Select>

                <Badge>"v0.1.0"</Badge>
                <Badge variant=BadgeVariant::Secondary>"CSR"</Badge>
            </div>

            <div class="flex items-center gap-3">
                <Slider value=volume max=100.0 class="max-w-56" />
                <span class="w-8 font-mono text-xs text-muted-foreground tabular-nums">
                    {move || volume.get().first().copied().unwrap_or_default() as i32}
                </span>
            </div>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex min-h-screen flex-col bg-background font-sans text-foreground">
            <header class="sticky top-0 z-40 flex h-16 items-center justify-end px-6">
                <ThemeSwitcher />
            </header>

            <main class="flex flex-1 flex-col items-center justify-center gap-16 px-6 py-16">
                <div class="flex w-full max-w-2xl flex-col border border-border">
                    <div class="flex h-8 w-full items-center bg-border px-2">
                        <div class="flex items-center gap-2 select-none">
                            <div class="flex size-5 items-center justify-center bg-primary font-bold text-primary-foreground">
                                <span class="text-[10px]">"NC"</span>
                            </div>
                            <p class="text-sm font-medium text-muted-foreground">"nocomponents"</p>
                        </div>
                        <div
                            aria-hidden="true"
                            class="ml-auto flex size-4 items-center justify-center rounded-full bg-muted"
                        >
                            <X class="size-3 text-foreground" />
                        </div>
                    </div>

                    <div class="flex flex-col gap-6 p-6">
                        <div class="flex flex-col gap-2">
                            <h1 class="text-4xl font-bold tracking-tight">"nocomponents"</h1>
                            <p class="max-w-md text-sm text-muted-foreground">
                                "Unstyled behaviour primitives and a styled layer on top, for Leptos. Take the whole thing, or take the behaviour and bring your own classes."
                            </p>
                        </div>

                        <div class="flex flex-wrap items-center gap-2">
                            <Button
                                size=ButtonSize::Lg
                                render=Callback::new(move |styled: StyledRender| {
                                    view! {
                                        <A href=crate::app::href("/docs") attr:class=styled.class>
                                            "Browse the components"
                                        </A>
                                    }
                                        .into_any()
                                })
                            />
                            <Button
                                variant=ButtonVariant::Outline
                                size=ButtonSize::Lg
                                render=Callback::new(move |styled: StyledRender| {
                                    view! {
                                        <a
                                            href="https://github.com/notreallyuri/nocomponents"
                                            class=styled.class
                                            rel="noreferrer"
                                            target="_blank"
                                        >
                                            "Source"
                                        </a>
                                    }
                                        .into_any()
                                })
                            />
                        </div>

                        <div class="flex items-center gap-3">
                            <Separator class="flex-1" />
                            <span class="text-xs whitespace-nowrap text-muted-foreground">
                                "everything below is running, not a screenshot"
                            </span>
                            <Separator class="flex-1" />
                        </div>

                        <Specimens />

                        <CodeBlock
                            code=INSTALL
                            label="Cargo.toml"
                            language=Language::Plain
                            class="text-xs"
                        />
                    </div>
                </div>
            </main>
        </div>
    }
}
