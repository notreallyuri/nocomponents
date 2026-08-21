//! A menubar: a row of menus that behaves as one control.
//!
//! Each menu is an ordinary [`crate::primitives::dropdown_menu`] — same surface, same items, same
//! keys — and what this file adds is the bar they share. Two things make a row of dropdowns a
//! menubar: only one of them is open at a time, and once one is, the bar is *armed*, so moving the
//! pointer or the arrow keys along it swaps menus instead of opening a second.
//!
//! Which menu is open lives in one place, [`MenubarContext::open`], keyed by each menu's `value`.
//! The floating layers follow it rather than owning it, in both directions: setting the bar's
//! value opens that menu, and a menu closing on its own — Escape, an outside click, an item —
//! clears the bar.

use crate::{
    primitives::{
        dropdown_menu::MenuRoot,
        floating::{FloatingContext, FloatingRoot, OpenFocus, TriggerAria},
        roving_focus::use_roving_focus,
    },
    utils::types::{Direction, Orientation},
};
use leptos::{context::Provider, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlElement;

/// The attribute that marks a menubar trigger, so the bar can find its menus in document order
/// without a registry — the same trick the roving-focus helper uses.
pub const MENUBAR_TRIGGER: &str = "data-menubar-trigger";

#[derive(Copy, Clone)]
pub struct MenubarContext {
    /// The `value` of the menu that is open, and the whole of the bar's state: a menubar with
    /// two menus open is not a menubar.
    pub open: RwSignal<Option<String>>,
    /// The bar itself, which is both the roving-focus group and the element the triggers are
    /// looked up inside.
    pub bar_ref: AnyNodeRef,
}

impl MenubarContext {
    pub fn is_open(&self, value: &str) -> bool {
        self.open.with(|open| open.as_deref() == Some(value))
    }

    /// Whether any menu is open, which is what makes hovering a neighbour switch to it.
    pub fn is_armed(&self) -> bool {
        self.open.with(|open| open.is_some())
    }

    pub fn open(&self, value: &str) {
        self.open.set(Some(value.to_string()));
    }

    pub fn close(&self) {
        self.open.set(None);
    }

    /// The bar's triggers, in document order.
    fn triggers(&self) -> Vec<HtmlElement> {
        let Some(bar) = self.bar_ref.get_untracked() else {
            return Vec::new();
        };
        let Ok(found) = bar.query_selector_all(&format!("[{MENUBAR_TRIGGER}]:not([disabled])"))
        else {
            return Vec::new();
        };

        (0..found.length())
            .filter_map(|i| found.item(i))
            .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
            .collect()
    }

    /// Opens the menu beside the one that is open, wrapping at both ends, and puts focus on its
    /// trigger. What Left and Right do from inside an open menu.
    pub fn step(&self, direction: Direction) {
        let triggers = self.triggers();
        if triggers.is_empty() {
            return;
        }

        let current = self.open.with_untracked(|open| {
            open.as_ref().and_then(|open| {
                triggers.iter().position(|trigger| {
                    trigger.get_attribute("data-value").as_deref() == Some(open)
                })
            })
        });

        let offset = match direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        let next = match current {
            Some(current) => {
                (current as isize + offset).rem_euclid(triggers.len() as isize) as usize
            }
            None => 0,
        };

        if let Some(value) = triggers[next].get_attribute("data-value") {
            self.open.set(Some(value));
            let _ = triggers[next].focus();
        }
    }
}

pub fn use_menubar() -> MenubarContext {
    expect_context::<MenubarContext>()
}

#[component]
pub fn MenubarRoot(
    /// Which menu is open, for a caller that wants to drive the bar or watch it.
    #[prop(optional, into)]
    open: RwSignal<Option<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = MenubarContext {
        open,
        bar_ref: AnyNodeRef::new(),
    };

    // The bar is one tab stop and the arrows move along it — the same group behaviour a menu's
    // items get, turned on its side.
    let roving = use_roving_focus(ctx.bar_ref, Orientation::Horizontal);

    Effect::new(move |_| {
        if ctx.bar_ref.get().is_some() {
            roving.sync_tab_stop();
        }
    });

    view! {
        <Provider value=ctx>
            <div
                node_ref=ctx.bar_ref
                data-slot="menubar"
                role="menubar"
                aria-orientation=Orientation::Horizontal.as_str()
                on:keydown=move |e| {
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

/// One menu of the bar. Provides the floating context every dropdown part expects, so the
/// surface, the items and the submenus below it are the dropdown's, unchanged.
#[component]
pub fn MenubarMenuRoot(
    /// What this menu is called in [`MenubarContext::open`]. Unique within the bar.
    #[prop(into)]
    value: String,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let bar = use_menubar();
    let ctx = FloatingContext::default();

    let value = StoredValue::new(value);
    let children = StoredValue::new(children);

    // The bar decides, the layer follows.
    Effect::new(move |_| {
        let open = value.with_value(|value| bar.is_open(value));
        if ctx.is_open.get_untracked() != open {
            ctx.is_open.set(open);
        }
    });

    // And back: Escape, an outside click and choosing an item all close the layer directly, and
    // the bar has to stop being armed when they do.
    Effect::new(move |_| {
        if !ctx.is_open.get() && value.with_value(|value| bar.is_open(value)) {
            bar.close();
        }
    });

    view! {
        <FloatingRoot class=class context=ctx trigger_aria=TriggerAria::Popup("menu")>
            // `MenuRoot` as well, and for the same reason a dropdown publishes it: an item chosen
            // inside a submenu has to take the whole menu down with it, not just the panel it sits
            // in — and the submenu's own `FloatingRoot` has shadowed this context by then.
            <Provider value=MenuRoot(ctx)>
                <Provider value=MenubarMenuValue(
                    value,
                )>{children.with_value(|children| children())}</Provider>
            </Provider>
        </FloatingRoot>
    }
}

/// A menu's own name, so its trigger and content can talk to the bar about themselves.
#[derive(Copy, Clone)]
pub struct MenubarMenuValue(pub StoredValue<String>);

pub fn use_menubar_menu() -> MenubarMenuValue {
    expect_context::<MenubarMenuValue>()
}

#[component]
pub fn MenubarTriggerRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let bar = use_menubar();
    let ctx = expect_context::<FloatingContext>();
    let value = use_menubar_menu().0;

    let is_open = Signal::derive(move || value.with_value(|value| bar.is_open(value)));

    view! {
        <button
            type="button"
            node_ref=ctx.trigger_ref
            data-menubar-trigger=""
            data-value=value.get_value()
            data-slot="menubar-trigger"
            data-roving-item=""
            role="menuitem"
            tabindex="-1"
            aria-haspopup="menu"
            aria-expanded=move || is_open.get().to_string()
            disabled=disabled
            data-state=move || if is_open.get() { "open" } else { "closed" }
            on:click=move |_| {
                if is_open.get_untracked() {
                    bar.close();
                } else {
                    value.with_value(|value| bar.open(value));
                }
            }
            // Only once the bar is armed: on a quiet bar, running the pointer across it must not
            // start opening menus.
            on:pointerenter=move |_| {
                if bar.is_armed() && !is_open.get_untracked() {
                    value.with_value(|value| bar.open(value));
                }
            }
            on:keydown=move |e| {
                match e.key().as_str() {
                    "ArrowDown" | "ArrowUp" | "Enter" | " " => {
                        e.prevent_default();
                        ctx.open_focus
                            .set(
                                if e.key() == "ArrowUp" {
                                    OpenFocus::Last
                                } else {
                                    OpenFocus::First
                                },
                            );
                        value.with_value(|value| bar.open(value));
                    }
                    _ => {}
                }
            }
            class=class
        >
            {children()}
        </button>
    }
}
