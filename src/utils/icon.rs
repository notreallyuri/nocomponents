//! The one shape an icon slot takes, in every component that draws a glyph.
//!
//! A component that shows a chevron, a check, a close cross or a play button takes it as
//! `icon: Option<ViewFn>` (or `<state>_icon` where one part swaps glyphs), and draws it through
//! [`IconSlot`]: the caller's view when there is one, the component's own fallback when there is
//! not. The fallbacks are the glyphs in `icons/`, which a project has installed as source and can
//! redraw for every component at once.
//!
//! The slot is a `<span>` the component sizes, colours, rotates and hides — the caller's markup
//! is never asked to carry those classes, which is what lets an icon from any set, at any
//! viewBox, look the same as the one it replaces. The `<svg>` inside fills the span
//! (`[&>svg]:size-full` in the styled layer). It is `aria-hidden`: a glyph in a slot is decoration
//! beside a label, and the part it sits in carries the accessible name.

use leptos::prelude::*;

/// The caller's icon, or the fallback, in a span the component styles.
#[component]
pub fn IconSlot(
    /// The caller's icon. `None` draws `fallback`.
    #[prop(into)]
    icon: Option<ViewFn>,
    /// What the component draws when the caller passed nothing.
    #[prop(into)]
    fallback: ViewFn,
    /// Written as `data-slot`, so a stylesheet can reach one slot and not the others.
    #[prop(optional)]
    data_slot: &'static str,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <span data-slot=(!data_slot.is_empty()).then_some(data_slot) aria-hidden="true" class=class>
            {icon.unwrap_or(fallback).run()}
        </span>
    }
}

/// The caller's icon, or `fallback`, as the `ViewFn` a part's `icon` prop takes.
///
/// A composite that holds an `Option<ViewFn>` for a glyph it does not draw itself cannot hand the
/// option to the inner part: those props are `optional`, so their setter takes the view and not
/// the `Option`. This is the one place that unwraps it. `fallback` must be the glyph the inner
/// part would have drawn.
pub fn icon_or(icon: Option<ViewFn>, fallback: impl Into<ViewFn>) -> ViewFn {
    icon.unwrap_or_else(|| fallback.into())
}
