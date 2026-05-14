use crate::pages::home::Home;
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
                    </Routes>
                </Router>
            </ToastProvider>
        </ThemeProvider>
    }
}
