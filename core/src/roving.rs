//! Index-based roving focus over linear item lists with disabled-item skipping.

/// Pure index-based roving focus calculation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RovingIndex {
    pub wrap: bool,
}

impl Default for RovingIndex {
    fn default() -> Self {
        Self { wrap: true }
    }
}

impl RovingIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// First enabled index in `0..count`, or `None` if all items are disabled or empty.
    pub fn first(&self, count: usize, is_disabled: impl Fn(usize) -> bool) -> Option<usize> {
        (0..count).find(|&index| !is_disabled(index))
    }

    /// Last enabled index in `0..count`, or `None` if all items are disabled or empty.
    pub fn last(&self, count: usize, is_disabled: impl Fn(usize) -> bool) -> Option<usize> {
        (0..count).rev().find(|&index| !is_disabled(index))
    }

    /// Steps from `current` by `offset` enabled items, skipping disabled indices and optionally
    /// wrapping at the ends.
    pub fn step(
        &self,
        count: usize,
        current: Option<usize>,
        offset: isize,
        is_disabled: impl Fn(usize) -> bool,
    ) -> Option<usize> {
        if count == 0 {
            return None;
        }

        let current = current.filter(|&index| index < count);
        if offset == 0 {
            return match current {
                Some(index) if !is_disabled(index) => Some(index),
                _ => self.first(count, is_disabled),
            };
        }

        let mut index = match current {
            Some(index) => index,
            None => {
                return if offset > 0 {
                    self.first(count, is_disabled)
                } else {
                    self.last(count, is_disabled)
                };
            }
        };

        let steps = offset.unsigned_abs();
        let forward = offset > 0;

        for _ in 0..steps {
            let next = if forward {
                self.step_forward(count, index, &is_disabled)
            } else {
                self.step_backward(count, index, &is_disabled)
            };

            match next {
                Some(found) => index = found,
                None => {
                    return if !is_disabled(index) {
                        Some(index)
                    } else {
                        None
                    };
                }
            }
        }

        Some(index)
    }

    fn step_forward(
        &self,
        count: usize,
        from: usize,
        is_disabled: &impl Fn(usize) -> bool,
    ) -> Option<usize> {
        if self.wrap {
            (1..=count)
                .map(|delta| (from + delta) % count)
                .find(|&candidate| !is_disabled(candidate))
        } else {
            ((from + 1)..count).find(|&candidate| !is_disabled(candidate))
        }
    }

    fn step_backward(
        &self,
        count: usize,
        from: usize,
        is_disabled: &impl Fn(usize) -> bool,
    ) -> Option<usize> {
        if self.wrap {
            (1..=count)
                .map(|delta| (from + count - (delta % count)) % count)
                .find(|&candidate| !is_disabled(candidate))
        } else {
            (0..from).rev().find(|&candidate| !is_disabled(candidate))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_and_last_skip_disabled_items() {
        let roving = RovingIndex::new();
        let disabled = |i: usize| matches!(i, 0 | 3);

        assert_eq!(roving.first(4, disabled), Some(1));
        assert_eq!(roving.last(4, disabled), Some(2));
        assert_eq!(roving.first(4, |_| true), None);
        assert_eq!(roving.last(0, |_| false), None);
    }

    #[test]
    fn step_wraps_and_skips_disabled_when_enabled() {
        let roving = RovingIndex::new();
        let disabled = |i: usize| matches!(i, 1 | 3);

        // From none: forward enters at first enabled, backward at last enabled.
        assert_eq!(roving.step(4, None, 1, disabled), Some(0));
        assert_eq!(roving.step(4, None, -1, disabled), Some(2));

        // Skipping disabled 1 and 3 while wrapping.
        assert_eq!(roving.step(4, Some(0), 1, disabled), Some(2));
        assert_eq!(roving.step(4, Some(2), 1, disabled), Some(0));
        assert_eq!(roving.step(4, Some(0), -1, disabled), Some(2));
        assert_eq!(roving.step(4, Some(2), -1, disabled), Some(0));
    }

    #[test]
    fn step_stops_at_ends_without_wrap() {
        let roving = RovingIndex::new().with_wrap(false);
        let disabled = |i: usize| matches!(i, 1 | 4);

        assert_eq!(roving.step(5, Some(0), 1, disabled), Some(2));
        assert_eq!(roving.step(5, Some(3), 1, disabled), Some(3));
        assert_eq!(roving.step(5, Some(2), -1, disabled), Some(0));
        assert_eq!(roving.step(5, Some(0), -1, disabled), Some(0));
    }

    #[test]
    fn multi_step_offset_advances_across_enabled_items() {
        let roving = RovingIndex::new();
        let disabled = |i: usize| i == 1;

        assert_eq!(roving.step(5, Some(0), 2, disabled), Some(3));
        assert_eq!(roving.step(5, Some(3), -2, disabled), Some(0));
    }
}
