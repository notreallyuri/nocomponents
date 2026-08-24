use crate::cn;
use leptos::prelude::*;

#[component]
pub fn AspectRatio(
    #[prop(default = 1.0)] ratio: f64,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="aspect-ratio"
            style=format!("aspect-ratio: {ratio};")
            class=move || { cn!("w-full overflow-hidden *:size-full *:object-cover", class.get()) }
        >
            {children()}
        </div>
    }
}
