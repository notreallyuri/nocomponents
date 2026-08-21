use crate::pages::{docs, home::Home};
use leptos::prelude::*;
use leptos_router::{components::*, path};
use nocomponents::{components::toast::ToastProvider, utils::theme::ThemeProvider};

/// Where the site is served from, for a project page that lives under a repository name rather
/// than at the root. Set at build time and passed to Trunk's `--public-url` as well, so the
/// router and the asset URLs agree; empty for `trunk serve`.
const BASE_PATH: &str = match option_env!("BASE_PATH") {
    Some(base) => base,
    None => "",
};

/// An internal link, prefixed with [`BASE_PATH`].
///
/// The router's `base` is used for *matching* the location, not for resolving links —
/// `use_resolved_path` passes an absolute href straight through — so a bare `/docs/button` under a
/// project page navigates out of the site. Every internal href goes through here, and comparisons
/// against `location.pathname` have to as well, since that carries the base.
pub fn href(path: &str) -> String {
    format!("{BASE_PATH}{path}")
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <ToastProvider>
                <Router base=BASE_PATH>
                    <Routes fallback=|| "Not Found.">
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/docs") view=docs::index::Page />
                        <Route path=path!("/docs/avatar") view=docs::avatar::Page />
                        <Route path=path!("/docs/badge") view=docs::badge::Page />
                        <Route path=path!("/docs/button") view=docs::button::Page />
                        <Route path=path!("/docs/card") view=docs::card::Page />
                        <Route path=path!("/docs/checkbox") view=docs::checkbox::Page />
                        <Route path=path!("/docs/code") view=docs::code::Page />
                        <Route path=path!("/docs/dialog") view=docs::dialog::Page />
                        <Route path=path!("/docs/drawer") view=docs::drawer::Page />
                        <Route path=path!("/docs/dropdown-menu") view=docs::dropdown_menu::Page />
                        <Route path=path!("/docs/input") view=docs::input::Page />
                        <Route path=path!("/docs/input-group") view=docs::input_group::Page />
                        <Route path=path!("/docs/input-otp") view=docs::input_otp::Page />
                        <Route path=path!("/docs/popover") view=docs::popover::Page />
                        <Route path=path!("/docs/progress") view=docs::progress::Page />
                        <Route path=path!("/docs/scroll-area") view=docs::scroll_area::Page />
                        <Route path=path!("/docs/select") view=docs::select::Page />
                        <Route path=path!("/docs/switch") view=docs::switch::Page />
                        <Route path=path!("/docs/tabs") view=docs::tabs::Page />
                        <Route path=path!("/docs/textarea") view=docs::textarea::Page />
                        <Route path=path!("/docs/skeleton") view=docs::skeleton::Page />
                        <Route path=path!("/docs/slider") view=docs::slider::Page />
                        <Route path=path!("/docs/toast") view=docs::toast::Page />
                        <Route path=path!("/docs/alert") view=docs::alert::Page />
                        <Route path=path!("/docs/alert-dialog") view=docs::alert_dialog::Page />
                        <Route path=path!("/docs/aspect-ratio") view=docs::aspect_ratio::Page />
                        <Route path=path!("/docs/kbd") view=docs::kbd::Page />
                        <Route path=path!("/docs/label") view=docs::label::Page />
                        <Route path=path!("/docs/separator") view=docs::separator::Page />
                        <Route path=path!("/docs/spinner") view=docs::spinner::Page />
                        <Route path=path!("/docs/toggle") view=docs::toggle::Page />
                        <Route path=path!("/docs/toggle-group") view=docs::toggle_group::Page />
                        <Route path=path!("/docs/button-group") view=docs::button_group::Page />
                        <Route path=path!("/docs/radio-group") view=docs::radio_group::Page />
                        <Route path=path!("/docs/native-select") view=docs::native_select::Page />
                        <Route path=path!("/docs/table") view=docs::table::Page />
                        <Route path=path!("/docs/empty") view=docs::empty::Page />
                        <Route path=path!("/docs/field") view=docs::field::Page />
                        <Route path=path!("/docs/item") view=docs::item::Page />
                        <Route path=path!("/docs/accordion") view=docs::accordion::Page />
                        <Route path=path!("/docs/collapsible") view=docs::collapsible::Page />
                        <Route path=path!("/docs/context-menu") view=docs::context_menu::Page />
                        <Route path=path!("/docs/tooltip") view=docs::tooltip::Page />
                        <Route path=path!("/docs/sheet") view=docs::sheet::Page />
                        <Route path=path!("/docs/sidebar") view=docs::sidebar::Page />
                        <Route path=path!("/docs/hover-card") view=docs::hover_card::Page />
                        <Route path=path!("/docs/breadcrumb") view=docs::breadcrumb::Page />
                        <Route path=path!("/docs/pagination") view=docs::pagination::Page />
                    </Routes>
                </Router>
            </ToastProvider>
        </ThemeProvider>
    }
}
