use crate::cn;
use leptos::prelude::*;

#[component]
pub fn AspectRatio(
    /// Width divided by height — `16.0 / 9.0` for widescreen, `1.0` for a square.
    #[prop(default = 1.0)]
    ratio: f64,
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
