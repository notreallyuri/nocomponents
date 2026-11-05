use crate::{
    primitives::roving_focus::use_roving_focus,
    utils::{controllable::controllable, next_id, types::Orientation},
};
use leptos::{ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum TabsListVariants {
    #[default]
    Default,
    Line,
}

#[derive(Copy, Clone)]
pub struct TabsContext {
    pub current_tab: RwSignal<String>,
    /// Which arrow keys walk the tab list, and what `aria-orientation` it reports.
    pub orientation: Orientation,
    /// The prefix every tab and panel id is minted from, so a trigger and its panel can point at
    /// each other without the caller naming either.
    pub id_prefix: StoredValue<String>,
}

impl TabsContext {
    pub fn trigger_id(&self, value: &str) -> String {
        format!("{}-tab-{}", self.id_prefix.get_value(), id_part(value))
    }

    pub fn content_id(&self, value: &str) -> String {
        format!("{}-panel-{}", self.id_prefix.get_value(), id_part(value))
    }

    pub fn set_tab(&self, tab: String) {
        self.current_tab.set(tab);
    }
}

fn id_part(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub fn use_tabs() -> TabsContext {
    expect_context::<TabsContext>()
}

#[component]
pub fn TabsRoot(
    /// The selected tab, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    value: Option<RwSignal<String>>,
    /// The tab an uncontrolled root starts on. Ignored when `value` is given.
    #[prop(optional, into)]
    default_value: String,
    /// Called with the new value whenever the selected tab changes, however that came about.
    #[prop(optional, into)]
    on_value_change: Option<Callback<String>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] orientation: Orientation,
    children: Children,
) -> impl IntoView {
    let current_tab = controllable(value, default_value, on_value_change);
    let orientation_str = orientation.as_str();

    provide_context(TabsContext {
        current_tab,
        orientation,
        id_prefix: StoredValue::new(next_id("tabs")),
    });

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

    let ctx = use_tabs();
    let list_ref = AnyNodeRef::new();
    let roving = use_roving_focus(list_ref, ctx.orientation);

    // The tab stop follows the selected tab, so Tab lands on the tab that is actually showing.
    Effect::new(move |_| {
        ctx.current_tab.track();
        roving.sync_tab_stop();
    });

    view! {
        <div
            node_ref=list_ref
            data-variant=variant_str
            role="tablist"
            aria-orientation=ctx.orientation.as_str()
            on:keydown=move |e: ev::KeyboardEvent| {
                if roving.on_keydown(&e.key()) {
                    e.prevent_default();
                    // Selection follows focus: arrowing along the list shows each panel in turn,
                    // which is the behaviour a tab list is expected to have.
                    if let Some(active) = document().active_element()
                        && let Ok(active) = active.dyn_into::<HtmlElement>()
                    {
                        active.click();
                    }
                }
            }
            on:focusin=move |_| roving.on_focus_in()
            class=class
        >
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
    let id = ctx.trigger_id(&value);
    let controls = ctx.content_id(&value);

    view! {
        <button
            type="button"
            role="tab"
            id=id
            aria-controls=controls
            tabindex="-1"
            data-roving-item=""
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

    let id = ctx.content_id(&value);
    let labelled_by = ctx.trigger_id(&value);
    let is_active = Memo::new(move |_| ctx.current_tab.get() == value);

    view! {
        <div
            role="tabpanel"
            id=id
            aria-labelledby=labelled_by
            data-state=move || if is_active.get() { "active" } else { "inactive" }
            class=class
        >
            {children()}
        </div>
    }
}
