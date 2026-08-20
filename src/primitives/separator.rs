use crate::utils::types::Orientation;
use leptos::prelude::*;

#[component]
pub fn SeparatorRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] orientation: Orientation,
    /// Purely visual separators are hidden from assistive tech; set to `false` when the rule
    /// carries meaning (e.g. it groups unrelated sets of menu items).
    #[prop(default = true)]
    decorative: bool,
) -> impl IntoView {
    let orientation_str = orientation.as_str();

    view! {
        <div
            data-slot="separator"
            data-orientation=orientation_str
            role=if decorative { "none" } else { "separator" }
            aria-orientation=(!decorative).then_some(orientation_str)
            class=class
        />
    }
}
