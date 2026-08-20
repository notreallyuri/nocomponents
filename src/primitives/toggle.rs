use leptos::{ev, prelude::*};

#[component]
pub fn ToggleRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Omit for an uncontrolled toggle that owns its own state.
    #[prop(default = None, into)]
    pressed: Option<RwSignal<bool>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None)] on_change: Option<Callback<bool>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let pressed = pressed.unwrap_or_else(|| RwSignal::new(false));

    let toggle = move |_: ev::MouseEvent| {
        if !disabled.get() {
            pressed.update(|p| *p = !*p);
            if let Some(cb) = on_change {
                cb.run(pressed.get_untracked());
            }
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
