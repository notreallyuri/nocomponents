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
use leptos::{ev, prelude::*};

/// Below Tailwind's `md` breakpoint. The same line the sidebar draws between itself and its sheet.
pub const SHEET_QUERY: &str = "(max-width: 767px)";

/// Whether the layer should be a sheet right now: it asked for one, and the viewport is a phone.
///
/// Call it once per content part, inside the owner that part renders under.
pub fn use_sheet_presentation(mobile_sheet: Signal<bool>) -> Signal<bool> {
    let phone = use_media_query(SHEET_QUERY);
    Signal::derive(move || mobile_sheet.get() && phone.get())
}

/// Inline layout for the `role="menu"` or `role="listbox"` element when it sits inside a sheet's
/// frame beside the header: it fills the frame and lets a scroller inside it shrink.
pub const SHEET_ROLE_STYLE: &str =
    "display: flex; flex-direction: column; flex: 1 1 auto; min-height: 0;";

/// Closes a layer whose sheet-or-floating presentation changes while it is open.
///
/// A menu or listbox in a sheet is built differently from a floating one — its header cannot be
/// inside the role element — and that is decided when the content mounts. A window dragged across
/// the breakpoint with one open would leave the other shape's markup under the wrong classes, so
/// the layer closes and opens again in the right one.
pub fn close_when_presentation_changes(sheet: Signal<bool>, close: Callback<()>) {
    let at_mount = sheet.get_untracked();
    Effect::new(move |_| {
        if sheet.get() != at_mount {
            close.run(());
        }
    });
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

/// The grabber and the optional title and close button at the top of a sheet. Renders nothing at
/// `md` and up.
///
/// It is the one part of a sheet that can be dragged, unless `draggable` is off: `on_grab` is
/// handed the press, and the body below keeps its own scrolling. The close button is drawn when `close` is given and `show_close`
/// is on; its glyph is `close_icon`, and a layer with no styled glyph gets a plain cross. The
/// layout inside the bar is inline and structural, the look of the title and the button is the
/// styled layer's, through `data-slot="menu-sheet-title"` and `data-slot="menu-sheet-close"`.
#[component]
pub fn SheetHeader(
    sheet: Signal<bool>,
    title: Signal<String>,
    #[prop(optional, into)] on_grab: Option<Callback<ev::PointerEvent>>,
    #[prop(default = true.into(), into)] draggable: Signal<bool>,
    #[prop(optional, into)] close: Option<Callback<()>>,
    #[prop(default = true.into(), into)] show_close: Signal<bool>,
    #[prop(default = "Close".into(), into)] close_label: Signal<String>,
    #[prop(default = None, into)] close_icon: Option<ViewFn>,
) -> impl IntoView {
    let close_icon = StoredValue::new(close_icon);
    let has_close = move || close.is_some() && show_close.get();
    let has_bar = move || has_close() || !title.with(|title| title.is_empty());

    view! {
        <Show when=move || sheet.get()>
            <div
                data-slot="menu-sheet-header"
                role="presentation"
                style=move || {
                    if draggable.get() {
                        "position: sticky; top: 0; z-index: 1; background: inherit; touch-action: none; user-select: none;"
                    } else {
                        "position: sticky; top: 0; z-index: 1; background: inherit;"
                    }
                }
                on:pointerdown=move |e| {
                    if let Some(on_grab) = on_grab {
                        on_grab.run(e);
                    }
                }
            >
                <span
                    aria-hidden="true"
                    data-slot="sheet-grabber"
                    class="mx-auto my-2 block h-1 w-9 rounded-full bg-current opacity-20"
                ></span>
                <Show when=has_bar>
                    <div
                        data-slot="menu-sheet-bar"
                        style="display: flex; align-items: center; justify-content: space-between; gap: 0.5rem;"
                    >
                        <div data-slot="menu-sheet-title">{move || title.get()}</div>
                        {move || {
                            close
                                .filter(|_| show_close.get())
                                .map(|close| {
                                    view! {
                                        <button
                                            type="button"
                                            data-slot="menu-sheet-close"
                                            aria-label=move || close_label.get()
                                            style="margin-left: auto; flex-shrink: 0;"
                                            on:click=move |_| close.run(())
                                        >
                                            {close_icon
                                                .get_value()
                                                .map(|icon| icon.run())
                                                .unwrap_or_else(|| {
                                                    view! { <span aria-hidden="true">"\u{d7}"</span> }
                                                        .into_any()
                                                })}
                                        </button>
                                    }
                                })
                        }}
                    </div>
                </Show>
            </div>
        </Show>
    }
}
