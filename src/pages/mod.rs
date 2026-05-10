pub mod home;

pub mod router {
    use leptos::prelude::*;
    use leptos_router::{components::*, path};

    use crate::pages::home::Home;
    use ui::prelude::{Button, ButtonVariant};

    #[component]
    pub fn PublicRouter() -> impl IntoView {
        view! {
            <Router>
                <header class="w-full flex items-center px-4 h-20 bg-muted border-b border-border shadow-md">
                    <Button variant=ButtonVariant::Outline>Home</Button>

                </header>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home />
                </Routes>
            </Router>
        }
    }
}
