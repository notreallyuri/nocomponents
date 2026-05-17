use crate::pages::{docs, home::Home};
use leptos::prelude::*;
use leptos_router::{components::*, path};
use nocomponents::{components::toast::ToastProvider, utils::theme::ThemeProvider};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <ToastProvider>
                <Router>
                    <Routes fallback=|| "Not Found.">
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/docs/avatar") view=docs::avatar::Page />
                        <Route path=path!("/docs/badge") view=docs::badge::Page />
                        <Route path=path!("/docs/button") view=docs::button::Page />
                        <Route path=path!("/docs/card") view=docs::card::Page />
                        <Route path=path!("/docs/checkbox") view=docs::checkbox::Page />
                        <Route path=path!("/docs/dialog") view=docs::dialog::Page />
                        <Route path=path!("/docs/dropdown-menu") view=docs::dropdown_menu::Page />
                        <Route path=path!("/docs/input") view=docs::input::Page />
                        <Route path=path!("/docs/popover") view=docs::popover::Page />
                        <Route path=path!("/docs/select") view=docs::select::Page />
                        <Route path=path!("/docs/switch") view=docs::switch::Page />
                        <Route path=path!("/docs/tabs") view=docs::tabs::Page />
                        <Route path=path!("/docs/toast") view=docs::toast::Page />
                    </Routes>
                </Router>
            </ToastProvider>
        </ThemeProvider>
    }
}
