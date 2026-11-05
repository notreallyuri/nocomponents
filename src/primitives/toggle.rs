use crate::utils::controllable::controllable;
use leptos::{ev, prelude::*};

#[component]
pub fn ToggleRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Omit for an uncontrolled toggle that owns its own state.
    #[prop(default = None, into)]
    pressed: Option<RwSignal<bool>>,
    /// Whether an uncontrolled toggle starts pressed. Ignored when `pressed` is given.
    #[prop(optional)]
    default_pressed: bool,
    /// Called with the new value whenever it is pressed or released, however that came about.
    #[prop(optional, into)]
    on_pressed_change: Option<Callback<bool>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let pressed = controllable(pressed, default_pressed, on_pressed_change);

    let toggle = move |_: ev::MouseEvent| {
        if !disabled.get() {
            pressed.update(|p| *p = !*p);
        }
    };

    view! {
        <button
            type="button"
            data-slot="toggle"
            aria-pressed=move || if pressed.get() { "true" } else { "false" }
            data-state=move || if pressed.get() { "on" } else { "off" }
            disabled=disabled
            on:click=toggle
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}
