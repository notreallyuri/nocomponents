mod app;
mod components;
mod layout;
mod pages;

fn main() {
    leptos::mount::mount_to_body(app::App)
}
