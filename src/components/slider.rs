use crate::{
    cn,
    primitives::slider::{SliderRangeRoot, SliderRoot, SliderThumbRoot, SliderTrackRoot},
    utils::{controllable::controllable, types::Orientation},
};
use leptos::prelude::*;

#[component]
pub fn Slider(
    #[prop(into, default = 0.0.into())] min: Signal<f64>,
    #[prop(into, default = 100.0.into())] max: Signal<f64>,
    #[prop(into, default = 1.0.into())] step: Signal<f64>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(optional, into)] form: Signal<String>,
    #[prop(default = None, into)] value: Option<RwSignal<Vec<f64>>>,
    #[prop(optional)] default_value: Option<Vec<f64>>,
    #[prop(optional, into)] on_value_change: Option<Callback<Vec<f64>>>,
) -> impl IntoView {
    let default_value = default_value.unwrap_or_else(|| vec![min.get_untracked()]);
    let values = controllable(value, default_value, on_value_change);
    let thumb_count = Memo::new(move |_| values.with(|values| values.len()));

    view! {
        <SliderRoot
            min=min
            max=max
            step=step
            orientation=orientation
            disabled=disabled
            value=values
            id=id
            name=name
            form=form
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
            {move || {
                (0..thumb_count.get())
                    .map(|index| view! { <SliderThumb index=index /> })
                    .collect_view()
            }}
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
