use crate::utils::types::Orientation;
use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum TabsListVariants {
    #[default]
    Default,
    Line,
}

#[derive(Copy, Clone)]
pub struct TabsContext {
    pub current_tab: RwSignal<String>,
}

impl TabsContext {
    pub fn set_tab(&self, tab: String) {
        self.current_tab.set(tab);
    }
}

pub fn use_tabs() -> TabsContext {
    expect_context::<TabsContext>()
}

#[component]
pub fn TabsRoot(
    #[prop(into)] default_value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] orientation: Orientation,
    children: Children,
) -> impl IntoView {
    let current_tab = RwSignal::new(default_value);
    let orientation_str = orientation.as_str();

    provide_context(TabsContext { current_tab });

    view! {
        <div class=class data-orientation=orientation_str data-slot="tabs">
            {children()}
        </div>
    }
}

#[component]
pub fn TabsListRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] variant: TabsListVariants,
    children: Children,
) -> impl IntoView {
    let variant_str = match variant {
        TabsListVariants::Line => "line",
        _ => "default",
    };

    view! {
        <div data-variant=variant_str role="tablist" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTriggerRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_tabs();

    let tab_value = value.clone();
    let is_active = Memo::new(move |_| ctx.current_tab.get() == tab_value);

    view! {
        <button
            type="button"
            role="tab"
            aria-selected=move || if is_active.get() { "true" } else { "false" }
            data-state=move || if is_active.get() { "active" } else { "inactive" }
            disabled=disabled
            class=class
            on:click=move |_| ctx.set_tab(value.clone())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContentRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_tabs();

    let is_active = Memo::new(move |_| ctx.current_tab.get() == value);

    view! {
        <div
            role="tabpanel"
            data-state=move || if is_active.get() { "active" } else { "inactive" }
            hidden=move || !is_active.get()
            class=class
        >
            {children()}
        </div>
    }
}
