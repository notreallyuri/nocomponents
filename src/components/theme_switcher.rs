use leptos::prelude::*;
use ui::utils::theme::{use_theme, Theme};
use ui::{
    cn,
    components::{
        button::ButtonVariant,
        dropdown_menu::{
            DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
            DropdownMenuSeparator, DropdownMenuTrigger,
        },
    },
    utils::types::{Align, Side},
};

#[component]
pub fn ThemeSwitcher(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let theme_ctx = use_theme();

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger class=cn!("w-18", class.get()) variant=ButtonVariant::Outline>
                {move || match theme_ctx.current.get() {
                    Theme::Light => "Light",
                    Theme::Dark => "Dark",
                    Theme::System => "System",
                }}
            </DropdownMenuTrigger>

            <DropdownMenuContent align=Align::Start side=Side::Bottom>
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
        </DropdownMenu>
    }
}
