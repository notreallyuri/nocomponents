use std::time::Duration;

use crate::{
    primitives::dismiss::{LayerBounds, use_dismissable_layer},
    utils::{next_id, types::TriggerRect},
};
use leptos::{context::Provider, either::Either, ev::MouseEvent, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

#[derive(Copy, Clone)]
pub struct FloatingContext {
    pub is_open: RwSignal<bool>,
    pub is_mounted: RwSignal<bool>,

    // Trigger
    pub trigger_ref: AnyNodeRef,
    pub trigger_rect: RwSignal<TriggerRect>,

    // The positioned element the content renders into. Doubles as the floating-ui ref and as the
    // "inside" test for outside-pointerdown dismissal, which a portal makes impossible to derive
    // from the DOM tree.
    pub content_ref: AnyNodeRef,

    // For controlled usage (select, combobox)
    pub value: RwSignal<Option<String>>,
    pub display_value: RwSignal<Option<String>>,

    // Accessibility
    pub trigger_id: RwSignal<String>,
    pub content_id: RwSignal<String>,
}

impl Default for FloatingContext {
    fn default() -> Self {
        Self {
            is_open: RwSignal::new(false),
            is_mounted: RwSignal::new(false),
            trigger_ref: AnyNodeRef::new(),
            trigger_rect: RwSignal::new(TriggerRect::default()),
            content_ref: AnyNodeRef::new(),
            value: RwSignal::new(None),
            display_value: RwSignal::new(None),
            trigger_id: RwSignal::new(String::new()),
            content_id: RwSignal::new(String::new()),
        }
    }
}

impl FloatingContext {
    pub fn open(&self) {
        self.is_open.set(true)
    }

    pub fn close(&self) {
        self.is_open.set(false)
    }

    pub fn toggle(&self) {
        self.is_open.update(|open| *open = !*open)
    }

    pub fn is_open(&self) -> bool {
        self.is_open.get()
    }
}

/// How a trigger and its floating content are related, for assistive tech.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum TriggerAria {
    /// Nothing announced. For content that is decorative or announced some other way.
    #[default]
    None,
    /// The trigger opens the content: `aria-haspopup` with this value, plus `aria-expanded` and,
    /// while open, `aria-controls`. Menus, listboxes, popovers.
    Popup(&'static str),
    /// The content *describes* the trigger rather than being opened by it: `aria-describedby`
    /// while it is open, and no `aria-expanded`, which would be a lie. Tooltips.
    Describes,
}

/// Whether focus currently sits inside this layer's content.
fn focus_is_inside_content(context: FloatingContext) -> bool {
    let Some(active) = document().active_element() else {
        return false;
    };

    context
        .content_ref
        .get_untracked()
        .map(|content| content.contains(Some(&active)))
        .unwrap_or(false)
}

#[component]
pub fn FloatingTrigger(
    context: FloatingContext,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] render: Option<
        Callback<(AnyNodeRef, Callback<MouseEvent>), AnyView>,
    >,
    children: Children,
) -> impl IntoView {
    let on_click = Callback::new(move |_: MouseEvent| context.toggle());

    match render {
        Some(render_fn) => Either::Left(render_fn.run((context.trigger_ref, on_click))),
        None => Either::Right(view! {
            <button
                type="button"
                disabled=disabled
                class=class
                on:click=move |_| context.toggle()
                node_ref=context.trigger_ref
            >
                {children()}
            </button>
        }),
    }
}

#[component]
pub fn FloatingProvider(context: FloatingContext, children: Children) -> impl IntoView {
    provide_context(context);
    children()
}

#[component]
pub fn FloatingRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(default = 150, into)] unmount_delay: u64,
    #[prop(optional)] context: Option<FloatingContext>,
    /// How the trigger relates to the content, announced on the trigger element.
    #[prop(optional)]
    trigger_aria: TriggerAria,
    children: ChildrenFn,
) -> impl IntoView {
    let context = context.unwrap_or_default();

    if context.trigger_id.get_untracked().is_empty() {
        context.trigger_id.set(next_id("trigger"));
        context.content_id.set(next_id("content"));
    }

    // The trigger is styled and rendered by the layer above — a plain button here, a `Button`
    // there, an arbitrary element behind `render` — so the only thing every case has in common is
    // the node ref. Writing the ARIA onto that element keeps all three correct.
    Effect::new(move |_| {
        let Some(trigger) = context.trigger_ref.get() else {
            return;
        };
        let is_open = context.is_open.get();

        let _ = trigger.set_attribute("id", &context.trigger_id.get());

        match trigger_aria {
            TriggerAria::None => {}
            TriggerAria::Popup(haspopup) => {
                let _ = trigger.set_attribute("aria-haspopup", haspopup);
                let _ =
                    trigger.set_attribute("aria-expanded", if is_open { "true" } else { "false" });
                // `aria-controls` and `aria-describedby` may only point at an element that exists,
                // and the content is not in the DOM until the layer opens.
                if is_open {
                    let _ = trigger.set_attribute("aria-controls", &context.content_id.get());
                } else {
                    let _ = trigger.remove_attribute("aria-controls");
                }
            }
            TriggerAria::Describes => {
                if is_open {
                    let _ = trigger.set_attribute("aria-describedby", &context.content_id.get());
                } else {
                    let _ = trigger.remove_attribute("aria-describedby");
                }
            }
        }
    });

    use_dismissable_layer(
        context.is_open,
        LayerBounds::new(context.content_ref, context.trigger_ref),
        move || context.close(),
    );

    // The pending unmount, cancelled both when the layer reopens inside the delay and when the
    // root is disposed: a submenu is torn down together with the menu that contains it, and a
    // timer that fired after that would touch signals which no longer exist.
    let unmount_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);

    let cancel_unmount = move || {
        if let Some(handle) = unmount_handle.get_value() {
            handle.clear();
            unmount_handle.set_value(None);
        }
    };

    // `is_mounted` trails `is_open` by `unmount_delay` so content survives its exit animation.
    Effect::new(move |was_open: Option<bool>| {
        let is_open = context.is_open.get();

        // Hand focus back to the trigger, but only if it is still inside the content: a layer
        // closed by moving the mouse away, or by a click somewhere else on the page, must leave
        // focus where the user put it rather than yanking it to the trigger.
        if !is_open
            && was_open == Some(true)
            && focus_is_inside_content(context)
            && let Some(trigger) = context.trigger_ref.get_untracked()
            && let Ok(trigger) = trigger.dyn_into::<HtmlElement>()
        {
            let _ = trigger.focus();
        }

        if is_open {
            cancel_unmount();
            context.is_mounted.set(true);
        } else if let Ok(handle) = set_timeout_with_handle(
            move || context.is_mounted.set(false),
            Duration::from_millis(unmount_delay),
        ) {
            unmount_handle.set_value(Some(handle));
        }

        is_open
    });

    on_cleanup(cancel_unmount);
    view! {
        <Provider value=context>
            <div class=class>{children()}</div>
        </Provider>
    }
}
