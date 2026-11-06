use std::time::Duration;

pub use crate::utils::types::TriggerRender;
use crate::{
    primitives::dismiss::{LayerBounds, use_dismissable_layer},
    utils::{
        controllable::controllable,
        get_placement,
        modality::{last_input_was_keyboard, track_input_modality},
        next_id,
        types::{Align, Side, SideOffset, TriggerRect},
    },
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions,
};
use leptos::{
    context::Provider,
    either::Either,
    ev,
    ev::MouseEvent,
    portal::Portal,
    prelude::*,
    tachys::renderer::{RemoveEventHandler, dom::Dom},
    wasm_bindgen::{JsCast, JsValue},
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::{Element, HtmlElement, KeyboardEvent};

/// A portal that lands inside the fullscreen element when there is one, and in `<body>` otherwise.
///
/// `document.body` is the right mount for a floating layer nearly always — it is what gets the
/// layer out from under an `overflow: hidden` ancestor — and it is exactly wrong for the one case
/// a video player is: **only the fullscreen element's own subtree is rendered**, so a menu
/// portalled to the body vanishes the moment anything goes fullscreen, and reappears on the way
/// out. A layer that cannot be seen in fullscreen is a layer that cannot be used there.
///
/// Read on every `fullscreenchange` rather than once at mount, because which element that is —
/// or whether there is one at all — changes underneath a layer that is already open, and the
/// portal has to move with it.
///
/// `container` mounts the layer in an element of the caller's instead of `<body>`: a sticky or
/// fixed ancestor the layer should move with in the same frame, rather than being re-positioned
/// by script while it scrolls. It yields to the fullscreen element when the container is not
/// inside it, for the reason above.
#[component]
pub fn FloatingPortal(
    #[prop(default = None)] container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let fullscreen = RwSignal::new_local(document().fullscreen_element());

    let listener = window_event_listener(ev::fullscreenchange, move |_| {
        let current = document().fullscreen_element();
        if fullscreen.with_untracked(|held| held != &current) {
            fullscreen.set(current);
        }
    });
    on_cleanup(move || listener.remove());

    let mount = move || {
        let fullscreen = fullscreen.get();
        let container = container.and_then(|container| container.get());
        match (container, fullscreen) {
            (Some(container), Some(fullscreen)) if !fullscreen.contains(Some(&container)) => {
                Some(fullscreen)
            }
            (Some(container), _) => Some(container),
            (None, fullscreen) => fullscreen,
        }
    };

    view! {
        {move || match mount() {
            Some(mount) => {
                Either::Left(
                    // Re-created rather than re-targeted: `Portal` reads its mount once, so moving the
                    // layer between documents' worth of subtree means building it again.
                    view! {
                        <Portal mount=mount>{children.with_value(|children| children())}</Portal>
                    },
                )
            }
            None => {
                Either::Right(
                    view! { <Portal>{children.with_value(|children| children())}</Portal> },
                )
            }
        }}
    }
}

/// The classes of the positioned shell a floating layer's content sits in. In a sheet the shell
/// spans the bottom edge instead of following `floating_styles`, which the caller then leaves
/// unapplied: nothing positions a sheet, so there is nothing for `use_floating` to fight.
pub fn shell_class(sheet: bool, floating: &'static str) -> &'static str {
    if sheet {
        "fixed inset-x-0 bottom-0 z-50 w-full pointer-events-none"
    } else {
        floating
    }
}

/// The placement `side` and `align` name together, following either as it changes.
pub fn side_placement(side: Signal<Side>, align: Signal<Align>) -> Signal<Placement> {
    Signal::derive(move || get_placement(side.get(), align.get()))
}

/// The options every positioned layer hands to `use_floating`: fixed strategy, auto-update, and
/// offset/flip/shift middleware.
///
/// `placement` and `middleware` are both watched by `use_floating`, which re-positions when either
/// changes. The middleware is a derived value rather than a vector built once, so a changing
/// `side_offset` rebuilds it, and a changing `placement` moves the layer, without the root being
/// created again.
pub fn floating_options(
    placement: Signal<Placement>,
    side_offset: Signal<SideOffset>,
) -> UseFloatingOptions {
    let middleware = Signal::derive(move || {
        let middleware: MiddlewareVec = vec![
            Box::new(Offset::new(OffsetOptions::Value(side_offset.get().0))),
            Box::new(Flip::new(FlipOptions::default())),
            Box::new(Shift::new(ShiftOptions::default())),
        ];
        Some(SendWrapper::new(middleware))
    });

    UseFloatingOptions::default()
        .placement(Signal::derive(move || Some(placement.get())))
        .strategy(Strategy::Fixed)
        .while_elements_mounted_auto_update()
        .middleware(middleware)
}

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
    /// The caller's signal for whether the layer is open. When given, it replaces the context's own
    /// `is_open`, and `default_open` is ignored.
    #[prop(default = None, into)]
    open: Option<RwSignal<bool>>,
    /// Whether the layer starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the layer opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    track_input_modality();
    let mut context = context.unwrap_or_default();

    // A context a caller built and kept hold of keeps its own `is_open` unless `open` replaces it.
    let is_open = match open {
        Some(open) => controllable(Some(open), false, on_open_change),
        None => {
            if default_open {
                context.is_open.set(true);
            }
            controllable(Some(context.is_open), false, on_open_change)
        }
    };
    context.is_open = is_open;
    // Mounted from the first paint when it starts open, rather than a tick later when the effect
    // below first runs.
    if is_open.get_untracked() {
        context.is_mounted.set(true);
    }

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

        // Hand focus back only if it is still inside the content, and only when the keyboard did
        // the closing: a layer closed by the mouse must leave focus where the user put it.
        if !is_open
            && was_open == Some(true)
            && last_input_was_keyboard()
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
