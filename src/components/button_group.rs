//! Buttons welded into one control.
//!
//! Style-only: the group flattens the corners and borders where its children meet, and
//! `src/components/button.rs` already normalises its smaller radii inside
//! `[data-slot=button-group]`, so any button size lines up.

use crate::{cn, components::separator::Separator, utils::types::Orientation};
use leptos::prelude::*;

/// Which edges get flattened where the children meet. Deliberately not an `AsClass` impl on
/// `Orientation`: these classes mean something only inside a button group.
fn seam_class(orientation: Orientation) -> &'static str {
    match orientation {
        Orientation::Horizontal => {
            "[&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none"
        }
        Orientation::Vertical => {
            "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none"
        }
    }
}

#[component]
pub fn ButtonGroup(
    #[prop(optional)] orientation: Orientation,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let orientation_class = seam_class(orientation);
    let orientation_str = orientation.as_str();

    view! {
        <div
            data-slot="button-group"
            data-orientation=orientation_str
            role="group"
            class=move || {
                cn!(
                    "flex w-fit items-stretch [&>*]:focus-visible:relative [&>*]:focus-visible:z-10 [&>input]:flex-1 has-[>[data-slot=button-group]]:gap-2",
                    orientation_class,
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

/// A label inside a group — a unit, a prefix, a count — styled like a button but inert.
#[component]
pub fn ButtonGroupText(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="button-group-text"
            class=move || {
                cn!(
                    "inline-flex h-8 shrink-0 items-center gap-1.5 rounded-lg border border-border bg-muted px-2.5 text-sm font-medium text-muted-foreground [&_svg:not([class*='size-'])]:size-4",
                    class.get()
                )
            }
        >
            {children()}
        </div>
    }
}

/// A rule between two buttons in a group, for when the flattened border is not enough on its own —
/// between two buttons of the same variant, say.
#[component]
pub fn ButtonGroupSeparator(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <Separator
            orientation=orientation
            class=move || {
                cn!(
                    "relative m-0 self-stretch bg-border data-[orientation=vertical]:h-auto",
                    class.get()
                )
            }
        />
    }
}
