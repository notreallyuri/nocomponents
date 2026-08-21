//! The two pieces of state a table grows the moment it holds real data: which column it is
//! sorted by, and which rows are picked.
//!
//! Both are plain handles rather than context providers. A table is not a tree of parts that have
//! to find each other — the caller already has the rows and the columns in one place — so passing
//! a `TableSort` where it is needed is clearer than hiding it in a context and hoping the right
//! component is inside the right root.
//!
//! Neither of them sorts or filters anything. They hold the state and answer questions about it;
//! what to do with the answers is the caller's, which is what keeps them usable over a `Vec`, a
//! paginated resource, or a server that does the sorting itself.

use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortDirection::Ascending => "ascending",
            SortDirection::Descending => "descending",
        }
    }

    pub fn flipped(&self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }
}

/// Which column a table is sorted by, and which way.
#[derive(Copy, Clone)]
pub struct TableSort {
    /// The key of the column in force, or `None` for the order the rows arrived in.
    pub column: RwSignal<Option<String>>,
    pub direction: RwSignal<SortDirection>,
}

impl Default for TableSort {
    fn default() -> Self {
        Self {
            column: RwSignal::new(None),
            direction: RwSignal::new(SortDirection::Ascending),
        }
    }
}

impl TableSort {
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts sorted by `column`.
    pub fn sorted_by(column: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            column: RwSignal::new(Some(column.into())),
            direction: RwSignal::new(direction),
        }
    }

    pub fn is_sorted_by(&self, column: &str) -> bool {
        self.column
            .with(|current| current.as_deref() == Some(column))
    }

    /// What `aria-sort` should say for a column: only the one in force claims a direction.
    pub fn aria_sort(&self, column: &str) -> &'static str {
        if self.is_sorted_by(column) {
            self.direction.get().as_str()
        } else {
            "none"
        }
    }

    /// Clicking a header: a new column sorts ascending, the same one flips, and flipping a
    /// descending column clears the sort — three states, so there is always a way back to the
    /// order the rows came in.
    pub fn toggle(&self, column: &str) {
        if !self.is_sorted_by(column) {
            self.column.set(Some(column.to_string()));
            self.direction.set(SortDirection::Ascending);
            return;
        }

        match self.direction.get_untracked() {
            SortDirection::Ascending => self.direction.set(SortDirection::Descending),
            SortDirection::Descending => {
                self.column.set(None);
                self.direction.set(SortDirection::Ascending);
            }
        }
    }

    pub fn clear(&self) {
        self.column.set(None);
        self.direction.set(SortDirection::Ascending);
    }
}

/// How much of a page is selected, which is the whole of what a header checkbox needs to know.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectionState {
    None,
    Some,
    All,
}

/// Which rows are picked, by whatever id the caller uses for a row.
#[derive(Copy, Clone, Default)]
pub struct TableSelection {
    pub selected: RwSignal<Vec<String>>,
}

impl TableSelection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_selected(&self, id: &str) -> bool {
        self.selected
            .with(|selected| selected.iter().any(|s| s == id))
    }

    pub fn count(&self) -> usize {
        self.selected.with(|selected| selected.len())
    }

    pub fn toggle(&self, id: &str) {
        self.selected.update(|selected| {
            match selected.iter().position(|selected| selected == id) {
                Some(index) => {
                    selected.remove(index);
                }
                None => selected.push(id.to_string()),
            }
        });
    }

    pub fn set(&self, id: &str, selected: bool) {
        if selected != self.is_selected(id) {
            self.toggle(id);
        }
    }

    pub fn clear(&self) {
        self.selected.set(Vec::new());
    }

    /// Whether none, some or all of `ids` are picked. "Some" is what makes a header checkbox
    /// indeterminate.
    pub fn state(&self, ids: &[String]) -> SelectionState {
        if ids.is_empty() {
            return SelectionState::None;
        }

        let picked = ids.iter().filter(|id| self.is_selected(id)).count();

        match picked {
            0 => SelectionState::None,
            picked if picked == ids.len() => SelectionState::All,
            _ => SelectionState::Some,
        }
    }

    /// Selects or clears exactly `ids`, leaving anything selected elsewhere alone — a select-all
    /// on page two must not silently drop page one.
    pub fn set_all(&self, ids: &[String], selected: bool) {
        self.selected.update(|current| {
            current.retain(|id| !ids.contains(id));
            if selected {
                current.extend(ids.iter().cloned());
            }
        });
    }
}

/// A column header that can be sorted by. Renders the `<th>`, its `aria-sort` and the button that
/// toggles it; what the button looks like is the caller's.
#[component]
pub fn TableSortHeaderRoot(
    sort: TableSort,
    /// The key this column is known by in [`TableSort::column`].
    #[prop(into)]
    column: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] button_class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let key = StoredValue::new(column);

    view! {
        <th
            scope="col"
            data-slot="table-sort-header"
            aria-sort=move || key.with_value(|key| sort.aria_sort(key))
            data-sorted=move || {
                key.with_value(|key| sort.is_sorted_by(key)).then(|| sort.direction.get().as_str())
            }
            class=class
        >
            <button
                type="button"
                on:click=move |_| key.with_value(|key| sort.toggle(key))
                class=button_class
            >
                {children()}
            </button>
        </th>
    }
}
