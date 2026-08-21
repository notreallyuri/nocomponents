use leptos::prelude::*;
use nocomponents::{
    cn,
    components::{
        button::ButtonVariant,
        dropdown_menu::{
            DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
            DropdownMenuPortal, DropdownMenuSeparator, DropdownMenuTrigger,
        },
    },
    utils::{
        theme::{Theme, use_theme},
        types::{Align, Side},
    },
};

#[component]
pub fn ThemeSwitcher(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let theme_ctx = use_theme();

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger
                class=move || cn!("w-18", class.get())
                variant=ButtonVariant::Outline
            >
                {move || match theme_ctx.current.get() {
                    Theme::Light => "Light",
                    Theme::Dark => "Dark",
                    Theme::System => "System",
                }}
            </DropdownMenuTrigger>

            <DropdownMenuPortal>
                <DropdownMenuContent align=Align::End side=Side::Bottom>
                    <DropdownMenuLabel>"Theme"</DropdownMenuLabel>
                    <DropdownMenuSeparator />

                    <DropdownMenuItem on_click=move |_| {
                        theme_ctx.set_theme(Theme::Light)
                    }>"Light"</DropdownMenuItem>

                    <DropdownMenuItem on_click=move |_| {
                        theme_ctx.set_theme(Theme::Dark)
                    }>"Dark"</DropdownMenuItem>

                    <DropdownMenuItem on_click=move |_| {
                        theme_ctx.set_theme(Theme::System)
                    }>"System"</DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenuPortal>
        </DropdownMenu>
    }
}
