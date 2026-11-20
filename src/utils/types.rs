use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;

pub use nocomponents_core::types::{
    Align, Direction, Language, Orientation, Side, SideOffset, State, TriggerRect,
};

pub trait AsClass {
    fn as_class(&self) -> &'static str;
}

/// What a styled part's `render` is handed when the caller draws the element themselves: the
/// classes the part would have rendered, and the node ref its state attributes go on.
///
/// A struct for the reason [`TriggerRender`] is one — that shape had
/// to grow, and a tuple has to be re-broken every time it does. It has since grown a `disabled`,
/// and every caller written against the two fields kept compiling, which is the whole argument.
/// The two are still separate rather than merged: a trigger is handed a click to run, and a
/// styled row has none.
#[derive(Copy, Clone)]
pub struct StyledRender {
    pub class: Signal<String>,
    pub node_ref: AnyNodeRef,
    /// Whatever the part resolved it to — `Button` folds in the field around it. Worth reading
    /// rather than ignoring: the element is the caller's, and on an `<a>`, which is what `render`
    /// mostly returns, the `disabled` attribute means nothing at all.
    pub disabled: Signal<bool>,
}

/// What a layer that opens on hover hands its `render`.
///
/// Only the node ref, and that is the honest shape: nothing here is clicked, so there is no
/// callback to hand over, and the trigger wears none of the layer's classes.
#[derive(Copy, Clone)]
pub struct AnchorRender {
    pub node_ref: AnyNodeRef,
}

/// What a trigger's `render` is handed — the floating triggers and the dialog family's alike.
///
/// A struct rather than a tuple because this is the one shape that has had to grow: it was
/// `(AnyNodeRef, Callback<MouseEvent>)` until a combobox inside a disabled `Field` needed the
/// resolved `disabled` as well. Every future addition is a new field, which no caller has to be
/// rewritten for — destructuring a tuple breaks the moment the tuple gains an element.
#[derive(Copy, Clone)]
pub struct TriggerRender {
    /// Must be attached to whatever is rendered, or the layer it opens has nothing to measure
    /// against and the ARIA has no node to be written onto.
    pub node_ref: AnyNodeRef,
    pub on_click: Callback<MouseEvent>,
    /// The trigger's own `disabled`, already resolved against the `Field` it may sit in — which is
    /// not the same as the `disabled` the caller passed.
    pub disabled: Signal<bool>,
}
