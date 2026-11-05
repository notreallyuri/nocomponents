use crate::cn;
use leptos::prelude::*;

#[component]
pub fn AspectRatio(
    #[prop(default = 1.0.into(), into)] ratio: Signal<f64>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="aspect-ratio"
            style=move || format!("aspect-ratio: {};", ratio.get())
            class=move || { cn!("w-full overflow-hidden *:size-full *:object-cover", class.get()) }
        >
            {children()}
        </div>
    }
}
