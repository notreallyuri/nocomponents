//! Stacked collapsible sections with one shared selection.
//!
//! Each item owns a `CollapsibleContext`, so the panel machinery — mounting through the exit
//! animation, publishing its own height — is the collapsible's, not a second copy of it. What the
//! accordion adds on top is the selection (`Single` closes its sibling, `Multiple` does not) and
//! the keyboard contract: the headers are one tab stop, and the arrows walk between them.

use crate::{
    primitives::{
        collapsible::{CollapsibleContentRoot, CollapsibleContext, CollapsibleRoot},
        roving_focus::use_roving_focus,
    },
    utils::{next_id, types::Orientation},
};
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum AccordionType {
    /// Opening a section closes the one that was open.
    #[default]
    Single,
    /// Any number of sections can be open at once.
    Multiple,
}

#[derive(Copy, Clone)]
pub struct AccordionContext {
    pub value: RwSignal<Vec<String>>,
    pub kind: AccordionType,
    pub disabled: Signal<bool>,
    /// Whether a `Single` accordion lets you close the open section by clicking its header.
    pub collapsible: bool,
}

impl AccordionContext {
    pub fn is_open(&self, value: &str) -> bool {
        self.value.with(|open| open.iter().any(|v| v == value))
    }

    pub fn toggle(&self, value: &str) {
        let (kind, collapsible) = (self.kind, self.collapsible);

        self.value.update(|open| {
            let existing = open.iter().position(|v| v == value);
            match (kind, existing) {
                (AccordionType::Single, Some(_)) => {
                    if collapsible {
                        open.clear();
                    }
                }
                (AccordionType::Single, None) => {
                    open.clear();
                    open.push(value.to_string());
                }
                (AccordionType::Multiple, Some(index)) => {
                    open.remove(index);
                }
                (AccordionType::Multiple, None) => open.push(value.to_string()),
            }
        });
    }
}

pub fn use_accordion() -> AccordionContext {
    expect_context::<AccordionContext>()
}

#[component]
pub fn AccordionRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] kind: AccordionType,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Whether a `Single` accordion can be closed entirely. Ignored for `Multiple`.
    #[prop(default = true)]
    collapsible: bool,
    /// Omit for an uncontrolled accordion that owns its own selection.
    #[prop(default = None, into)]
    value: Option<RwSignal<Vec<String>>>,
    children: Children,
) -> impl IntoView {
    let value = value.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let context = AccordionContext {
        value,
        kind,
        disabled,
        collapsible,
    };

    let accordion_ref = AnyNodeRef::new();
    let roving = use_roving_focus(accordion_ref, Orientation::Vertical);

    // The headers are one tab stop; Tab reaches the accordion, the arrows walk it.
    Effect::new(move |_| {
        value.track();
        roving.sync_tab_stop();
    });

    view! {
        <Provider value=context>
            <div
                node_ref=accordion_ref
                data-slot="accordion"
                on:keydown=move |e: ev::KeyboardEvent| {
                    if roving.on_keydown(&e.key()) {
                        e.prevent_default();
                    }
                }
                on:focusin=move |_| roving.on_focus_in()
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// Which section a trigger belongs to. Provided by the item, read by the trigger.
#[derive(Clone)]
struct AccordionItem(String);

#[component]
pub fn AccordionItemRoot(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_accordion();

    let item = value.clone();
    let is_open = RwSignal::new(ctx.is_open(&item));
    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());

    // The accordion owns the selection and the collapsible owns one panel, so the group's answer
    // is mirrored into the item's signal — one direction only. The trigger toggles the *group*
    // (see `AccordionTriggerRoot`), which is what closes the sibling in a `Single` accordion.
    let mirrored = item.clone();
    Effect::new(move |_| {
        let open = ctx.is_open(&mirrored);
        if is_open.get_untracked() != open {
            is_open.set(open);
        }
    });

    let collapsible = CollapsibleContext {
        is_open,
        is_mounted: RwSignal::new(is_open.get_untracked()),
        disabled: is_disabled,
        content_id: RwSignal::new(next_id("accordion-content")),
        content_ref: AnyNodeRef::new(),
    };

    let item_state = item.clone();
    let state = move || {
        if ctx.is_open(&item_state) {
            "open"
        } else {
            "closed"
        }
    };

    view! {
        <CollapsibleRoot context=collapsible class=class>
            <Provider value=AccordionItem(item.clone())>
                <div
                    data-slot="accordion-item"
                    data-state=state
                    data-disabled=move || is_disabled.get().then_some("")
                >
                    {children()}
                </div>
            </Provider>
        </CollapsibleRoot>
    }
}

/// The clickable header of a section. Renders its own `<h3>` wrapper so the accordion is a list of
/// headings to a screen reader, not a row of loose buttons.
#[component]
pub fn AccordionTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = crate::primitives::collapsible::use_collapsible();
    let accordion = use_accordion();
    let item = expect_context::<AccordionItem>().0;
    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());

    view! {
        <h3 data-slot="accordion-header" class="flex">
            <button
                type="button"
                data-slot="accordion-trigger"
                data-roving-item=""
                tabindex="-1"
                aria-expanded=move || ctx.is_open.get().to_string()
                aria-controls=move || ctx.content_id.get()
                data-state=move || ctx.state()
                data-disabled=move || is_disabled.get().then_some("")
                disabled=is_disabled
                on:click=move |_: ev::MouseEvent| {
                    if !is_disabled.get_untracked() {
                        accordion.toggle(&item);
                    }
                }
                class=class
            >
                {children()}
            </button>
        </h3>
    }
}

#[component]
pub fn AccordionContentRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! { <CollapsibleContentRoot class=class>{children()}</CollapsibleContentRoot> }
}
