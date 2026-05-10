use crate::pages::router::PublicRouter;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="">
            <PublicRouter />
        </div>
    }
}
