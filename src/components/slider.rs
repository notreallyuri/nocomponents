//! The styled slider.
//!
//! Positions are the primitive's, written as inline geometry; this decides what a track, a filled
//! range and a thumb look like in both orientations.

use crate::{
    cn,
    primitives::slider::{SliderRangeRoot, SliderRoot, SliderThumbRoot, SliderTrackRoot},
    utils::types::Orientation,
};
use leptos::prelude::*;

#[component]
pub fn Slider(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    /// Omit for an uncontrolled slider. The length decides how many thumbs there are.
    #[prop(default = None, into)]
    value: Option<RwSignal<Vec<f64>>>,
    #[prop(optional)] default_value: Option<Vec<f64>>,
) -> impl IntoView {
    // One thumb unless told otherwise; a controlled slider's own length wins.
    let default_value = default_value.unwrap_or_else(|| vec![min]);
    let thumb_count = value.map_or(default_value.len(), |v| v.with_untracked(|v| v.len()));

    view! {
        <SliderRoot
            min=min
            max=max
            step=step
            orientation=orientation
            disabled=disabled
            value=value
            default_value=default_value
            class=move || {
                cn!(
                    "relative flex touch-none items-center select-none data-[orientation=horizontal]:h-4 data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-40 data-[orientation=vertical]:w-4 data-[orientation=vertical]:flex-col data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
                    class.get()
                )
            }
        >
            <SliderTrack>
                <SliderRange />
            </SliderTrack>
            {(0..thumb_count).map(|index| view! { <SliderThumb index=index /> }).collect_view()}
        </SliderRoot>
    }
}

#[component]
pub fn SliderTrack(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <SliderTrackRoot class=move || {
            cn!(
                "relative grow overflow-hidden rounded-full bg-muted data-[orientation=horizontal]:h-1.5 data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-1.5",
                class.get()
            )
        }>{children()}</SliderTrackRoot>
    }
}

#[component]
pub fn SliderRange(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <SliderRangeRoot class=move || {
            cn!(
                "absolute bg-primary data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full",
                class.get()
            )
        } />
    }
}

#[component]
pub fn SliderThumb(
    #[prop(default = 0)] index: usize,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <SliderThumbRoot
            index=index
            class=move || {
                cn!(
                    "absolute block size-4 shrink-0 rounded-full border border-primary bg-background shadow-sm transition-[color,box-shadow] outline-none hover:ring-4 hover:ring-ring/50 focus-visible:ring-4 focus-visible:ring-ring/50 data-[state=dragging]:ring-4 data-[state=dragging]:ring-ring/50 data-[disabled=true]:pointer-events-none",
                    class.get()
                )
            }
        />
    }
}
