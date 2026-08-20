use crate::{
    cn,
    primitives::progress::{ProgressIndicatorRoot, ProgressRoot},
};
use leptos::prelude::*;

#[component]
pub fn Progress(
    #[prop(optional, into)] value: Signal<Option<f64>>,
    #[prop(default = 100.0.into(), into)] max: Signal<f64>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <ProgressRoot
            value=value
            max=max
            class=move || {
                cn!(
                    "relative flex h-1 w-full items-center overflow-x-hidden rounded-full bg-muted",
                    class.get()
                )
            }
        >
            <ProgressIndicatorRoot
                class=move || {
                    cn!(
                        "size-full flex-1 bg-primary transition-all",
                        "data-[state=indeterminate]:animate-pulse"
                    )
                }
                style=move || {
                    let v = value.get().unwrap_or(0.0);
                    let m = max.get();
                    let pct = (v / m).clamp(0.0, 1.0) * 100.0;
                    format!("transform: translateX(-{}%);", 100.0 - pct)
                }
            />
        </ProgressRoot>
    }
}
