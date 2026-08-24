use crate::{
    primitives::{
        field::field_control_for_trigger,
        floating::{ArrowOpen, FloatingContext, FloatingRoot, FloatingTrigger, TriggerAria},
        roving_focus::use_roving_focus,
        typeahead::Typeahead,
    },
    utils::types::Orientation,
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{either::Either, ev, portal::Portal, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::HtmlElement;

pub type SelectContext = FloatingContext;

pub fn use_select() -> SelectContext {
    expect_context::<SelectContext>()
}
#[component]
pub fn SelectRoot(
    #[prop(optional, into)] value: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = FloatingContext {
        value,
        ..Default::default()
    };

    view! {
        <FloatingRoot
            class=class
            context=ctx
            trigger_aria=TriggerAria::Popup("listbox")
            arrow_open=ArrowOpen::Open
        >
            {children()}
            <select class="hidden" aria-hidden="true" tabindex="-1">
                <option value=move || ctx.value.get().unwrap_or_default() selected=true></option>
            </select>
        </FloatingRoot>
    }
}

#[component]
pub fn SelectTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<
        Callback<(AnyNodeRef, Callback<ev::MouseEvent>), AnyView>,
    >,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_select();
    let field = field_control_for_trigger(ctx.trigger_id, ctx.trigger_ref, disabled);

    view! {
        <FloatingTrigger context=ctx class=class disabled=field.disabled render=as_child>
            {match children {
                Some(child) => Either::Left(child()),
                None => Either::Right(""),
            }}
        </FloatingTrigger>
    }
}

#[component]
pub fn SelectPortalRoot(children: ChildrenFn) -> impl IntoView {
    let ctx = use_select();
    let stored_children = StoredValue::new(children);

    view! {
        <Portal>
            <Show when=move || ctx.is_mounted.get()>
                {stored_children.with_value(|c| c())}
            </Show>
        </Portal>
    }
}

#[component]
pub fn SelectContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_select();
    let floating_ref = ctx.content_ref;

    let middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Value(0.0))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];

    let UseFloatingReturn {
        floating_styles,
        is_positioned,
        placement,
        ..
    } = use_floating(
        ctx.trigger_ref,
        floating_ref,
        UseFloatingOptions::default()
            .placement(Placement::BottomStart)
            .strategy(Strategy::Fixed)
            .while_elements_mounted_auto_update()
            .middleware(SendWrapper::new(middleware)),
    );

    let listbox_ref = AnyNodeRef::new();
    let roving = use_roving_focus(listbox_ref, Orientation::Vertical);
    let typeahead = Typeahead::new();

    // A select opens *on* its current value, the way a native one does: focus lands on the chosen
    // option, so the first arrow key steps away from it rather than from the top of the list.
    Effect::new(move |_| {
        if ctx.is_open.get() && is_positioned.get() {
            roving.focus_tab_stop();
        }
    });

    view! {
        <div
            node_ref=floating_ref
            style=move || {
                let width = ctx
                    .trigger_ref
                    .get()
                    .map(|el| el.get_bounding_client_rect().width())
                    .unwrap_or(0.0);
                let mut base_style = if !is_positioned.get() {
                    format!("{} visibility: hidden;", floating_styles.get())
                } else {
                    floating_styles.get().to_string()
                };
                base_style.push_str(&format!(" width: {}px;", width));
                base_style
            }
            class="fixed z-50 pointer-events-none"
        >
            <div
                node_ref=listbox_ref
                id=move || ctx.content_id.get()
                role="listbox"
                aria-labelledby=move || ctx.trigger_id.get()
                tabindex="-1"
                on:keydown=move |e| {
                    let key = e.key();
                    if roving.on_keydown(&key) {
                        e.prevent_default();
                        return;
                    }
                    match key.as_str() {
                        // An option is a div, so Enter and Space have to be turned into the click
                        // that commits it.
                        "Enter" | " " => {
                            if let Some(active) = document().active_element()
                                && active.has_attribute("data-roving-item")
                                && let Ok(active) = active.dyn_into::<HtmlElement>()
                            {
                                e.prevent_default();
                                active.click();
                            }
                        }
                        "Tab" => ctx.close(),
                        _ => {
                            if typeahead.push(&key, listbox_ref) {
                                e.prevent_default();
                            }
                        }
                    }
                }
                on:focusin=move |_| roving.on_focus_in()
                data-state=move || {
                    if ctx.is_open.get() && is_positioned.get() { "open" } else { "closed" }
                }
                data-side=move || match placement.get() {
                    Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                    _ => "bottom",
                }
                class=class
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn SelectItemRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_select();
    let item_value = value.clone();

    let is_selected = Signal::derive(move || ctx.value.get() == Some(item_value.clone()));

    let on_click = move |_: ev::MouseEvent| {
        ctx.value.set(Some(value.clone()));
        ctx.close();
    };

    view! {
        <div
            role="option"
            tabindex="-1"
            data-roving-item=""
            aria-selected=move || is_selected.get().to_string()
            on:click=on_click
            data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
            class=class
        >
            {children()}
        </div>
    }
}
