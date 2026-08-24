use leptos::{ev, prelude::*, wasm_bindgen::JsCast};
use std::{sync::Arc, time::Duration};
use web_sys::Element;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Destructive,
}

#[derive(Copy, Clone, PartialEq, Default)]
pub enum ToastSize {
    #[default]
    Default,
    Lg,
}

impl ToastSize {
    fn as_str(&self) -> &'static str {
        match self {
            ToastSize::Default => "default",
            ToastSize::Lg => "lg",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

#[derive(Copy, Clone)]
pub struct ToastDefaults {
    pub variant: ToastVariant,
    pub size: ToastSize,
    pub position: ToastPosition,
    pub duration: f64,
}

/// The one thing a toast can be pressed for: "deleted · undo", "saved · view". A toast that
/// announces something the user may want back is only half of the errand without it.
#[derive(Clone)]
pub struct ToastAction {
    pub label: String,
    /// Run when the action is pressed; the toast dismisses itself afterwards either way, since
    /// the thing it was announcing has just been answered.
    ///
    /// A plain `Arc` rather than a [`Callback`], because of what fires a toast with an action in
    /// it: something that is on its way out. `rows.remove(index)` disposes the reactive owner the
    /// row's own click handler is running under, and a `Callback` — arena-allocated under
    /// whichever owner made it — would go down with it, taking the undo with it.
    pub on_press: Arc<dyn Fn() + Send + Sync>,
}

#[derive(Clone)]
pub struct ToastData {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub size: ToastSize,
    pub position: ToastPosition,
    pub is_open: RwSignal<bool>,
    pub height: RwSignal<f64>,
    pub duration: f64,
    pub action: Option<ToastAction>,
}

#[derive(Copy, Clone)]
pub struct ToastContext {
    pub toasts: RwSignal<Vec<ToastData>>,
    pub next_id: RwSignal<usize>,
    pub defaults: ToastDefaults,
    /// The provider's own reactive owner, and the reason a toast can outlive what fired it. A
    /// toast is very often raised by something on its way out — the row being deleted, the dialog
    /// being closed — and [`ToastBuilder::show`] mints signals. Minted under the caller's owner
    /// they would be disposed in the same breath, which is a panic rather than a missing toast.
    pub owner: StoredValue<Option<Owner>>,
}

impl ToastContext {
    pub fn toast(&self, title: impl Into<String>) -> ToastBuilder {
        ToastBuilder {
            ctx: *self,
            title: title.into(),
            description: None,
            variant: None,
            size: None,
            position: None,
            duration: None,
            action: None,
        }
    }

    pub fn dismiss(&self, id: usize) {
        self.toasts.update_untracked(|t| {
            if let Some(toast) = t.iter_mut().find(|t| t.id == id) {
                toast.is_open.set(false);
            }
        });

        let toasts = self.toasts;
        set_timeout(
            move || {
                toasts.update(|t| t.retain(|toast| toast.id != id));
            },
            Duration::from_millis(350),
        );
    }
}

pub struct ToastBuilder {
    ctx: ToastContext,
    title: String,
    description: Option<String>,
    variant: Option<ToastVariant>,
    size: Option<ToastSize>,
    position: Option<ToastPosition>,
    duration: Option<f64>,
    action: Option<ToastAction>,
}

impl ToastBuilder {
    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn duration(mut self, ms: f64) -> Self {
        self.duration = Some(ms);
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn size(mut self, size: ToastSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// A button in the toast: the label, and what pressing it does. Pressing it also dismisses
    /// the toast — "undo" that leaves the toast offering to undo again is a second undo waiting
    /// to happen.
    pub fn action(
        mut self,
        label: impl Into<String>,
        on_press: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        self.action = Some(ToastAction {
            label: label.into(),
            on_press: Arc::new(on_press),
        });
        self
    }

    /// Raises the toast. Its signals belong to the provider rather than to whatever called this,
    /// so a caller may dispose itself in the same handler — deleting the row it is announcing,
    /// closing the dialog it was raised from — and still have a toast afterwards.
    pub fn show(self) {
        match self.ctx.owner.get_value() {
            Some(owner) => owner.clone().with(|| self.push()),
            None => self.push(),
        }
    }

    fn push(self) {
        let id = self.ctx.next_id.get_untracked();
        self.ctx.next_id.update(|n| *n += 1);

        let is_open = RwSignal::new(true);
        let height = RwSignal::new(0.0_f64);

        let toast = ToastData {
            id,
            title: self.title,
            description: self.description,
            variant: self.variant.unwrap_or(self.ctx.defaults.variant),
            size: self.size.unwrap_or(self.ctx.defaults.size),
            position: self.position.unwrap_or(self.ctx.defaults.position),
            duration: self.duration.unwrap_or(self.ctx.defaults.duration),
            is_open,
            height,
            action: self.action,
        };

        self.ctx.toasts.update(|t| t.push(toast));
    }
}

pub fn use_toast() -> ToastContext {
    expect_context::<ToastContext>()
}

#[component]
pub fn ToastProviderRoot(
    #[prop(default = None, into)] default_variant: Option<ToastVariant>,
    #[prop(default = None, into)] default_size: Option<ToastSize>,
    #[prop(default = None, into)] default_position: Option<ToastPosition>,
    children: Children,
) -> impl IntoView {
    let toasts = RwSignal::new(Vec::new());
    let next_id = RwSignal::new(0);

    let defaults = ToastDefaults {
        variant: default_variant.unwrap_or_default(),
        size: default_size.unwrap_or_default(),
        position: default_position.unwrap_or_default(),
        duration: 5000.0,
    };

    provide_context(ToastContext {
        toasts,
        next_id,
        defaults,
        owner: StoredValue::new(Owner::current()),
    });

    view! { {children()} }
}

#[component]
pub fn ToastRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] style: Signal<String>,
    #[prop(optional)] is_interacting: Option<RwSignal<bool>>,
    toast: ToastData,
    children: Children,
) -> impl IntoView {
    let ctx = use_toast();
    let id = toast.id;

    let is_dragging = RwSignal::new(false);
    let drag_offset_x = RwSignal::new(0);
    let drag_offset_y = RwSignal::new(0);
    let start_x = RwSignal::new(0);
    let start_y = RwSignal::new(0);

    let on_pointer_down = move |e: ev::PointerEvent| {
        is_dragging.set(true);

        if let Some(interacting) = is_interacting {
            interacting.set(true);
        }

        start_x.set(e.client_x());
        start_y.set(e.client_y());

        if let Some(target) = e.target()
            && let Ok(el) = target.dyn_into::<Element>()
        {
            let _ = el.set_pointer_capture(e.pointer_id());
        }
    };

    let on_pointer_move = move |e: ev::PointerEvent| {
        if is_dragging.get() {
            let diff_x = e.client_x() - start_x.get();
            let diff_y = e.client_y() - start_y.get();

            match toast.position {
                ToastPosition::TopLeft | ToastPosition::BottomLeft => {
                    if diff_x < 0 {
                        drag_offset_x.set(diff_x);
                    }
                }
                ToastPosition::TopRight | ToastPosition::BottomRight => {
                    if diff_x > 0 {
                        drag_offset_x.set(diff_x);
                    }
                }
                ToastPosition::TopCenter => {
                    if diff_y < 0 {
                        drag_offset_y.set(diff_y);
                    }
                }
                ToastPosition::BottomCenter => {
                    if diff_y > 0 {
                        drag_offset_y.set(diff_y);
                    }
                }
            }
        }
    };

    let on_pointer_up = move |e: ev::PointerEvent| {
        if is_dragging.get() {
            is_dragging.set(false);

            if let Some(interacting) = is_interacting {
                interacting.set(false);
            }

            if let Some(target) = e.target()
                && let Ok(el) = target.dyn_into::<Element>()
            {
                let _ = el.release_pointer_capture(e.pointer_id());
            }

            if drag_offset_x.get().abs() > 60 || drag_offset_y.get().abs() > 60 {
                ctx.dismiss(id);
            } else {
                drag_offset_x.set(0);
                drag_offset_y.set(0);
            }
        }
    };

    view! {
        <div
            data-state=move || if toast.is_open.get() { "open" } else { "closed" }
            data-dragging=move || if is_dragging.get() { "true" } else { "false" }
            data-size=toast.size.as_str()
            style=move || {
                let base = style.get();
                let x = drag_offset_x.get();
                let y = drag_offset_y.get();
                format!("{base} --drag-x: {x}px; --drag-y: {y}px;")
            }
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointerleave=on_pointer_up
            on:pointercancel=on_pointer_up
            class=class
        >
            {children()}
        </div>
    }
}
