//! Type-to-jump prefix accumulation and case-insensitive item matching.

/// Default pause (in milliseconds) after which typed characters reset into a new search.
pub const DEFAULT_RESET_AFTER_MS: u64 = 500;

/// Headless typeahead search buffer with a configurable reset window.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeaheadBuffer {
    buffer: String,
    last_input_ms: Option<u64>,
    pub reset_after_ms: u64,
}

impl Default for TypeaheadBuffer {
    fn default() -> Self {
        Self::new(DEFAULT_RESET_AFTER_MS)
    }
}

impl TypeaheadBuffer {
    pub fn new(reset_after_ms: u64) -> Self {
        Self {
            buffer: String::new(),
            last_input_ms: None,
            reset_after_ms,
        }
    }

    /// Accumulates a typed character, resetting the buffer if `now_ms` is past the reset window.
    /// Control characters are ignored and return `false`.
    pub fn push(&mut self, ch: char, now_ms: u64) -> bool {
        if ch.is_control() {
            return false;
        }
        if self.buffer.is_empty() && ch == ' ' {
            return false;
        }

        if let Some(last) = self.last_input_ms
            && now_ms.saturating_sub(last) >= self.reset_after_ms
        {
            self.buffer.clear();
        }

        for lower in ch.to_lowercase() {
            self.buffer.push(lower);
        }
        self.last_input_ms = Some(now_ms);
        true
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.last_input_ms = None;
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// The raw accumulated lowercase query string.
    pub fn query(&self) -> &str {
        &self.buffer
    }

    /// Whether the buffer consists of the same character repeated (e.g. `"bb"`), which cycles
    /// through items starting with that character rather than matching `"bb..."`.
    pub fn is_repeated(&self) -> bool {
        let mut chars = self.buffer.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        let rest_count = chars.clone().count();
        rest_count > 0 && chars.all(|c| c == first)
    }

    /// The search prefix: a single character when [`is_repeated`](Self::is_repeated), otherwise the
    /// full buffer.
    pub fn needle(&self) -> &str {
        if self.is_repeated() {
            let first_len = self.buffer.chars().next().map(char::len_utf8).unwrap_or(0);
            &self.buffer[..first_len]
        } else {
            &self.buffer
        }
    }

    /// Finds the first matching item index, wrapping around after `start_after` case-insensitively.
    pub fn find_match<S: AsRef<str>>(
        &self,
        items: &[S],
        start_after: Option<usize>,
    ) -> Option<usize> {
        let needle = self.needle();
        if needle.is_empty() || items.is_empty() {
            return None;
        }

        let start = match start_after {
            Some(index) => (index + 1) % items.len(),
            None => 0,
        };

        (0..items.len())
            .map(|offset| (start + offset) % items.len())
            .find(|&index| {
                items[index]
                    .as_ref()
                    .trim()
                    .to_lowercase()
                    .starts_with(needle)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accumulates_prefix_within_window_and_resets_after_pause() {
        let mut buf = TypeaheadBuffer::new(500);
        let items = ["Apple", "Apricot", "Banana", "Blueberry"];

        assert!(buf.push('b', 100));
        assert_eq!(buf.query(), "b");
        assert_eq!(buf.find_match(&items, None), Some(2));

        assert!(buf.push('l', 300));
        assert_eq!(buf.query(), "bl");
        assert_eq!(buf.find_match(&items, None), Some(3));

        // Past the 500ms window: resets to "a".
        assert!(buf.push('a', 900));
        assert_eq!(buf.query(), "a");
        assert_eq!(buf.find_match(&items, None), Some(0));

        buf.clear();
        assert!(buf.is_empty());
        assert_eq!(buf.find_match(&items, None), None);
    }

    #[test]
    fn repeated_letter_cycles_and_wraps_case_insensitively() {
        let mut buf = TypeaheadBuffer::default();
        let items = ["Alpha", "Banana", "Blueberry", "Cherry"];

        buf.push('B', 0);
        assert_eq!(buf.find_match(&items, None), Some(1));

        buf.push('B', 100);
        assert!(buf.is_repeated());
        assert_eq!(buf.needle(), "b");
        assert_eq!(buf.find_match(&items, Some(1)), Some(2));

        // Wraps around past Cherry and Alpha back to Banana.
        buf.push('b', 200);
        assert_eq!(buf.find_match(&items, Some(2)), Some(1));
    }

    #[test]
    fn ignores_control_and_leading_space() {
        let mut buf = TypeaheadBuffer::default();
        assert!(!buf.push('\n', 0));
        assert!(!buf.push(' ', 10));
        assert!(buf.is_empty());
    }
}
