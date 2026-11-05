use crate::{cn, primitives::separator::SeparatorRoot};
use leptos::prelude::*;

#[allow(unused_imports)]
pub use crate::utils::types::Orientation;

#[component]
pub fn Separator(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] orientation: Signal<Orientation>,
    #[prop(default = true.into(), into)] decorative: Signal<bool>,
) -> impl IntoView {
    view! {
        <SeparatorRoot
            orientation=orientation
            decorative=decorative
            class=move || {
                cn!(
                    "shrink-0 bg-border data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
                    class.get()
                )
            }
        />
    }
}
