//! Headless table sort and row-selection state.

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
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TableSortState {
    /// The key of the column in force, or `None` for the order the rows arrived in.
    pub column: Option<String>,
    pub direction: SortDirection,
}

impl TableSortState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts sorted by `column`.
    pub fn sorted_by(column: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            column: Some(column.into()),
            direction,
        }
    }

    pub fn is_sorted_by(&self, column: &str) -> bool {
        self.column.as_deref() == Some(column)
    }

    /// What `aria-sort` should say for a column: only the one in force claims a direction.
    pub fn aria_sort(&self, column: &str) -> &'static str {
        if self.is_sorted_by(column) {
            self.direction.as_str()
        } else {
            "none"
        }
    }

    /// Clicking a header: a new column sorts ascending, the same one flips, and flipping a
    /// descending column clears the sort — three states, so there is always a way back to the
    /// order the rows came in.
    pub fn toggle(&mut self, column: &str) {
        if !self.is_sorted_by(column) {
            self.column = Some(column.to_string());
            self.direction = SortDirection::Ascending;
            return;
        }

        match self.direction {
            SortDirection::Ascending => self.direction = SortDirection::Descending,
            SortDirection::Descending => {
                self.column = None;
                self.direction = SortDirection::Ascending;
            }
        }
    }

    pub fn clear(&mut self) {
        self.column = None;
        self.direction = SortDirection::Ascending;
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
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TableSelectionState {
    pub selected: Vec<String>,
}

impl TableSelectionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_selected(&self, id: &str) -> bool {
        self.selected.iter().any(|s| s == id)
    }

    pub fn count(&self) -> usize {
        self.selected.len()
    }

    pub fn toggle(&mut self, id: &str) {
        match self.selected.iter().position(|selected| selected == id) {
            Some(index) => {
                self.selected.remove(index);
            }
            None => self.selected.push(id.to_string()),
        }
    }

    pub fn set(&mut self, id: &str, selected: bool) {
        if selected != self.is_selected(id) {
            self.toggle(id);
        }
    }

    pub fn clear(&mut self) {
        self.selected.clear();
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
    pub fn set_all(&mut self, ids: &[String], selected: bool) {
        self.selected.retain(|id| !ids.contains(id));
        if selected {
            self.selected.extend(ids.iter().cloned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_cycles_ascending_descending_and_none() {
        let mut sort = TableSortState::new();
        assert_eq!(sort.aria_sort("name"), "none");

        sort.toggle("name");
        assert!(sort.is_sorted_by("name"));
        assert_eq!(sort.aria_sort("name"), "ascending");
        assert_eq!(sort.aria_sort("date"), "none");

        sort.toggle("name");
        assert_eq!(sort.aria_sort("name"), "descending");

        sort.toggle("name");
        assert!(!sort.is_sorted_by("name"));
        assert_eq!(sort.aria_sort("name"), "none");
    }

    #[test]
    fn switching_columns_resets_to_ascending_and_clear_resets() {
        let mut sort = TableSortState::sorted_by("name", SortDirection::Descending);
        assert_eq!(sort.aria_sort("name"), "descending");

        sort.toggle("date");
        assert!(sort.is_sorted_by("date"));
        assert_eq!(sort.aria_sort("date"), "ascending");

        sort.clear();
        assert_eq!(sort.column, None);
        assert_eq!(sort.direction, SortDirection::Ascending);
    }

    #[test]
    fn selection_tracks_individual_rows_and_page_state() {
        let page1: Vec<String> = vec!["1".into(), "2".into(), "3".into()];
        let page2: Vec<String> = vec!["4".into(), "5".into()];

        let mut selection = TableSelectionState::new();
        assert_eq!(selection.state(&[]), SelectionState::None);
        assert_eq!(selection.state(&page1), SelectionState::None);

        selection.toggle("1");
        assert!(selection.is_selected("1"));
        assert_eq!(selection.count(), 1);
        assert_eq!(selection.state(&page1), SelectionState::Some);

        selection.set("2", true);
        selection.set("2", true);
        assert_eq!(selection.count(), 2);

        selection.set_all(&page1, true);
        assert_eq!(selection.state(&page1), SelectionState::All);

        // Selecting all on page 2 leaves page 1 selected.
        selection.set_all(&page2, true);
        assert_eq!(selection.count(), 5);
        assert_eq!(selection.state(&page1), SelectionState::All);
        assert_eq!(selection.state(&page2), SelectionState::All);

        // Clearing page 2 leaves page 1 intact.
        selection.set_all(&page2, false);
        assert_eq!(selection.count(), 3);
        assert_eq!(selection.state(&page2), SelectionState::None);
        assert_eq!(selection.state(&page1), SelectionState::All);

        selection.clear();
        assert_eq!(selection.count(), 0);
    }
}
