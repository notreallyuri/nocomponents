//! The pieces every layer shares when it shows itself as a bottom sheet on a phone.
//!
//! A sheet is not a different component but a different *presentation* of the same content: the
//! same portal, the same items, the same focus management and dismissal, anchored to the bottom
//! edge over a scrim instead of positioned against a trigger. So each layer keeps its own tree and
//! asks [`use_sheet_presentation`] which way to draw it; what changes is written as
//! `data-presentation="sheet" | "floating"` for the styled layer to key one class string on.
//!
//! The switch is a media query rather than a class because the floating layers must not run their
//! positioning at all in a sheet, and a dialog or a sheet is not something CSS can pick between.

use crate::utils::use_media_query;
use leptos::prelude::*;

/// Below Tailwind's `md` breakpoint. The same line the sidebar draws between itself and its sheet.
pub const SHEET_QUERY: &str = "(max-width: 767px)";

/// Whether the layer should be a sheet right now: it asked for one, and the viewport is a phone.
///
/// Call it once per content part, inside the owner that part renders under.
pub fn use_sheet_presentation(mobile_sheet: Signal<bool>) -> Signal<bool> {
    let phone = use_media_query(SHEET_QUERY);
    Signal::derive(move || mobile_sheet.get() && phone.get())
}

/// The value of `data-presentation` for a layer that is, or is not, a sheet.
pub fn presentation_attr(sheet: bool) -> &'static str {
    if sheet { "sheet" } else { "floating" }
}

/// Dims the page behind a floating layer shown as a sheet. A press on it is an outside press, so
/// the layer's dismissable bounds close it without a handler of its own.
///
/// A dialog does not need this: its overlay is already a scrim.
#[component]
pub fn SheetScrim(sheet: Signal<bool>, open: Signal<bool>) -> impl IntoView {
    view! {
        <Show when=move || sheet.get()>
            <div
                aria-hidden="true"
                data-slot="menu-sheet-scrim"
                data-state=move || if open.get() { "open" } else { "closed" }
                class="fixed inset-0 z-50 bg-black/40 animation-duration-100 fill-mode-forwards data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:pointer-events-none"
            ></div>
        </Show>
    }
}

/// The grabber and the optional title row at the top of a sheet. Renders nothing at `md` and up.
#[component]
pub fn SheetHeader(sheet: Signal<bool>, title: Signal<String>) -> impl IntoView {
    view! {
        <Show when=move || sheet.get()>
            <div data-slot="menu-sheet-header" role="presentation">
                <span
                    aria-hidden="true"
                    data-slot="sheet-grabber"
                    class="mx-auto my-2 block h-1 w-9 rounded-full bg-current opacity-20"
                ></span>
                <Show when=move || !title.with(|title| title.is_empty())>
                    <div data-slot="menu-sheet-title" class="px-2 pb-2 text-sm font-medium">
                        {move || title.get()}
                    </div>
                </Show>
            </div>
        </Show>
    }
}
