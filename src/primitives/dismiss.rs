//! A global stack of dismissable layers.
//!
//! Every dismissable overlay registers here while open, ordered by the moment it opened, so
//! Escape takes only the topmost — one listener rather than one per overlay.
//!
//! Layers are keyed on the signal that drives them, so a layer leaves the stack when it closes
//! rather than when its exit animation finishes.

use leptos::{ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use std::cell::{Cell, RefCell};
use web_sys::Node;

/// What a pointerdown has to land in to count as inside a layer. A layer with no bounds —
/// `LayerBounds::modal()` — is never dismissed from outside and stops the cascade at itself, and
/// so is one whose [`modal_while`](LayerBounds::modal_while) signal is true.
#[derive(Clone, Copy, Default)]
pub struct LayerBounds {
    content: Option<AnyNodeRef>,
    trigger: Option<AnyNodeRef>,
    modal_while: Option<Signal<bool>>,
}

impl LayerBounds {
    /// A layer whose content and trigger are both known; outside pointerdowns dismiss it.
    pub fn new(content: AnyNodeRef, trigger: AnyNodeRef) -> Self {
        Self {
            content: Some(content),
            trigger: Some(trigger),
            modal_while: None,
        }
    }

    /// While `when` is true the layer behaves like [`modal`](LayerBounds::modal): outside
    /// pointerdowns neither dismiss it nor go past it to the layers below. For a layer that is
    /// floating at one width and a sheet at another, where the sheet's scrim closes it instead.
    pub fn modal_while(self, when: Signal<bool>) -> Self {
        Self {
            modal_while: Some(when),
            ..self
        }
    }

    /// A layer that outside pointerdowns never dismiss. Modals dismiss through their own overlay.
    pub fn modal() -> Self {
        Self::default()
    }

    fn dismisses_on_outside(&self) -> bool {
        (self.content.is_some() || self.trigger.is_some())
            && !self.modal_while.is_some_and(|when| when.get_untracked())
    }

    /// The trigger counts as inside, or clicking it while open would dismiss and then reopen.
    fn contains(&self, target: &Node) -> bool {
        [self.content, self.trigger]
            .into_iter()
            .flatten()
            .filter_map(|node_ref| node_ref.get())
            .any(|el| el.contains(Some(target)))
    }
}

struct Layer {
    id: u64,
    bounds: LayerBounds,
    dismiss: Callback<()>,
}

thread_local! {
    static LAYERS: RefCell<Vec<Layer>> = const { RefCell::new(Vec::new()) };
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };
    static ESCAPE_LISTENER: Cell<bool> = const { Cell::new(false) };
    static POINTER_LISTENER: Cell<bool> = const { Cell::new(false) };
}

fn next_id() -> u64 {
    NEXT_ID.with(|next| {
        let id = next.get();
        next.set(id + 1);
        id
    })
}

fn push(id: u64, bounds: LayerBounds, dismiss: Callback<()>) {
    LAYERS.with(|layers| {
        let mut layers = layers.borrow_mut();
        layers.retain(|layer| layer.id != id);
        layers.push(Layer {
            id,
            bounds,
            dismiss,
        });
    });
}

fn remove(id: u64) {
    LAYERS.with(|layers| layers.borrow_mut().retain(|layer| layer.id != id));
}

/// Dismisses the topmost open layer, returning whether there was one. The callback is copied out
/// of the borrow first: what reacts to the signal must be free to touch the stack.
pub fn dismiss_topmost() -> bool {
    let topmost = LAYERS.with(|layers| layers.borrow().last().map(|layer| layer.dismiss));

    match topmost {
        Some(dismiss) => {
            dismiss.run(());
            true
        }
        None => false,
    }
}

/// Dismisses every layer above the one the pointer landed in. Walking down from the top, a layer
/// containing the target keeps itself and everything below, and so does a modal one.
fn dismiss_outside(target: &Node) {
    let stack: Vec<(LayerBounds, Callback<()>)> = LAYERS.with(|layers| {
        layers
            .borrow()
            .iter()
            .map(|layer| (layer.bounds, layer.dismiss))
            .collect()
    });

    for (bounds, dismiss) in stack.into_iter().rev() {
        if bounds.contains(target) || !bounds.dismisses_on_outside() {
            break;
        }
        dismiss.run(());
    }
}

fn install_listeners() {
    install_escape_listener();
    install_outside_listener();
}

fn install_outside_listener() {
    if POINTER_LISTENER.with(|installed| installed.replace(true)) {
        return;
    }

    // Deliberately leaked, like the Escape listener below.
    let _ = window_event_listener(ev::pointerdown, |e| {
        if let Some(target) = e.target().and_then(|t| t.dyn_into::<Node>().ok()) {
            dismiss_outside(&target);
        }
    });
}

fn install_escape_listener() {
    if ESCAPE_LISTENER.with(|installed| installed.replace(true)) {
        return;
    }

    // Deliberately leaked: the listener is installed once and lives as long as the document.
    let _ = window_event_listener(ev::keydown, |e| {
        if e.key() == "Escape" && dismiss_topmost() {
            e.prevent_default();
        }
    });
}

/// Registers `is_open` as a dismissable layer, for the length of the signal being true. Escape
/// runs `on_dismiss` only when it is topmost; a pointerdown outside `bounds` runs it unless the
/// layer is modal, or is modal at that moment through [`LayerBounds::modal_while`]. Call once per
/// layer root, in the root itself.
pub fn use_dismissable_layer(
    is_open: RwSignal<bool>,
    bounds: LayerBounds,
    on_dismiss: impl Fn() + Send + Sync + 'static,
) {
    install_listeners();

    let id = next_id();
    let dismiss = Callback::new(move |()| on_dismiss());

    Effect::new(move |_| {
        if is_open.get() {
            push(id, bounds, dismiss);
        } else {
            remove(id);
        }
    });

    on_cleanup(move || remove(id));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds() -> LayerBounds {
        LayerBounds::new(AnyNodeRef::new(), AnyNodeRef::new())
    }

    #[test]
    fn modal_does_not_dismiss_on_outside() {
        let _owner = Owner::new();
        assert!(!LayerBounds::modal().dismisses_on_outside());
    }

    #[test]
    fn bounded_dismisses_on_outside() {
        let _owner = Owner::new();
        assert!(bounds().dismisses_on_outside());
    }

    #[test]
    fn modal_while_follows_the_signal() {
        let owner = Owner::new();
        owner.set();
        let sheet = RwSignal::new(false);
        let layer = bounds().modal_while(sheet.into());

        assert!(layer.dismisses_on_outside());
        sheet.set(true);
        assert!(!layer.dismisses_on_outside());
        sheet.set(false);
        assert!(layer.dismisses_on_outside());
    }
}
