use crate::{
    cn,
    components::toggle::{TOGGLE_BASE, ToggleSize, ToggleVariant},
    primitives::toggle_group::{ToggleGroupItemRoot, ToggleGroupRoot, ToggleGroupType},
    utils::{
        controllable::forward,
        types::{AsClass, Orientation},
    },
};
use leptos::{context::Provider, prelude::*};

#[derive(Copy, Clone, Default)]
struct ToggleGroupStyle {
    variant: Signal<ToggleVariant>,
    size: Signal<ToggleSize>,
}

#[component]
pub fn ToggleGroup(
    #[prop(optional)] kind: ToggleGroupType,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] variant: Signal<ToggleVariant>,
    #[prop(optional, into)] size: Signal<ToggleSize>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] value: Option<RwSignal<Vec<String>>>,
    #[prop(optional, into)] default_value: Vec<String>,
    #[prop(optional, into)] on_value_change: Option<Callback<Vec<String>>>,
    children: Children,
) -> impl IntoView {
    view! {
        <Provider value=ToggleGroupStyle { variant, size }>
            <ToggleGroupRoot
                kind=kind
                orientation=orientation
                disabled=disabled
                value=value
                default_value=default_value
                on_value_change=forward(on_value_change)
                class=move || {
                    cn!(
                        "group/toggle-group flex w-fit items-center rounded-lg data-[orientation=vertical]:flex-col data-[orientation=vertical]:items-stretch",
                        class.get()
                    )
                }
            >
                {children()}
            </ToggleGroupRoot>
        </Provider>
    }
}

#[component]
pub fn ToggleGroupItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] variant: MaybeProp<ToggleVariant>,
    #[prop(optional, into)] size: MaybeProp<ToggleSize>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let inherited = use_context::<ToggleGroupStyle>().unwrap_or_default();
    let variant = Signal::derive(move || variant.get().unwrap_or_else(|| inherited.variant.get()));
    let size = Signal::derive(move || size.get().unwrap_or_else(|| inherited.size.get()));

    view! {
        <ToggleGroupItemRoot
            value=value
            disabled=disabled
            class=move || {
                cn!(
                    TOGGLE_BASE,
                    variant.get().as_class(),
                    size.get().as_class(),
                    "rounded-none first:rounded-l-lg last:rounded-r-lg group-data-[orientation=vertical]/toggle-group:first:rounded-t-lg group-data-[orientation=vertical]/toggle-group:last:rounded-b-lg group-data-[orientation=vertical]/toggle-group:first:rounded-r-lg group-data-[orientation=vertical]/toggle-group:last:rounded-l-lg",
                    class.get()
                )
            }
        >
            {children.map(|c| c())}
        </ToggleGroupItemRoot>
    }
}
