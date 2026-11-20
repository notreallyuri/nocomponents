//! Headless state machines for tabs, accordions, and resizable split panels.

/// Headless active-tab state.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TabsState {
    pub active: String,
}

impl TabsState {
    pub fn new(active: impl Into<String>) -> Self {
        Self {
            active: active.into(),
        }
    }

    pub fn active(&self) -> &str {
        &self.active
    }

    pub fn is_active(&self, value: &str) -> bool {
        self.active == value
    }

    /// Selects `value`, returning `true` if the active tab changed.
    pub fn select(&mut self, value: impl Into<String>) -> bool {
        let next = value.into();
        if self.active != next {
            self.active = next;
            true
        } else {
            false
        }
    }
}

/// Whether an accordion permits one open section or many.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AccordionType {
    /// Opening a section closes the one that was open.
    #[default]
    Single,
    /// Any number of sections can be open at once.
    Multiple,
}

/// Headless accordion disclosure state supporting single/multiple and collapsible behaviour.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AccordionState {
    pub open: Vec<String>,
    pub kind: AccordionType,
    pub collapsible: bool,
}

impl Default for AccordionState {
    fn default() -> Self {
        Self {
            open: Vec::new(),
            kind: AccordionType::Single,
            collapsible: true,
        }
    }
}

impl AccordionState {
    pub fn new(kind: AccordionType, collapsible: bool) -> Self {
        Self {
            open: Vec::new(),
            kind,
            collapsible,
        }
    }

    pub fn single(collapsible: bool) -> Self {
        Self::new(AccordionType::Single, collapsible)
    }

    pub fn multiple() -> Self {
        Self::new(AccordionType::Multiple, true)
    }

    pub fn with_open(mut self, open: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.open = open.into_iter().map(Into::into).collect();
        if self.kind == AccordionType::Single && self.open.len() > 1 {
            self.open.truncate(1);
        }
        self
    }

    pub fn is_open(&self, value: &str) -> bool {
        self.open.iter().any(|item| item == value)
    }

    pub fn toggle(&mut self, value: &str) {
        let existing = self.open.iter().position(|item| item == value);
        match (self.kind, existing) {
            (AccordionType::Single, Some(_)) => {
                if self.collapsible {
                    self.open.clear();
                }
            }
            (AccordionType::Single, None) => {
                self.open.clear();
                self.open.push(value.to_string());
            }
            (AccordionType::Multiple, Some(index)) => {
                self.open.remove(index);
            }
            (AccordionType::Multiple, None) => {
                self.open.push(value.to_string());
            }
        }
    }

    pub fn open_items(&self) -> &[String] {
        &self.open
    }
}

/// Initial size and min/max bounds (in percentage points, 0–100) for one resizable panel.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ResizablePanel {
    pub min: f64,
    pub max: f64,
    pub default: Option<f64>,
}

impl Default for ResizablePanel {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            default: None,
        }
    }
}

/// Headless percentage-based split-panel layout whose sizes always sum to 100%.
#[derive(Clone, PartialEq, Debug)]
pub struct ResizableState {
    pub sizes: Vec<f64>,
    pub limits: Vec<(f64, f64)>,
}

impl ResizableState {
    /// Distributes initial percentages across `panels` so the total equals 100%.
    pub fn new(panels: &[ResizablePanel]) -> Self {
        if panels.is_empty() {
            return Self {
                sizes: Vec::new(),
                limits: Vec::new(),
            };
        }

        let claimed: f64 = panels.iter().filter_map(|panel| panel.default).sum();
        let unclaimed = panels
            .iter()
            .filter(|panel| panel.default.is_none())
            .count();
        let share = if unclaimed > 0 {
            ((100.0 - claimed) / unclaimed as f64).max(0.0)
        } else {
            0.0
        };

        let mut sizes: Vec<f64> = panels
            .iter()
            .map(|panel| panel.default.unwrap_or(share))
            .collect();

        let total: f64 = sizes.iter().sum();
        if total > 0.0 && (total - 100.0).abs() > f64::EPSILON {
            for size in &mut sizes {
                *size = *size / total * 100.0;
            }
        }

        let limits = panels
            .iter()
            .map(|panel| (panel.min, panel.max.max(panel.min)))
            .collect();

        Self { sizes, limits }
    }

    /// Creates `count` equally-sized panels with `[0.0, 100.0]` bounds.
    pub fn equal(count: usize) -> Self {
        Self::new(&vec![ResizablePanel::default(); count])
    }

    pub fn size(&self, index: usize) -> f64 {
        self.sizes.get(index).copied().unwrap_or(0.0)
    }

    /// Moves the boundary after panel `handle_index` by `delta_percent` from its current sizes,
    /// honouring both adjacent panels' min/max limits while keeping the sum at 100%.
    pub fn drag_handle(&mut self, handle_index: usize, delta_percent: f64) -> bool {
        let (Some(&before), Some(&after)) = (
            self.sizes.get(handle_index),
            self.sizes.get(handle_index + 1),
        ) else {
            return false;
        };
        self.resize_from(handle_index, (before, after), delta_percent)
    }

    /// Moves the boundary after panel `handle_index` by `delta_percent` measured from `from`
    /// (the pair of sizes when a drag started, avoiding accumulated rounding drift).
    pub fn resize_from(
        &mut self,
        handle_index: usize,
        from: (f64, f64),
        delta_percent: f64,
    ) -> bool {
        let (Some(&(before_min, before_max)), Some(&(after_min, after_max))) = (
            self.limits.get(handle_index),
            self.limits.get(handle_index + 1),
        ) else {
            return false;
        };

        let (start_before, start_after) = from;
        let wanted = (start_before + delta_percent).clamp(before_min, before_max.max(before_min));
        let applied = wanted - start_before;
        let other = (start_after - applied).clamp(after_min, after_max.max(after_min));
        let applied = start_after - other;

        let next_before = start_before + applied;
        if let (Some(&curr_a), Some(&curr_b)) = (
            self.sizes.get(handle_index),
            self.sizes.get(handle_index + 1),
        ) && (curr_a - next_before).abs() + (curr_b - other).abs() > f64::EPSILON
        {
            self.sizes[handle_index] = next_before;
            self.sizes[handle_index + 1] = other;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_state_tracks_active_tab() {
        let mut tabs = TabsState::new("account");
        assert!(tabs.is_active("account"));
        assert!(!tabs.select("account"));
        assert!(tabs.select("password"));
        assert_eq!(tabs.active(), "password");
    }

    #[test]
    fn accordion_single_respects_collapsible_flag() {
        let mut collapsible = AccordionState::single(true).with_open(["a"]);
        assert!(collapsible.is_open("a"));
        collapsible.toggle("a");
        assert!(!collapsible.is_open("a"));
        collapsible.toggle("b");
        assert_eq!(collapsible.open_items(), ["b"]);
        collapsible.toggle("c");
        assert_eq!(collapsible.open_items(), ["c"]);

        let mut non_collapsible = AccordionState::single(false).with_open(["a"]);
        non_collapsible.toggle("a");
        assert!(non_collapsible.is_open("a"));
        non_collapsible.toggle("b");
        assert_eq!(non_collapsible.open_items(), ["b"]);
    }

    #[test]
    fn accordion_multiple_allows_several_open_sections() {
        let mut acc = AccordionState::multiple();
        acc.toggle("a");
        acc.toggle("b");
        assert!(acc.is_open("a"));
        assert!(acc.is_open("b"));
        acc.toggle("a");
        assert!(!acc.is_open("a"));
        assert!(acc.is_open("b"));
    }

    #[test]
    fn resizable_state_distributes_and_clamps_preserving_total() {
        let mut state = ResizableState::new(&[
            ResizablePanel {
                min: 20.0,
                max: 60.0,
                default: Some(30.0),
            },
            ResizablePanel {
                min: 25.0,
                max: 80.0,
                default: None,
            },
            ResizablePanel {
                min: 10.0,
                max: 50.0,
                default: None,
            },
        ]);

        assert_eq!(state.sizes, [30.0, 35.0, 35.0]);
        let sum: f64 = state.sizes.iter().sum();
        assert!((sum - 100.0).abs() < 1e-9);

        // Panel 0 wants +50 -> clamped at max 60, but panel 1 has min 25 so max delta is +10.
        assert!(state.drag_handle(0, 50.0));
        assert_eq!(state.sizes, [40.0, 25.0, 35.0]);

        // Out-of-bounds handle is a no-op.
        assert!(!state.drag_handle(2, 10.0));
        let sum: f64 = state.sizes.iter().sum();
        assert!((sum - 100.0).abs() < 1e-9);
    }
}
