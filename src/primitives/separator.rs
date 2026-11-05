use crate::utils::types::Orientation;
use leptos::prelude::*;

#[component]
pub fn SeparatorRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] orientation: Signal<Orientation>,
    /// Purely visual separators are hidden from assistive tech; set to `false` when the rule
    /// carries meaning (e.g. it groups unrelated sets of menu items).
    #[prop(default = true.into(), into)]
    decorative: Signal<bool>,
) -> impl IntoView {
    view! {
        <div
            data-slot="separator"
            data-orientation=move || orientation.get().as_str()
            role=move || if decorative.get() { "none" } else { "separator" }
            aria-orientation=move || (!decorative.get()).then(|| orientation.get().as_str())
            class=class
        />
    }
}
