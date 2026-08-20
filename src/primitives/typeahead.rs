//! Type-to-jump over a list of items.
//!
//! Typing `b`, `a` inside an open select jumps to "Banana", not to "Banana" then "Apple": the
//! keystrokes accumulate into a search string that resets after a pause. A repeated single letter
//! is the exception — pressing `b` twice cycles through the items starting with "b", which is what
//! every native list control does.
//!
//! Items come from the same `data-roving-item` marker [`crate::primitives::roving_focus`] uses, so
//! a component gets typeahead by adding one call to its keydown handler.

use leptos::{prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use std::time::Duration;
use web_sys::HtmlElement;

/// How long keystrokes keep accumulating before the search starts over.
const RESET_AFTER: Duration = Duration::from_millis(1000);

#[derive(Clone, Copy)]
pub struct Typeahead {
    search: StoredValue<String>,
    reset: StoredValue<Option<TimeoutHandle>>,
}

impl Default for Typeahead {
    fn default() -> Self {
        Self::new()
    }
}

impl Typeahead {
    pub fn new() -> Self {
        let typeahead = Self {
            search: StoredValue::new(String::new()),
            reset: StoredValue::new(None),
        };

        on_cleanup(move || {
            if let Some(handle) = typeahead.reset.get_value() {
                handle.clear();
            }
        });

        typeahead
    }

    /// Feeds one key press to the search, returning whether it was a character this consumed.
    ///
    /// `container` is the element whose `data-roving-item` descendants are searched.
    pub fn push(&self, key: &str, container: AnyNodeRef) -> bool {
        // Anything that is not a single printable character — Shift, Escape, F5 — is not typing.
        let mut chars = key.chars();
        let Some(char) = chars.next() else {
            return false;
        };
        if chars.next().is_some() || char.is_control() || char == ' ' {
            return false;
        }

        let search = self.search.get_value() + &char.to_lowercase().to_string();
        self.search.set_value(search.clone());
        self.schedule_reset();

        // "bb" means "the next item starting with b", not "an item starting with bb".
        let repeated = search.len() > 1 && search.chars().all(|c| c == char.to_ascii_lowercase());
        let needle = if repeated { &search[..1] } else { &search[..] };

        self.focus_match(needle, repeated, container);
        true
    }

    fn schedule_reset(&self) {
        if let Some(handle) = self.reset.get_value() {
            handle.clear();
        }

        let search = self.search;
        let reset = self.reset;
        if let Ok(handle) = set_timeout_with_handle(
            move || {
                search.set_value(String::new());
                reset.set_value(None);
            },
            RESET_AFTER,
        ) {
            self.reset.set_value(Some(handle));
        }
    }

    fn focus_match(&self, needle: &str, from_current: bool, container: AnyNodeRef) {
        let Some(container) = container.get_untracked() else {
            return;
        };
        let Ok(found) = container.query_selector_all("[data-roving-item]:not([data-disabled])")
        else {
            return;
        };

        let items: Vec<HtmlElement> = (0..found.length())
            .filter_map(|i| found.item(i))
            .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
            .collect();
        if items.is_empty() {
            return;
        }

        // Cycling starts looking *after* whatever is focused; a fresh search starts at the top.
        let start = match (from_current, document().active_element()) {
            (true, Some(active)) => items
                .iter()
                .position(|item| item.is_same_node(Some(&active)))
                .map(|i| i + 1)
                .unwrap_or(0),
            _ => 0,
        };

        let matched = (0..items.len())
            .map(|offset| &items[(start + offset) % items.len()])
            .find(|item| {
                item.text_content()
                    .unwrap_or_default()
                    .trim()
                    .to_lowercase()
                    .starts_with(needle)
            });

        if let Some(item) = matched {
            for other in &items {
                let _ = other.set_attribute("tabindex", "-1");
            }
            let _ = item.set_attribute("tabindex", "0");
            let _ = item.focus();
        }
    }
}
