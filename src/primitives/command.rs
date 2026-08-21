//! A command palette: a list of items filtered by what is typed above it.
//!
//! Focus stays in the input the whole time and the highlight moves through `aria-activedescendant`
//! — which is why this cannot reuse [`crate::primitives::roving_focus`], whose whole mechanism is
//! moving DOM focus. What it borrows from it instead is the idea of reading the items back out of
//! the DOM: an item that does not match is not rendered at all, so document order *is* the order
//! the arrows walk, and nothing has to be kept in a registry to stay in step with it.
//!
//! The one thing that is registered is how many items match, because [`CommandEmptyRoot`] and a
//! group's own emptiness cannot be derived from a DOM query without a second pass.

use crate::utils::next_id;
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::{Element, HtmlElement};

/// The attribute that marks an element as a command item; the arrow keys walk these in document
/// order.
pub const COMMAND_ITEM: &str = "data-command-item";

/// Decides whether an item survives the current search. Takes `(search, haystack)`, where the
/// haystack is the item's value followed by its keywords.
pub type CommandFilter = Callback<(String, String), bool>;

/// The default match: every whitespace-separated term of the search has to appear somewhere in
/// the item's value or keywords, case-insensitively. Terms are unordered, so "new file" finds
/// "File — create new" as well as "New file".
pub fn default_filter(search: &str, haystack: &str) -> bool {
    let haystack = haystack.to_lowercase();

    search
        .split_whitespace()
        .all(|term| haystack.contains(&term.to_lowercase()))
}

#[derive(Copy, Clone)]
pub struct CommandContext {
    /// What has been typed into the input.
    pub search: RwSignal<String>,
    /// The highlighted item's `value`. Not a selection — nothing is committed until an item is
    /// activated — but it is what Enter acts on.
    pub active: RwSignal<Option<String>>,
    /// The last value an item was activated with, for a caller that would rather read a signal
    /// than pass a callback to every item.
    pub value: RwSignal<Option<String>>,

    pub input_ref: AnyNodeRef,
    /// The list, which is both the element the items are searched inside and the scroll port the
    /// highlight is kept visible in.
    pub list_ref: AnyNodeRef,

    pub input_id: RwSignal<String>,
    pub list_id: RwSignal<String>,

    /// How many items match the current search, kept by the items themselves.
    pub matches: RwSignal<usize>,

    /// Whether items filter themselves at all. Off when the caller has already narrowed the list
    /// — a search that went to a server has no business being filtered again on the way in.
    pub should_filter: bool,
    /// Replaces [`default_filter`].
    pub filter: Option<CommandFilter>,
}

impl Default for CommandContext {
    fn default() -> Self {
        Self {
            search: RwSignal::new(String::new()),
            active: RwSignal::new(None),
            value: RwSignal::new(None),
            input_ref: AnyNodeRef::new(),
            list_ref: AnyNodeRef::new(),
            input_id: RwSignal::new(String::new()),
            list_id: RwSignal::new(String::new()),
            matches: RwSignal::new(0),
            should_filter: true,
            filter: None,
        }
    }
}

pub fn use_command() -> CommandContext {
    expect_context::<CommandContext>()
}

/// A group's share of the item count, so a group with nothing left in it can take itself out of
/// the flow. Provided by [`CommandGroupRoot`]; absent for an item that is not in a group.
#[derive(Copy, Clone)]
pub struct CommandGroupContext {
    pub matches: RwSignal<usize>,
    pub label_id: RwSignal<String>,
}

pub fn use_command_group() -> Option<CommandGroupContext> {
    use_context::<CommandGroupContext>()
}

/// Keeps a highlighted item inside its scroll port. `scroll_into_view` is not usable here: it
/// scrolls every ancestor, so a palette in a dialog drags the page behind it as well.
fn reveal(list: &Element, item: &Element) {
    let port = list.get_bounding_client_rect();
    let target = item.get_bounding_client_rect();
    let scroll_top = list.scroll_top() as f64;

    // A group's padding sits between the first item and the top of the port; scrolling flush
    // against the item hides it, which reads as a clipped list.
    const PADDING: f64 = 4.0;

    if target.top() < port.top() {
        list.set_scroll_top((scroll_top + (target.top() - port.top()) - PADDING).round() as i32);
    } else if target.bottom() > port.bottom() {
        list.set_scroll_top(
            (scroll_top + (target.bottom() - port.bottom()) + PADDING).round() as i32,
        );
    }
}

impl CommandContext {
    /// Whether an item whose value and keywords read as `haystack` survives the current search.
    pub fn matches_search(&self, haystack: &str) -> bool {
        if !self.should_filter {
            return true;
        }

        let search = self.search.get();
        if search.trim().is_empty() {
            return true;
        }

        match self.filter {
            Some(filter) => filter.run((search, haystack.to_string())),
            None => default_filter(&search, haystack),
        }
    }

    /// The rendered, enabled items, in document order.
    fn items(&self) -> Vec<HtmlElement> {
        let Some(list) = self.list_ref.get_untracked() else {
            return Vec::new();
        };
        let Ok(found) = list.query_selector_all(&format!("[{COMMAND_ITEM}]:not([data-disabled])"))
        else {
            return Vec::new();
        };

        (0..found.length())
            .filter_map(|i| found.item(i))
            .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
            .collect()
    }

    fn value_of(item: &HtmlElement) -> Option<String> {
        item.get_attribute("data-value")
    }

    /// Highlights `item` and scrolls it into view.
    fn highlight(&self, item: &HtmlElement) {
        if let Some(value) = Self::value_of(item) {
            self.active.set(Some(value));
        }
        if let Some(list) = self.list_ref.get_untracked() {
            reveal(&list, item);
        }
    }

    /// Moves the highlight `offset` items, wrapping at both ends the way a palette does.
    pub fn move_active(&self, offset: isize) {
        let items = self.items();
        if items.is_empty() {
            return;
        }

        let active = self.active.get_untracked();
        let current = active.and_then(|value| {
            items
                .iter()
                .position(|item| Self::value_of(item).as_deref() == Some(value.as_str()))
        });

        let next = match current {
            Some(current) => (current as isize + offset).rem_euclid(items.len() as isize) as usize,
            // Nothing is highlighted: enter the list from the end the key came from.
            None if offset > 0 => 0,
            None => items.len() - 1,
        };

        self.highlight(&items[next]);
    }

    pub fn focus_first(&self) {
        if let Some(first) = self.items().first() {
            self.highlight(first);
        }
    }

    pub fn focus_last(&self) {
        if let Some(last) = self.items().last() {
            self.highlight(last);
        }
    }

    /// Clicks the highlighted item, so that activating it by keyboard goes through exactly the
    /// path a pointer takes.
    pub fn activate(&self) {
        let Some(active) = self.active.get_untracked() else {
            return;
        };

        if let Some(item) = self
            .items()
            .into_iter()
            .find(|item| Self::value_of(item).as_deref() == Some(active.as_str()))
        {
            item.click();
        }
    }

    /// The element id of the highlighted item, for `aria-activedescendant`.
    fn active_id(&self) -> Option<String> {
        let active = self.active.get()?;

        self.items()
            .into_iter()
            .find(|item| Self::value_of(item).as_deref() == Some(active.as_str()))
            .and_then(|item| item.get_attribute("id"))
    }

    /// Puts the highlight back on the first item whenever the one it was on has been filtered
    /// away — a palette with no highlight has nothing for Enter to do.
    fn sync_active(&self) {
        let items = self.items();

        if items.is_empty() {
            if self.active.get_untracked().is_some() {
                self.active.set(None);
            }
            return;
        }

        let still_there = self.active.get_untracked().is_some_and(|active| {
            items
                .iter()
                .any(|item| Self::value_of(item).as_deref() == Some(active.as_str()))
        });

        if !still_there {
            self.highlight(&items[0]);
        }
    }
}

/// Toggles `open` on Ctrl/Cmd + `key`, from anywhere in the document. The listener goes away with
/// whatever called this.
///
/// `key` is compared case-insensitively against the event's key, so it is written unshifted:
/// `use_command_shortcut("k", open)`.
pub fn use_command_shortcut(key: &'static str, open: RwSignal<bool>) {
    let handle = window_event_listener(ev::keydown, move |e| {
        if (e.meta_key() || e.ctrl_key()) && e.key().eq_ignore_ascii_case(key) {
            e.prevent_default();
            open.update(|open| *open = !*open);
        }
    });

    on_cleanup(move || handle.remove());
}

#[component]
pub fn CommandRoot(
    /// What has been typed, for a caller that wants to read or preset it.
    #[prop(optional, into)]
    search: RwSignal<String>,
    /// The last activated item's value.
    #[prop(optional, into)]
    value: RwSignal<Option<String>>,
    /// Off when the list arrives already filtered.
    #[prop(default = true)]
    should_filter: bool,
    /// Replaces the substring match every item is otherwise put through.
    #[prop(default = None, into)]
    filter: Option<CommandFilter>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = CommandContext {
        search,
        value,
        should_filter,
        filter,
        input_id: RwSignal::new(next_id("command-input")),
        list_id: RwSignal::new(next_id("command-list")),
        ..Default::default()
    };

    // Reads `matches` rather than `search` alone: the items update their own count from an
    // effect, so this runs once more after they have settled and the DOM is the list the arrows
    // will actually walk.
    Effect::new(move |_| {
        ctx.search.track();
        ctx.matches.track();
        ctx.sync_active();
    });

    // The keys are bound here rather than on the input so that they work wherever focus is
    // inside the palette — a footer button, a filter chip — and so a caller can put their own
    // input in without re-implementing them.
    let on_keydown = move |e: ev::KeyboardEvent| {
        match e.key().as_str() {
            "ArrowDown" => ctx.move_active(1),
            "ArrowUp" => ctx.move_active(-1),
            // Home and End belong to the caret while focus is in a textbox; that is the ARIA
            // combobox pattern, and cmdk's choice to take them is the deviation.
            "Enter" => ctx.activate(),
            _ => return,
        }

        e.prevent_default();
    };

    view! {
        <Provider value=ctx>
            <div data-slot="command" on:keydown=on_keydown class=class>
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn CommandInputRoot(
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    /// Takes focus on mount. A palette in a dialog does not need it — the dialog focuses the
    /// first thing it contains, which is this — but a standalone one does.
    #[prop(optional)]
    auto_focus: bool,
) -> impl IntoView {
    let ctx = use_command();

    // Written onto the node, not rendered: the id it has to point at belongs to an element in
    // the list, which is found by walking the DOM rather than from a signal.
    Effect::new(move |_| {
        ctx.active.track();
        ctx.matches.track();

        let Some(input) = ctx.input_ref.get() else {
            return;
        };

        match ctx.active_id() {
            Some(id) => {
                let _ = input.set_attribute("aria-activedescendant", &id);
            }
            None => {
                let _ = input.remove_attribute("aria-activedescendant");
            }
        }
    });

    if auto_focus {
        Effect::new(move |ran: Option<bool>| {
            if ran == Some(true) {
                return true;
            }
            if let Some(input) = ctx.input_ref.get()
                && let Ok(input) = input.dyn_into::<HtmlElement>()
            {
                let _ = input.focus();
                return true;
            }
            false
        });
    }

    view! {
        <input
            node_ref=ctx.input_ref
            id=move || ctx.input_id.get()
            data-slot="command-input"
            type="text"
            role="combobox"
            autocomplete="off"
            spellcheck="false"
            aria-expanded="true"
            aria-autocomplete="list"
            aria-controls=move || ctx.list_id.get()
            placeholder=move || placeholder.get()
            prop:value=move || ctx.search.get()
            on:input=move |e| ctx.search.set(event_target_value(&e))
            class=class
        />
    }
}

#[component]
pub fn CommandListRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_command();

    view! {
        <div
            node_ref=ctx.list_ref
            id=move || ctx.list_id.get()
            data-slot="command-list"
            role="listbox"
            aria-labelledby=move || ctx.input_id.get()
            class=class
        >
            {children()}
        </div>
    }
}

/// What is shown when nothing matches. Rendered only then, so it costs nothing while the list has
/// results.
#[component]
pub fn CommandEmptyRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_command();
    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.matches.get() == 0>
            <div data-slot="command-empty" role="presentation" class=class>
                {children.with_value(|children| children())}
            </div>
        </Show>
    }
}

#[component]
pub fn CommandGroupRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let group = CommandGroupContext {
        matches: RwSignal::new(0),
        label_id: RwSignal::new(String::new()),
    };

    view! {
        <Provider value=group>
            <div
                data-slot="command-group"
                role="group"
                aria-labelledby=move || { Some(group.label_id.get()).filter(|id| !id.is_empty()) }
                data-empty=move || (group.matches.get() == 0).then_some("")
                // The group has to keep rendering its children whether or not any of them match
                // — they are what reports the count back — so it is hidden rather than dropped,
                // and inline, so that an unstyled group still disappears.
                style=move || if group.matches.get() == 0 { "display: none;" } else { "" }
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn CommandGroupLabelRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let id = next_id("command-group-label");

    if let Some(group) = use_command_group() {
        group.label_id.set(id.clone());
    }

    view! {
        <div id=id data-slot="command-group-label" aria-hidden="true" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CommandItemRoot(
    /// What the item is matched and reported by. Has to be unique within the palette: it is the
    /// handle the highlight and `on_select` are carried on.
    #[prop(into)]
    value: String,
    /// Extra words the item can be found by, which are matched but not shown — "logout" on a
    /// "Sign out" item.
    #[prop(optional, into)]
    keywords: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Run when the item is activated, by click or by Enter, with the item's value.
    #[prop(default = None, into)]
    on_select: Option<Callback<String>>,
    /// Whether the item is the one currently *chosen*, which a palette has no notion of but a
    /// combobox does. Left out, the item carries no selected state at all; passed, it is
    /// announced as `aria-checked` and marked `data-selected`, both of which are separate from
    /// the highlight in `data-state`.
    #[prop(default = None, into)]
    selected: Option<Signal<bool>>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_command();
    let group = use_command_group();

    let value = StoredValue::new(value);
    let id = StoredValue::new(next_id("command-item"));
    let children = StoredValue::new(children);

    let matches = Memo::new(move |_| {
        let haystack = value.with_value(|value| {
            let keywords = keywords.get();
            if keywords.is_empty() {
                value.clone()
            } else {
                format!("{value} {keywords}")
            }
        });

        ctx.matches_search(&haystack)
    });

    let is_active = Signal::derive(move || {
        value.with_value(|value| ctx.active.with(|active| active.as_deref() == Some(value)))
    });

    // The count is a running total rather than a recount, so what an item has already put in is
    // what it has to take back out — on a search that filters it away, and on unmount.
    let counted = StoredValue::new(false);

    let adjust = move |delta: isize| {
        let apply = move |n: &mut usize| *n = n.saturating_add_signed(delta);
        ctx.matches.update(apply);
        if let Some(group) = group {
            group.matches.update(apply);
        }
    };

    Effect::new(move |_| {
        let matches = matches.get();
        if matches != counted.get_value() {
            counted.set_value(matches);
            adjust(if matches { 1 } else { -1 });
        }
    });

    on_cleanup(move || {
        if counted.get_value() {
            let apply = |n: &mut usize| *n = n.saturating_sub(1);
            let _ = ctx.matches.try_update(apply);
            if let Some(group) = group {
                let _ = group.matches.try_update(apply);
            }
        }
    });

    let select = move || {
        if disabled.get_untracked() {
            return;
        }

        let value = value.get_value();
        ctx.active.set(Some(value.clone()));
        ctx.value.set(Some(value.clone()));

        if let Some(on_select) = on_select {
            on_select.run(value);
        }
    };

    view! {
        <Show when=move || matches.get()>
            <div
                id=id.get_value()
                data-command-item=""
                data-value=value.get_value()
                data-slot="command-item"
                role="option"
                aria-selected=move || is_active.get().to_string()
                aria-checked=move || selected.map(|selected| selected.get().to_string())
                aria-disabled=move || disabled.get().then_some("true")
                data-disabled=move || disabled.get().then_some("")
                data-selected=move || { selected.and_then(|selected| selected.get().then_some("")) }
                data-state=move || if is_active.get() { "active" } else { "inactive" }
                on:click=move |_| select()
                // `pointermove`, not `pointerenter`: scrolling the list under a still pointer
                // must not move the highlight out from under the arrow keys.
                on:pointermove=move |_| {
                    if !disabled.get_untracked() && !is_active.get_untracked() {
                        ctx.active.set(Some(value.get_value()));
                    }
                }
                class=class
            >
                {children.with_value(|children| children())}
            </div>
        </Show>
    }
}

/// A rule between groups. Style-only, but it lives here so that the palette's parts are all in
/// one place.
#[component]
pub fn CommandSeparatorRoot(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! { <div data-slot="command-separator" role="separator" class=class /> }
}
