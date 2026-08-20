//! Roving tabindex: arrow-key movement inside a group that is a single tab stop.
//!
//! A group of related controls — menu items, tabs, toggles, options — should take *one* Tab stop,
//! not one per item, and move focus between its items with the arrow keys. This is the shared
//! implementation behind all of them.
//!
//! Items are found in the DOM rather than kept in a registry: everything inside the container
//! marked `data-roving-item` is an item, in document order, which is also the order the arrow keys
//! follow. That keeps the helper indifferent to how a component renders its children — portalled,
//! wrapped, conditionally shown or reordered — and there is no registration to keep in sync.
//!
//! Disabled items (`disabled`, `aria-disabled="true"` or `data-disabled`) are skipped by the keys
//! and never hold the tab stop.

use crate::utils::types::Orientation;
use leptos::{prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, HtmlElement};

/// The attribute that marks an element as a roving-focus item.
pub const ROVING_ITEM: &str = "data-roving-item";

/// Where the tab stop starts out, when focus has not entered the group yet.
///
/// Selectors are tried in order, so a group with a selected item (a tabs list, a toggle group with
/// something pressed, a select with a chosen option) puts the tab stop there rather than on the
/// first item — Tab should land on what the group currently represents.
const SELECTED_ITEM: &[&str] = &[
    "[data-state=active]",
    "[data-state=checked]",
    "[data-state=on]",
    "[aria-selected=true]",
];

fn is_disabled(el: &Element) -> bool {
    el.has_attribute("disabled")
        || el.has_attribute("data-disabled")
        || el.get_attribute("aria-disabled").as_deref() == Some("true")
}

fn items(container: &Element) -> Vec<HtmlElement> {
    let Ok(found) = container.query_selector_all(&format!("[{ROVING_ITEM}]")) else {
        return Vec::new();
    };

    (0..found.length())
        .filter_map(|i| found.item(i))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .filter(|el| !is_disabled(el))
        .collect()
}

/// Arrow-key focus movement over the items inside `container`.
///
/// Attach [`RovingFocus::on_keydown`] to the container's `on:keydown` and
/// [`RovingFocus::on_focus_in`] to its `on:focusin`; mark each item with the [`ROVING_ITEM`]
/// attribute and `tabindex="-1"`.
#[derive(Clone, Copy)]
pub struct RovingFocus {
    container: AnyNodeRef,
    orientation: Signal<Orientation>,
    /// Whether moving past the last item wraps to the first.
    wrap: bool,
}

impl RovingFocus {
    pub fn new(container: AnyNodeRef, orientation: Signal<Orientation>) -> Self {
        Self {
            container,
            orientation,
            wrap: true,
        }
    }

    /// Stop at the ends instead of wrapping around.
    pub fn without_wrap(mut self) -> Self {
        self.wrap = false;
        self
    }

    fn items(&self) -> Vec<HtmlElement> {
        self.container
            .get_untracked()
            .map(|container| items(&container))
            .unwrap_or_default()
    }

    fn focused_index(&self, items: &[HtmlElement]) -> Option<usize> {
        let active = document().active_element()?;
        items
            .iter()
            .position(|item| item.is_same_node(Some(&active)))
    }

    /// Moves the tab stop onto `item` and focuses it: exactly one item in the group is reachable
    /// with Tab, and it is always the one focus last rested on.
    fn focus(&self, item: &HtmlElement) {
        for other in self.items() {
            let _ = other.set_attribute("tabindex", "-1");
        }
        let _ = item.set_attribute("tabindex", "0");
        let _ = item.focus();
    }

    pub fn focus_first(&self) {
        if let Some(first) = self.items().first() {
            self.focus(first);
        }
    }

    pub fn focus_last(&self) {
        if let Some(last) = self.items().last() {
            self.focus(last);
        }
    }

    fn focus_offset(&self, offset: isize) {
        let items = self.items();
        if items.is_empty() {
            return;
        }

        let next = match self.focused_index(&items) {
            // Focus is on the container itself rather than an item — enter from the near end.
            None if offset > 0 => 0,
            None => items.len() - 1,
            Some(current) => {
                let target = current as isize + offset;
                if self.wrap {
                    target.rem_euclid(items.len() as isize) as usize
                } else {
                    target.clamp(0, items.len() as isize - 1) as usize
                }
            }
        };

        self.focus(&items[next]);
    }

    /// Puts the tab stop on the item focus landed on, so tabbing away and back returns to it.
    pub fn on_focus_in(&self) {
        let items = self.items();
        if let Some(current) = self.focused_index(&items) {
            for (i, item) in items.iter().enumerate() {
                let _ = item.set_attribute("tabindex", if i == current { "0" } else { "-1" });
            }
        }
    }

    /// Gives the group its initial tab stop: the selected item if it has one, else the first.
    ///
    /// Call this when the group appears — items that mount later are handled by `on_focus_in`.
    pub fn sync_tab_stop(&self) {
        let items = self.items();
        if items.is_empty() || self.focused_index(&items).is_some() {
            return;
        }

        let selected = items.iter().position(|item| {
            SELECTED_ITEM
                .iter()
                .any(|selector| item.matches(selector).unwrap_or(false))
        });

        for (i, item) in items.iter().enumerate() {
            let stop = i == selected.unwrap_or(0);
            let _ = item.set_attribute("tabindex", if stop { "0" } else { "-1" });
        }
    }

    /// Focuses whichever item holds the tab stop — the selected one, when there is a selection.
    ///
    /// Lists that open *on* their current value (a select) use this; menus focus their container
    /// instead so that opening with the mouse does not paint a focus ring on the first entry.
    pub fn focus_tab_stop(&self) {
        self.sync_tab_stop();

        let items = self.items();
        let stop = items
            .iter()
            .find(|item| item.get_attribute("tabindex").as_deref() == Some("0"))
            .or_else(|| items.first());

        if let Some(item) = stop {
            let _ = item.focus();
        }
    }

    /// Handles one key press, returning whether it moved focus.
    ///
    /// The caller decides what to do with the keys this leaves alone — a menu closes a submenu on
    /// the cross-axis arrow, a select commits on Enter — and should `prevent_default` when this
    /// returns true, so arrows do not also scroll the page.
    pub fn on_keydown(&self, key: &str) -> bool {
        let vertical = self.orientation.get_untracked() == Orientation::Vertical;

        let offset = match key {
            "ArrowDown" if vertical => 1,
            "ArrowUp" if vertical => -1,
            "ArrowRight" if !vertical => 1,
            "ArrowLeft" if !vertical => -1,
            "Home" => {
                self.focus_first();
                return true;
            }
            "End" => {
                self.focus_last();
                return true;
            }
            _ => return false,
        };

        self.focus_offset(offset);
        true
    }
}

/// Creates a [`RovingFocus`] for `container`.
pub fn use_roving_focus(
    container: AnyNodeRef,
    orientation: impl Into<Signal<Orientation>>,
) -> RovingFocus {
    RovingFocus::new(container, orientation.into())
}
