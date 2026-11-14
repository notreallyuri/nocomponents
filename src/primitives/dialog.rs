use crate::{
    primitives::{
        dismiss::{LayerBounds, use_dismissable_layer},
        field::field_disabled,
        sheet_drag::use_sheet_grab,
    },
    utils::{
        controllable::controllable,
        modality::{last_input_was_keyboard, track_input_modality},
        next_id,
        scroll_lock::use_scroll_lock,
        sheet::{SheetHeader, presentation_attr, use_sheet_presentation},
        types::TriggerRender,
    },
};
use leptos::{
    context::Provider, either::Either, ev, portal::Portal, prelude::*, wasm_bindgen::JsCast,
};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;
use web_sys::HtmlElement;

#[derive(Copy, Clone)]
pub struct DialogContext {
    pub is_open: RwSignal<bool>,
    pub is_mounted: RwSignal<bool>,

    /// A modal dialog traps focus, announces `aria-modal` and is dismissed through its overlay;
    /// a non-modal one leaves the page usable and closes on an outside pointer.
    pub modal: bool,

    /// The panel and the button that opened it. Only a non-modal dialog needs them, but they are
    /// minted here: the dismissable layer is set up long before the content mounts.
    pub content_ref: AnyNodeRef,
    pub trigger_ref: AnyNodeRef,

    /// Ids of the title and description, empty until those parts render, so a dialog without a
    /// description does not claim to have one.
    pub title_id: RwSignal<String>,
    pub description_id: RwSignal<String>,

    /// The id the panel wears, which the trigger's `aria-controls` names.
    pub content_id: RwSignal<String>,
}

impl DialogContext {
    pub fn close(&self) {
        self.is_open.set(false);
    }
    pub fn toggle(&self) {
        self.is_open.update(|open| *open = !*open);
    }
}

pub fn use_dialog() -> DialogContext {
    expect_context::<DialogContext>()
}

#[component]
pub fn DialogRoot(
    #[prop(default = 150, into)] unmount_delay: u64,
    #[prop(default = None, into)] open: Option<RwSignal<bool>>,
    /// Whether the dialog starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the dialog opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    /// Non-modal leaves the page interactive underneath.
    #[prop(default = true)]
    modal: bool,
    children: Children,
) -> impl IntoView {
    track_input_modality();
    let is_open = controllable(open, default_open, on_open_change);
    let is_mounted = RwSignal::new(is_open.get_untracked());

    let context = DialogContext {
        modal,
        content_ref: AnyNodeRef::new(),
        trigger_ref: AnyNodeRef::new(),
        is_open,
        is_mounted,
        title_id: RwSignal::new(String::new()),
        description_id: RwSignal::new(String::new()),
        content_id: RwSignal::new(next_id("dialog-content")),
    };

    // What had focus before the dialog took it, so it can be handed back on close.
    let previously_focused = StoredValue::new_local(None::<HtmlElement>);

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

    // `is_mounted` trails `is_open` by `unmount_delay`, so the content survives its exit
    // animation; the portal gates on it rather than on `is_open`.
    Effect::new(move |was_open: Option<bool>| {
        let is_open = context.is_open.get();

        if is_open {
            previously_focused.set_value(
                document()
                    .active_element()
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok()),
            );
            cancel_unmount();
            context.is_mounted.set(true);
        } else {
            // Only on an actual close, not on the first run, and only when the keyboard closed
            // it: a pointer close leaves focus where the pointer put it.
            if was_open == Some(true)
                && last_input_was_keyboard()
                && let Some(el) = previously_focused.get_value()
            {
                let _ = el.focus();
            }
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

    // A modal layer is dismissed through its overlay; a non-modal one has none, so it needs
    // bounds to tell inside from outside.
    let bounds = if modal {
        LayerBounds::modal()
    } else {
        LayerBounds::new(context.content_ref, context.trigger_ref)
    };

    use_dismissable_layer(context.is_open, bounds, move || context.close());

    // `Provider`, not a bare `provide_context`: the portal renders under an owner of its own, so
    // a context on this owner is invisible from there and `use_dialog()` finds another dialog's.
    view! { <Provider value=context>{children()}</Provider> }
}

/// Registers an id on the context so `DialogContentRoot` can point `aria-labelledby` /
/// `aria-describedby` at this element, and returns it for the element to render.
pub fn use_dialog_label_id(part: DialogPart) -> String {
    let ctx = use_dialog();
    let id = next_id(match part {
        DialogPart::Title => "dialog-title",
        DialogPart::Description => "dialog-description",
    });

    match part {
        DialogPart::Title => ctx.title_id.set(id.clone()),
        DialogPart::Description => ctx.description_id.set(id.clone()),
    }

    id
}

/// The labelling parts of a dialog, for [`use_dialog_label_id`].
#[derive(Clone, Copy)]
pub enum DialogPart {
    Title,
    Description,
}

/// The button that opens the dialog.
///
/// Writes `aria-haspopup="dialog"`, `aria-expanded` and, while the panel is mounted,
/// `aria-controls` onto whatever node it ends up on, through `trigger_ref`: the element is this
/// `<button>` in one place and whatever `as_child` returned in another. The same node ref is what
/// the non-modal layer counts as inside, so a click on the trigger is not also an outside press.
#[component]
pub fn DialogTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// What `data-slot` the trigger wears, named by the component that owns it —
    /// `"sheet-trigger"`, `"drawer-trigger"` — for the reason `FloatingTrigger`'s is.
    #[prop(default = "dialog-trigger")]
    data_slot: &'static str,
    /// Render as something else. Handed the node ref to attach, the click to run, and the
    /// `disabled` already resolved against any `Field` around it.
    #[prop(optional, into)]
    as_child: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_dialog();
    let disabled = field_disabled(disabled);
    let on_click = Callback::new(move |_: ev::MouseEvent| ctx.toggle());

    Effect::new(move |_| {
        let Some(trigger) = ctx.trigger_ref.get() else {
            return;
        };

        let _ = trigger.set_attribute("data-slot", data_slot);
        let _ = trigger.set_attribute("aria-haspopup", "dialog");
        let _ = trigger.set_attribute(
            "aria-expanded",
            if ctx.is_open.get() { "true" } else { "false" },
        );
        let _ = trigger.set_attribute(
            "data-state",
            if ctx.is_open.get() { "open" } else { "closed" },
        );

        if ctx.is_mounted.get() {
            let _ = trigger.set_attribute("aria-controls", &ctx.content_id.get());
        } else {
            let _ = trigger.remove_attribute("aria-controls");
        }
    });

    match as_child {
        Some(render_fn) => Either::Left(render_fn.run(TriggerRender {
            node_ref: ctx.trigger_ref,
            on_click,
            disabled,
        })),
        None => Either::Right(view! {
            <button
                type="button"
                node_ref=ctx.trigger_ref
                class=class
                disabled=move || disabled.get()
                on:click=move |e| on_click.run(e)
            >
                {match children {
                    Some(child) => Either::Left(child()),
                    None => Either::Right(""),
                }}
            </button>
        }),
    }
}

#[component]
pub fn DialogPortalRoot(children: ChildrenFn) -> impl IntoView {
    let context = use_dialog();
    let stored_children = StoredValue::new(children);

    // Only a modal dialog freezes the page behind it.
    use_scroll_lock(Signal::derive(move || {
        context.modal && context.is_open.get()
    }));

    view! {
        <Portal>
            <Show when=move || {
                context.is_mounted.get()
            }>{stored_children.with_value(|c| c())}</Show>
        </Portal>
    }
}

#[component]
pub fn DialogContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Inline geometry, for a caller that transforms the panel itself: the drawer's drag offset
    /// cannot live in a class.
    #[prop(optional, into)]
    style: Signal<String>,
    /// The ARIA role of the panel: `"dialog"`, or `"alertdialog"` for one that interrupts.
    #[prop(default = "dialog", into)]
    role: &'static str,
    /// Below the `md` breakpoint, present the panel as a sheet on the bottom edge, with a grabber
    /// and, if `sheet_title` is set, a title row. Written as `data-presentation="sheet"`; the
    /// panel's own overlay is the scrim, and the styled layer does the anchoring. Off by default.
    #[prop(optional, into)]
    mobile_sheet: Signal<bool>,
    /// The title in the sheet's header row, beside the close button. Empty shows no title; it has
    /// no effect at `md` and up, where the dialog is centred and its own title is the heading.
    #[prop(optional, into)]
    sheet_title: Signal<String>,
    /// The heights the sheet rests at, as fractions of the viewport height — `vec![0.4, 0.9]` is a
    /// peek and a tall. The sheet is as tall as the highest, opens at the first, and a drag on its
    /// header settles on the nearest (a flick moves one, a release below the lowest dismisses).
    /// Written as `data-snap`, the index of the active one. Empty is a sheet as tall as its content.
    #[prop(optional, into)]
    sheet_snap_points: Signal<Vec<f64>>,
    /// Whether the sheet's header has a close button. On by default; it exists only in a sheet.
    #[prop(default = true.into(), into)]
    sheet_show_close: Signal<bool>,
    /// The close button's accessible name.
    #[prop(default = "Close".into(), into)]
    sheet_close_label: Signal<String>,
    /// What the close button draws; a plain cross when none is given. The styled layer passes its
    /// own glyph.
    #[prop(default = None, into)]
    sheet_close_icon: Option<ViewFn>,
    /// Whether dragging the sheet's header row moves and dismisses it. Off, the header is inert and
    /// leaves the browser's touch scrolling alone; the scrim, Escape and the close button remain.
    #[prop(default = true.into(), into)]
    sheet_draggable: Signal<bool>,
    /// Whether the styled layer's own close button is on. A sheet's header close follows it as
    /// well as `sheet_show_close`, so a dialog that has no close button in the corner has none in
    /// the sheet either.
    #[prop(default = true.into(), into)]
    show_close: Signal<bool>,
    /// The glyph of the styled layer's own close button. A sheet's header close draws
    /// `sheet_close_icon` if there is one and this otherwise.
    #[prop(default = None, into)]
    close_icon: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let ctx = use_dialog();
    let content_ref = ctx.content_ref;
    let sheet = use_sheet_presentation(mobile_sheet);
    let close = Callback::new(move |_| ctx.close());
    let grab = use_sheet_grab(
        sheet,
        ctx.is_open.into(),
        sheet_snap_points,
        sheet_draggable,
        close,
    );

    let focus_trap = move |e: ev::KeyboardEvent| {
        // A non-modal dialog must not hold Tab: the page underneath is still the user's.
        if !ctx.is_open.get() || !ctx.modal {
            return;
        }

        if e.key() == "Tab"
            && let Some(container) = content_ref.get()
        {
            let selector = "a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex='-1'])";

            if let Ok(node_list) = container.query_selector_all(selector) {
                let mut focusable = Vec::new();
                for i in 0..node_list.length() {
                    if let Some(node) = node_list.item(i)
                        && let Ok(el) = node.dyn_into::<HtmlElement>()
                    {
                        focusable.push(el);
                    }
                }

                if focusable.is_empty() {
                    e.prevent_default();
                    return;
                }

                let first = focusable.first().unwrap();
                let last = focusable.last().unwrap();
                let active = document().active_element();

                if e.shift_key() && active.as_ref() == Some(first.as_ref()) {
                    e.prevent_default();
                    let _ = last.focus();
                } else if active.as_ref() == Some(last.as_ref()) {
                    e.prevent_default();
                    let _ = first.focus();
                }
            }
        }
    };

    let _ = window_event_listener(ev::keydown, focus_trap);

    Effect::new(move |_| {
        if ctx.is_open.get()
            && let Some(container) = content_ref.get()
        {
            let selector = "a[href], button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex='-1'])";

            if let Ok(Some(first_node)) = container.query_selector(selector) {
                if let Ok(el) = first_node.dyn_into::<HtmlElement>() {
                    let _ = el.focus();
                }
            } else {
                let _ = container.set_attribute("tabindex", "-1");
                if let Ok(el) = container.dyn_into::<HtmlElement>() {
                    let _ = el.focus();
                }
            }
        }
    });

    view! {
        <div
            node_ref=content_ref
            id=move || ctx.content_id.get()
            class=class
            style=move || format!("{} {}", style.get(), grab.style())
            role=role
            aria-modal=move || ctx.modal.then_some("true")
            aria-labelledby=move || Some(ctx.title_id.get()).filter(|id| !id.is_empty())
            aria-describedby=move || Some(ctx.description_id.get()).filter(|id| !id.is_empty())
            data-state=move || if ctx.is_open.get() { "open" } else { "closed" }
            data-presentation=move || mobile_sheet.get().then(|| presentation_attr(sheet.get()))
            data-dragging=move || grab.dragging()
            data-snap=move || grab.snap()
        >
            <SheetHeader
                sheet=sheet
                title=sheet_title
                on_grab=grab.on_grab()
                draggable=sheet_draggable
                close=close
                show_close=Signal::derive(move || show_close.get() && sheet_show_close.get())
                close_label=sheet_close_label
                close_icon=sheet_close_icon.or(close_icon)
            />
            {children()}
        </div>
    }
}
