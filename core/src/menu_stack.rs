//! View stack for menus whose panels swap in place: the root view, then the sub-views the user
//! stepped into.

/// What Escape does at the current depth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EscapeAction<K> {
    /// A sub-view is showing: go back to the view below it (the popped key is the row to refocus).
    Back(K),
    /// The root is showing: close the menu.
    Close,
}

/// Stack of sub-view keys inside an in-place menu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuStack<K> {
    stack: Vec<K>,
}

impl<K> Default for MenuStack<K> {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl<K: Clone + PartialEq> MenuStack<K> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The sub-view on show, or `None` at the root.
    pub fn current(&self) -> Option<&K> {
        self.stack.last()
    }

    pub fn is_root(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Steps into a sub-view. Pushing the view already on show changes nothing.
    pub fn push(&mut self, key: K) {
        if self.stack.last() != Some(&key) {
            self.stack.push(key);
        }
    }

    /// Steps back one view; returns the one left, or `None` at the root.
    pub fn pop(&mut self) -> Option<K> {
        self.stack.pop()
    }

    /// What Escape should do now, popping the current sub-view if any; at the root it reports `Close`.
    pub fn escape(&mut self) -> EscapeAction<K> {
        match self.stack.pop() {
            Some(left) => EscapeAction::Back(left),
            None => EscapeAction::Close,
        }
    }

    /// Back to the root, for when the menu closes.
    pub fn reset(&mut self) {
        self.stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_the_root() {
        let stack = MenuStack::<&str>::new();
        assert!(stack.is_root());
        assert_eq!(stack.current(), None);
        assert_eq!(stack.depth(), 0);
    }

    #[test]
    fn push_shows_the_view_and_pop_returns_it() {
        let mut stack = MenuStack::new();
        stack.push("speed");
        assert_eq!(stack.current(), Some(&"speed"));
        assert!(!stack.is_root());
        assert_eq!(stack.pop(), Some("speed"));
        assert!(stack.is_root());
    }

    #[test]
    fn pop_at_the_root_is_nothing() {
        let mut stack = MenuStack::<&str>::new();
        assert_eq!(stack.pop(), None);
        assert!(stack.is_root());
    }

    #[test]
    fn pushing_the_view_on_show_again_does_not_stack() {
        let mut stack = MenuStack::new();
        stack.push("speed");
        stack.push("speed");
        assert_eq!(stack.depth(), 1);
    }

    #[test]
    fn nested_views_unwind_in_order() {
        let mut stack = MenuStack::new();
        stack.push("quality");
        stack.push("custom");
        assert_eq!(stack.current(), Some(&"custom"));
        assert_eq!(stack.pop(), Some("custom"));
        assert_eq!(stack.current(), Some(&"quality"));
    }

    #[test]
    fn escape_goes_back_first_then_closes() {
        let mut stack = MenuStack::new();
        stack.push("speed");
        assert_eq!(stack.escape(), EscapeAction::Back("speed"));
        assert!(stack.is_root());
        assert_eq!(stack.escape(), EscapeAction::Close);
        assert!(stack.is_root());
    }

    #[test]
    fn reset_returns_to_the_root_from_any_depth() {
        let mut stack = MenuStack::new();
        stack.push("quality");
        stack.push("custom");
        stack.reset();
        assert!(stack.is_root());
        assert_eq!(stack.escape(), EscapeAction::Close);
    }
}
