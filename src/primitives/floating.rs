use std::time::Duration;

use crate::{
    primitives::dismiss::{LayerBounds, use_dismissable_layer},
    utils::{next_id, types::TriggerRect},
};
use leptos::{
    context::Provider,
    either::Either,
    ev::MouseEvent,
    prelude::*,
    tachys::renderer::{RemoveEventHandler, dom::Dom},
    wasm_bindgen::{JsCast, JsValue},
};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, HtmlElement, KeyboardEvent};

#[derive(Copy, Clone)]
pub struct FloatingContext {
    pub is_open: RwSignal<bool>,
    pub is_mounted: RwSignal<bool>,

    // Trigger
    pub trigger_ref: AnyNodeRef,
    pub trigger_rect: RwSignal<TriggerRect>,

    // The positioned element the content renders into. Doubles as the "inside" test for outside
    // dismissal, which a portal makes impossible to derive from the DOM tree.
    pub content_ref: AnyNodeRef,

    // For controlled usage (select, combobox)
    pub value: RwSignal<Option<String>>,
    pub display_value: RwSignal<Option<String>>,

    // Accessibility
    pub trigger_id: RwSignal<String>,
    pub content_id: RwSignal<String>,
    /// Where focus lands the next time the content opens, set by whatever opened it. Read once
    /// and cleared on close, so it describes the current opening only.
    pub open_focus: RwSignal<OpenFocus>,
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
            open_focus: RwSignal::new(OpenFocus::Auto),
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

/// Where focus goes when the content opens.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum OpenFocus {
    /// Whatever the surface does by default: a menu its container, a select its current value.
    #[default]
    Auto,
    /// The first item, however the surface finds its items.
    First,
    /// The last item.
    Last,
}

/// What ArrowDown and ArrowUp do while the trigger is focused and the layer is closed.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ArrowOpen {
    /// Nothing: a popover, tooltip or hover card is not a list.
    #[default]
    None,
    /// Open it, and let the surface place focus — a select opens on its current value.
    Open,
    /// Open it and step into the list — Down onto the first item, Up onto the last, as a menu
    /// button does.
    OpenIntoList,
}

/// How a trigger and its floating content are related, for assistive tech.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum TriggerAria {
    /// Nothing announced. For content that is decorative or announced some other way.
    #[default]
    None,
    /// The trigger opens the content: `aria-haspopup` with this value, plus `aria-expanded` and,
    /// while open, `aria-controls`.
    Popup(&'static str),
    /// The content describes the trigger rather than being opened by it: `aria-describedby` while
    /// open, and no `aria-expanded`, which would be a lie.
    Describes,
}

/// What a trigger's `render` is handed.
///
/// A struct rather than a tuple because this is the one shape that has had to grow: it was
/// `(AnyNodeRef, Callback<MouseEvent>)` until a combobox inside a disabled `Field` needed the
/// resolved `disabled` as well. Every future addition is a new field, which no caller has to be
/// rewritten for — destructuring a tuple breaks the moment the tuple gains an element.
#[derive(Copy, Clone)]
pub struct TriggerRender {
    /// Must be attached to whatever is rendered, or the floating layer has nothing to measure
    /// against and the ARIA has no node to be written onto.
    pub node_ref: AnyNodeRef,
    pub on_click: Callback<MouseEvent>,
    /// The trigger's own `disabled`, already resolved against the `Field` it may sit in — which is
    /// not the same as the `disabled` the caller passed.
    pub disabled: Signal<bool>,
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
    /// What `data-slot` this trigger wears — `"select-trigger"`, `"popover-trigger"`. Named by the
    /// component that owns the trigger rather than by the caller: a slot is what a stylesheet
    /// matches on, so it is part of the component's shape and not a decoration.
    #[prop(default = None, into)]
    data_slot: Option<&'static str>,
    #[prop(default = None, into)] render: Option<Callback<TriggerRender, AnyView>>,
    children: Children,
) -> impl IntoView {
    let on_click = Callback::new(move |_: MouseEvent| context.toggle());

    // Onto the node rather than into the markup, for the reason the ARIA above is: the trigger is
    // this `<button>` in one place and whatever `render` returned in another, and the node ref is
    // all the shapes have in common. One path, so the slot cannot be there for a plain trigger and
    // missing for an `as_child` one.
    if let Some(slot) = data_slot {
        Effect::new(move |_| {
            if let Some(trigger) = context.trigger_ref.get() {
                let _ = trigger.set_attribute("data-slot", slot);
            }
        });
    }

    match render {
        Some(render_fn) => Either::Left(render_fn.run(TriggerRender {
            node_ref: context.trigger_ref,
            on_click,
            disabled,
        })),
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
    /// Whether the arrow keys open this layer from its closed trigger, and where they leave focus.
    #[prop(optional)]
    arrow_open: ArrowOpen,
    children: ChildrenFn,
) -> impl IntoView {
    let context = context.unwrap_or_default();

    if context.trigger_id.get_untracked().is_empty() {
        context.trigger_id.set(next_id("trigger"));
        context.content_id.set(next_id("content"));
    }

    // The trigger comes in three shapes — a button here, a `Button` there, anything behind
    // `render` — and the node ref is all they share, so the ARIA is written onto the node.
    Effect::new(move |_| {
        let Some(trigger) = context.trigger_ref.get() else {
            return;
        };
        let is_open = context.is_open.get();
        let minted = context.trigger_id.get();
        let existing = trigger.id();

        // An id the caller wrote is theirs, and something may already be pointing at it — a
        // `<label for>`, an `aria-labelledby` elsewhere on the page. So it is adopted as the
        // trigger id rather than overwritten, and everything downstream (the field's label, the
        // content's `aria-labelledby`) follows it. Only a trigger with no id of its own is given
        // the minted one.
        if existing.is_empty() {
            let _ = trigger.set_attribute("id", &minted);
        } else if existing != minted {
            context.trigger_id.set(existing);
        }

        match trigger_aria {
            TriggerAria::None => {}
            TriggerAria::Popup(haspopup) => {
                let _ = trigger.set_attribute("aria-haspopup", haspopup);
                let _ =
                    trigger.set_attribute("aria-expanded", if is_open { "true" } else { "false" });
                // These may only point at an element that exists, and the content is not in the
                // DOM until the layer opens.
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

    // Bound to the node for the same reason as the ARIA above. The remover lives in a
    // `StoredValue`, so replacing it or disposing the root takes the old listener off.
    let arrow_listener: StoredValue<Option<RemoveEventHandler<Element>>> = StoredValue::new(None);

    if arrow_open != ArrowOpen::None {
        Effect::new(move |_| {
            let Some(trigger) = context.trigger_ref.get() else {
                return;
            };

            let handler = Box::new(move |e: JsValue| {
                let Ok(e) = e.dyn_into::<KeyboardEvent>() else {
                    return;
                };
                // Only from the closed state; the content handles its own arrows once open.
                if context.is_open.get_untracked() {
                    return;
                }

                let focus = match (e.key().as_str(), arrow_open) {
                    ("ArrowDown", ArrowOpen::OpenIntoList) => OpenFocus::First,
                    ("ArrowUp", ArrowOpen::OpenIntoList) => OpenFocus::Last,
                    ("ArrowDown" | "ArrowUp", _) => OpenFocus::Auto,
                    _ => return,
                };

                e.prevent_default();
                context.open_focus.set(focus);
                context.open();
            });

            arrow_listener.set_value(Some(Dom::add_event_listener(&trigger, "keydown", handler)));
        });

        on_cleanup(move || arrow_listener.set_value(None));
    }

    use_dismissable_layer(
        context.is_open,
        LayerBounds::new(context.content_ref, context.trigger_ref),
        move || context.close(),
    );

    // Cancelled when the layer reopens inside the delay, and when the root is disposed: a
    // submenu goes down with its menu, and a timer firing after that touches dead signals.
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

        // Hand focus back only if it is still inside the content: a layer closed by the mouse
        // moving away must leave focus where the user put it.
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
        } else {
            // Cleared here rather than by the content, which may never have mounted to read it.
            context.open_focus.set(OpenFocus::Auto);

            if let Ok(handle) = set_timeout_with_handle(
                move || context.is_mounted.set(false),
                Duration::from_millis(unmount_delay),
            ) {
                unmount_handle.set_value(Some(handle));
            }
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
