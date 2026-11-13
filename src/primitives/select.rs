use crate::{
    primitives::{
        field::{field_control_for_trigger, field_disabled},
        floating::{
            ArrowOpen, FloatingContext, FloatingPortal, FloatingRoot, FloatingTrigger, TriggerAria,
            TriggerRender, shell_class,
        },
        roving_focus::use_roving_focus,
        sheet_drag::{SheetFrame, use_sheet_grab},
        typeahead::Typeahead,
    },
    utils::{
        controllable::{controllable, forward},
        form::{SHADOW_STYLE, non_empty, on_form_reset},
        sheet::{
            SHEET_ROLE_STYLE, SheetScrim, close_when_presentation_changes, presentation_attr,
            use_sheet_presentation,
        },
        types::Orientation,
    },
};
use floating_ui_leptos::{
    Flip, FlipOptions, MiddlewareVec, Offset, OffsetOptions, Placement, Shift, ShiftOptions,
    Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{
    context::Provider, either::Either, ev, mount::mount_to, prelude::*, wasm_bindgen::JsCast,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use std::{collections::HashMap, time::Duration};
use web_sys::HtmlElement;

pub type SelectContext = FloatingContext;

pub fn use_select() -> SelectContext {
    expect_context::<SelectContext>()
}

/// What each option is called, by value: the text a `SelectValue` shows once one is chosen.
///
/// Held by the root rather than by the items, because the items only exist while the list is
/// mounted and the trigger has to name the choice long before — and long after — that. Entries are
/// never removed; an option that unmounts is still the label of what was picked from it.
#[derive(Copy, Clone)]
pub struct SelectLabels(RwSignal<HashMap<String, String>>);

fn use_select_labels() -> SelectLabels {
    expect_context::<SelectLabels>()
}

/// The chosen option's label, or its value while no option has said what it is called, or `None`
/// when nothing is chosen.
pub fn use_select_label() -> Signal<Option<String>> {
    let ctx = use_select();
    let labels = use_select_labels();

    Signal::derive(move || {
        ctx.value.get().map(|value| {
            labels
                .0
                .with(|labels| labels.get(&value).cloned())
                .unwrap_or(value)
        })
    })
}
/// Whether the root was disabled, for the trigger to wear. Held in a context because the trigger
/// is a descendant with a `disabled` of its own, and the two are one answer.
#[derive(Copy, Clone)]
struct SelectDisabled(Signal<bool>);

#[component]
pub fn SelectRoot(
    /// The chosen value, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    value: Option<RwSignal<Option<String>>>,
    /// The value the root starts with. Ignored when `value` is given. Declared `optional, into`
    /// rather than `default = None, into` so that `default_value="apple"` compiles: `&str` does
    /// not convert into `Option<String>`.
    #[prop(optional, into)]
    default_value: Option<String>,
    /// Called with the new value whenever the choice changes, however that came about.
    #[prop(optional, into)]
    on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] class: Signal<String>,
    /// The name the choice is submitted under, through the hidden native `<select>`. Empty leaves
    /// it out of a form's data.
    #[prop(optional, into)]
    name: Signal<String>,
    /// The browser refuses to submit the form while nothing is chosen.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// Disables the trigger, and keeps the choice out of a form's data.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// The `id` of a `<form>` the select belongs to when it is not inside one. Empty means the one
    /// around it.
    #[prop(optional, into)]
    form: Signal<String>,
    /// Whether the list is open, held by the caller. Omit it and the root keeps its own.
    #[prop(default = None, into)]
    open: Option<RwSignal<bool>>,
    /// Whether the list starts open. Ignored when `open` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called with the new value whenever the list opens or closes, however that came about.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = FloatingContext {
        value: controllable(value, default_value, on_value_change),
        ..Default::default()
    };
    let initial = ctx.value.get_untracked();

    let labels = SelectLabels(RwSignal::new(HashMap::new()));
    let disabled = field_disabled(disabled);

    let native_ref = AnyNodeRef::new();
    on_form_reset(native_ref, move || ctx.value.set(initial.clone()));

    view! {
        <Provider value=labels>
            <Provider value=SelectDisabled(disabled)>
                <FloatingRoot
                    class=class
                    context=ctx
                    open=open
                    default_open=default_open
                    on_open_change=forward(on_open_change)
                    trigger_aria=TriggerAria::Popup("listbox")
                    arrow_open=ArrowOpen::Open
                >
                    {children()}
                    <select
                        node_ref=native_ref
                        style=SHADOW_STYLE
                        aria-hidden="true"
                        tabindex="-1"
                        name=move || non_empty(name)
                        form=move || non_empty(form)
                        required=move || required.get()
                        disabled=move || disabled.get()
                    >
                        {move || {
                            let value = ctx.value.get().unwrap_or_default();
                            view! { <option value=value selected=true></option> }
                        }}
                    </select>
                </FloatingRoot>
            </Provider>
        </Provider>
    }
}

#[component]
pub fn SelectTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] as_child: Option<Callback<TriggerRender, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_select();
    let root_disabled = use_context::<SelectDisabled>()
        .map(|inner| inner.0)
        .unwrap_or_default();
    let own = Signal::derive(move || disabled.get() || root_disabled.get());
    let field = field_control_for_trigger(ctx.trigger_id, ctx.trigger_ref, own);

    view! {
        <FloatingTrigger
            context=ctx
            class=class
            disabled=field.disabled
            data_slot="select-trigger"
            render=as_child
        >
            {match children {
                Some(child) => Either::Left(child()),
                None => Either::Right(""),
            }}
        </FloatingTrigger>
    }
}

#[component]
pub fn SelectPortalRoot(
    /// Mount the layer in this element instead of `<body>` — a sticky or fixed ancestor it should
    /// move with. The fullscreen element still wins when the container is not inside it.
    #[prop(default = None)]
    container: Option<AnyNodeRef>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_select();
    let labels = use_select_labels();
    let stored_children = StoredValue::new(children);
    let preloaded = StoredValue::new(false);

    // The items are not in the DOM until the list first opens, so a value chosen before that —
    // `default_value`, or a controlled signal set from outside — would be shown as itself. Once,
    // and only when it is needed, the content is built into a detached node under a throwaway
    // context so the items can say what they are called, then thrown away. The labels outlive it.
    Effect::new(move |_| {
        let Some(value) = ctx.value.get() else {
            return;
        };

        if preloaded.get_value()
            || ctx.is_mounted.get_untracked()
            || labels
                .0
                .with_untracked(|labels| labels.contains_key(&value))
        {
            return;
        }
        preloaded.set_value(true);

        let Ok(detached) = document().create_element("div") else {
            return;
        };
        let Ok(detached) = detached.dyn_into::<HtmlElement>() else {
            return;
        };

        let handle = mount_to(detached, move || {
            provide_context(SelectContext::default());
            stored_children.with_value(|children| children())
        });

        // The items register from an effect, once their nodes exist.
        let _ = set_timeout_with_handle(move || drop(handle), Duration::ZERO);
    });

    view! {
        <FloatingPortal container=container>
            <Show when=move || ctx.is_mounted.get()>{stored_children.with_value(|c| c())}</Show>
        </FloatingPortal>
    }
}

#[component]
pub fn SelectContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    /// Below the `md` breakpoint, present the list as a bottom sheet instead of a popup under the
    /// trigger: a scrim, the list on the bottom edge, at full width, no positioning. Written as
    /// `data-presentation="sheet"` (`"floating"` otherwise). Off by default.
    #[prop(optional, into)]
    mobile_sheet: Signal<bool>,
    /// The title in the sheet's header row, beside the close button. Empty shows no title.
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
    children: Children,
) -> impl IntoView {
    let ctx = use_select();
    let floating_ref = ctx.content_ref;
    let sheet = use_sheet_presentation(mobile_sheet);
    let close = Callback::new(move |_| ctx.close());
    let as_sheet = sheet.get_untracked();
    let grab = use_sheet_grab(
        sheet,
        ctx.is_open.into(),
        sheet_snap_points,
        sheet_draggable,
        close,
    );

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
            .while_elements_mounted_auto_update_with_enabled(Signal::derive(move || !sheet.get()))
            .middleware(SendWrapper::new(middleware)),
    );

    close_when_presentation_changes(sheet, close);

    let ready = Signal::derive(move || sheet.get() || is_positioned.get());

    let listbox_ref = AnyNodeRef::new();
    let roving = use_roving_focus(listbox_ref, Orientation::Vertical);
    let typeahead = Typeahead::new();

    // A select opens *on* its current value, the way a native one does: focus lands on the chosen
    // option, so the first arrow key steps away from it rather than from the top of the list.
    Effect::new(move |_| {
        if ctx.is_open.get() && ready.get() {
            roving.focus_tab_stop();
        }
    });

    let role_element = view! {
        <div
            node_ref=listbox_ref
            data-slot="select-content"
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
                if ctx.is_open.get() && ready.get() { "open" } else { "closed" }
            }
            data-presentation=move || presentation_attr(sheet.get())
            data-side=move || {
                (!sheet.get())
                    .then(|| match placement.get() {
                        Placement::Top | Placement::TopStart | Placement::TopEnd => "top",
                        _ => "bottom",
                    })
            }
            class=move || if as_sheet { String::new() } else { class.get() }
            style=if as_sheet { SHEET_ROLE_STYLE } else { "" }
        >
                {children()}
        </div>
    };

    view! {
        <SheetScrim sheet=sheet open=ctx.is_open.into() />
        <div
            node_ref=floating_ref
            style=move || {
                if sheet.get() {
                    return String::new();
                }
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
            class=move || shell_class(sheet.get(), "fixed z-50 pointer-events-none")
        >
            {if as_sheet {
                view! {
                    <SheetFrame
                        grab=grab
                        sheet=sheet
                        open=ctx.is_open.into()
                        close=close
                        class=class
                        title=sheet_title
                        show_close=sheet_show_close
                        close_label=sheet_close_label
                        close_icon=sheet_close_icon
                    >
                        {role_element}
                    </SheetFrame>
                }
                    .into_any()
            } else {
                role_element.into_any()
            }}
        </div>
    }
}

#[component]
pub fn SelectItemRoot(
    #[prop(into)] value: String,
    /// What the trigger shows once this item is chosen. Empty means the item's own text.
    #[prop(optional, into)]
    label: Signal<String>,
    /// Skipped by the keyboard and refused by the pointer.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_select();
    let labels = use_select_labels();
    let item_value = value.clone();
    let item_ref = AnyNodeRef::new();

    let is_selected = Signal::derive(move || ctx.value.get() == Some(item_value.clone()));

    let label_value = value.clone();
    Effect::new(move |_| {
        let explicit = label.get();
        let text = if explicit.is_empty() {
            let Some(el) = item_ref.get() else {
                return;
            };
            el.text_content().unwrap_or_default().trim().to_string()
        } else {
            explicit
        };

        if text.is_empty() {
            return;
        }

        labels.0.update(|labels| {
            if labels.get(&label_value) != Some(&text) {
                labels.insert(label_value.clone(), text);
            }
        });
    });

    let on_click = move |_: ev::MouseEvent| {
        if disabled.get() {
            return;
        }
        ctx.value.set(Some(value.clone()));
        ctx.close();
    };

    view! {
        <div
            node_ref=item_ref
            data-slot="select-item"
            role="option"
            tabindex="-1"
            data-roving-item=""
            aria-selected=move || is_selected.get().to_string()
            aria-disabled=move || disabled.get().then_some("true")
            data-disabled=move || disabled.get().then_some("")
            on:click=on_click
            data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
            class=class
        >
            {children()}
        </div>
    }
}
