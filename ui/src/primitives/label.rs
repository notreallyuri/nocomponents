use leptos::prelude::*;

#[component]
pub fn LabelRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] html_for: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <label for=html_for class=class>
            {children()}
        </label>
    }
}
