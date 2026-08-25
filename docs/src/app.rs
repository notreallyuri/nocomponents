use crate::{
    layout::doc_layout::DocShell,
    pages::{docs, home::Home},
};
use leptos::prelude::*;
use leptos_router::{components::*, path};
use nocomponents::{components::toast::ToastProvider, utils::theme::ThemeProvider};

const BASE_PATH: &str = match option_env!("BASE_PATH") {
    Some(base) => base,
    None => "",
};

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
                        // Every docs page hangs off one shell, so navigating swaps only the
                        // `Outlet` and leaves the sidebar — and its scroll position — mounted.
                        <ParentRoute path=path!("/docs") view=DocShell>
                            <Route path=path!("") view=docs::index::Page />
                            <Route path=path!("/:component/blocks") view=docs::blocks::Page />
                            <Route path=path!("/avatar") view=docs::avatar::Page />
                            <Route path=path!("/badge") view=docs::badge::Page />
                            <Route path=path!("/button") view=docs::button::Page />
                            <Route path=path!("/card") view=docs::card::Page />
                            <Route path=path!("/checkbox") view=docs::checkbox::Page />
                            <Route path=path!("/code") view=docs::code::Page />
                            <Route path=path!("/dialog") view=docs::dialog::Page />
                            <Route path=path!("/drawer") view=docs::drawer::Page />
                            <Route path=path!("/dropzone") view=docs::dropzone::Page />
                            <Route path=path!("/dropdown-menu") view=docs::dropdown_menu::Page />
                            <Route path=path!("/input") view=docs::input::Page />
                            <Route path=path!("/installation") view=docs::installation::Page />
                            <Route path=path!("/input-group") view=docs::input_group::Page />
                            <Route path=path!("/input-otp") view=docs::input_otp::Page />
                            <Route path=path!("/popover") view=docs::popover::Page />
                            <Route path=path!("/progress") view=docs::progress::Page />
                            <Route path=path!("/scroll-area") view=docs::scroll_area::Page />
                            <Route path=path!("/select") view=docs::select::Page />
                            <Route path=path!("/switch") view=docs::switch::Page />
                            <Route path=path!("/tabs") view=docs::tabs::Page />
                            <Route path=path!("/textarea") view=docs::textarea::Page />
                            <Route path=path!("/skeleton") view=docs::skeleton::Page />
                            <Route path=path!("/slider") view=docs::slider::Page />
                            <Route path=path!("/toast") view=docs::toast::Page />
                            <Route path=path!("/alert") view=docs::alert::Page />
                            <Route path=path!("/alert-dialog") view=docs::alert_dialog::Page />
                            <Route path=path!("/aspect-ratio") view=docs::aspect_ratio::Page />
                            <Route path=path!("/kanban") view=docs::kanban::Page />
                            <Route path=path!("/kbd") view=docs::kbd::Page />
                            <Route path=path!("/label") view=docs::label::Page />
                            <Route path=path!("/separator") view=docs::separator::Page />
                            <Route path=path!("/spinner") view=docs::spinner::Page />
                            <Route path=path!("/toggle") view=docs::toggle::Page />
                            <Route path=path!("/tree") view=docs::tree::Page />
                            <Route path=path!("/toggle-group") view=docs::toggle_group::Page />
                            <Route path=path!("/button-group") view=docs::button_group::Page />
                            <Route path=path!("/radio-group") view=docs::radio_group::Page />
                            <Route path=path!("/native-select") view=docs::native_select::Page />
                            <Route path=path!("/table") view=docs::table::Page />
                            <Route path=path!("/empty") view=docs::empty::Page />
                            <Route path=path!("/field") view=docs::field::Page />
                            <Route path=path!("/floating-menu") view=docs::floating_menu::Page />
                            <Route path=path!("/form") view=docs::form::Page />
                            <Route path=path!("/theme") view=docs::theme::Page />
                            <Route path=path!("/utility") view=docs::utility::Page />
                            <Route path=path!("/item") view=docs::item::Page />
                            <Route path=path!("/accordion") view=docs::accordion::Page />
                            <Route path=path!("/collapsible") view=docs::collapsible::Page />
                            <Route path=path!("/combobox") view=docs::combobox::Page />
                            <Route path=path!("/command") view=docs::command::Page />
                            <Route path=path!("/context-menu") view=docs::context_menu::Page />
                            <Route path=path!("/tooltip") view=docs::tooltip::Page />
                            <Route path=path!("/sheet") view=docs::sheet::Page />
                            <Route path=path!("/sidebar") view=docs::sidebar::Page />
                            <Route path=path!("/hover-card") view=docs::hover_card::Page />
                            <Route path=path!("/breadcrumb") view=docs::breadcrumb::Page />
                            <Route path=path!("/pagination") view=docs::pagination::Page />
                            <Route path=path!("/image-cropper") view=docs::image_cropper::Page />
                            <Route path=path!("/color-picker") view=docs::color_picker::Page />
                            <Route path=path!("/chart") view=docs::chart::Page />
                            <Route path=path!("/data-table") view=docs::data_table::Page />
                            <Route path=path!("/date-picker") view=docs::date_picker::Page />
                            <Route path=path!("/calendar") view=docs::calendar::Page />
                            <Route path=path!("/carousel") view=docs::carousel::Page />
                            <Route path=path!("/resizable") view=docs::resizable::Page />
                            <Route
                                path=path!("/navigation-menu")
                                view=docs::navigation_menu::Page
                            />
                            <Route path=path!("/menubar") view=docs::menubar::Page />
                        </ParentRoute>
                    </Routes>
                </Router>
            </ToastProvider>
        </ThemeProvider>
    }
}
