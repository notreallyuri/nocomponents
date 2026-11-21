//! Toast styling enums and headless [`ToastQueue`].

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum ToastVariant {
    #[default]
    Default,
    Destructive,
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum ToastSize {
    #[default]
    Default,
    Lg,
}

impl ToastSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToastSize::Default => "default",
            ToastSize::Lg => "lg",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl ToastPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToastPosition::TopLeft => "top-left",
            ToastPosition::TopCenter => "top-center",
            ToastPosition::TopRight => "top-right",
            ToastPosition::BottomLeft => "bottom-left",
            ToastPosition::BottomCenter => "bottom-center",
            ToastPosition::BottomRight => "bottom-right",
        }
    }
}

/// One toast stored in [`ToastQueue`], paired with its monotonic id.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ToastEntry<T> {
    pub id: u64,
    pub data: T,
}

/// Headless toast queue with monotonic `u64` ids, optional `max_visible` cap, and pause state.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ToastQueue<T> {
    items: Vec<ToastEntry<T>>,
    next_id: u64,
    max_visible: Option<usize>,
    paused: bool,
}

impl<T> Default for ToastQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ToastQueue<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
            max_visible: None,
            paused: false,
        }
    }

    pub fn with_max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = Some(max_visible);
        self
    }

    pub fn max_visible(&self) -> Option<usize> {
        self.max_visible
    }

    pub fn set_max_visible(&mut self, max_visible: Option<usize>) {
        self.max_visible = max_visible;
    }

    /// Pushes a toast onto the queue and returns its monotonic `u64` id.
    pub fn push(&mut self, data: T) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(ToastEntry { id, data });
        id
    }

    /// Removes the toast with `id`, returning it if present.
    pub fn dismiss(&mut self, id: u64) -> Option<ToastEntry<T>> {
        let index = self.items.iter().position(|entry| entry.id == id)?;
        Some(self.items.remove(index))
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[ToastEntry<T>] {
        &self.items
    }

    /// The newest toasts up to `max_visible`, or all items if no cap is set.
    pub fn visible(&self) -> &[ToastEntry<T>] {
        match self.max_visible {
            Some(max) => {
                let start = self.items.len().saturating_sub(max);
                &self.items[start..]
            }
            None => &self.items,
        }
    }

    pub fn get(&self, id: u64) -> Option<&ToastEntry<T>> {
        self.items.iter().find(|entry| entry.id == id)
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_assigns_monotonic_ids_and_dismiss_removes_by_id() {
        let mut queue = ToastQueue::new();
        assert!(queue.is_empty());

        let first = queue.push("Saved");
        let second = queue.push("Updated");
        let third = queue.push("Deleted");
        assert!(first < second && second < third);
        assert_eq!(queue.len(), 3);

        let removed = queue.dismiss(second).unwrap();
        assert_eq!(removed.data, "Updated");
        assert_eq!(queue.len(), 2);
        assert!(queue.dismiss(second).is_none());

        let fourth = queue.push("Restored");
        assert!(fourth > third);

        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn max_visible_caps_visible_slice_to_newest_entries() {
        let mut queue = ToastQueue::new().with_max_visible(2);
        let id1 = queue.push("One");
        let id2 = queue.push("Two");
        let id3 = queue.push("Three");

        assert_eq!(queue.items().len(), 3);
        let visible: Vec<&str> = queue.visible().iter().map(|e| e.data).collect();
        assert_eq!(visible, ["Two", "Three"]);

        queue.dismiss(id3);
        let visible: Vec<&str> = queue.visible().iter().map(|e| e.data).collect();
        assert_eq!(visible, ["One", "Two"]);

        assert!(queue.get(id1).is_some());
        assert!(queue.get(id2).is_some());
    }

    #[test]
    fn pause_and_resume_track_hover_or_focus_state() {
        let mut queue: ToastQueue<&str> = ToastQueue::new();
        assert!(!queue.is_paused());

        queue.pause();
        assert!(queue.is_paused());

        queue.resume();
        assert!(!queue.is_paused());

        queue.set_paused(true);
        assert!(queue.is_paused());
    }
}
