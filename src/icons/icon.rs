//! The one wrapper every icon goes through, the library's own and the project's alike.
//!
//! It assumes nothing about the artwork. The viewBox is the icon's own (Lucide draws on
//! `0 0 24 24`, a Font Awesome file on `0 0 512 512` or `0 0 448 512`, one per icon), the paint
//! is `fill="currentColor"` unless `stroke_width` asks for an outline, and the children are any
//! SVG at all — a duotone icon's two `<path>`s and the opacity on the second come through as
//! they are written.
//!
//! **Size** is `1em` square, written as the `width` and `height` *attributes* rather than a
//! class. A presentation attribute loses to any CSS that targets the element, so the size a
//! component sets (`[&_svg]:size-4` on a button, a slot's `[&>svg]:size-full`) or a class the
//! caller passes always wins, and an icon that nothing sizes is as tall as the text beside it.
//! A non-square viewBox is centred in that square (`xMidYMid meet`, the SVG default), which is
//! what Font Awesome calls a fixed-width icon; pass `class="w-auto"` for the narrow one.
//!
//! **Colour** is `currentColor`, so an icon is whatever colour the text around it is.
//!
//! **Accessibility**: decorative unless labelled. With no `label` the element is `aria-hidden`
//! and out of the tab order; with one it is `role="img"` named by it — for an icon that is the
//! only content of something, though a button is better named on the button (`aria_label`).

use leptos::prelude::*;

#[component]
pub fn Icon(
    /// The artwork's coordinate system, copied from the icon's own `<svg>`. Required: there is no
    /// viewBox that is right for every icon set.
    #[prop(into)]
    view_box: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    /// The icon's accessible name. Empty (the default) makes it decorative.
    #[prop(optional, into)]
    label: Signal<String>,
    /// Draw the children as outlines of this width, in viewBox units — the shape Lucide, Feather
    /// and Tabler share. Unset, they are filled, which is how Font Awesome and most others draw.
    #[prop(default = None, into)]
    stroke_width: Option<f64>,
    children: Children,
) -> impl IntoView {
    let labelled = move || !label.with(String::is_empty);

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            data-slot="icon"
            viewBox=view_box
            width="1em"
            height="1em"
            fill=if stroke_width.is_some() { "none" } else { "currentColor" }
            stroke=stroke_width.map(|_| "currentColor")
            stroke-width=stroke_width
            stroke-linecap=stroke_width.map(|_| "square")
            stroke-linejoin=stroke_width.map(|_| "miter")
            focusable="false"
            role=move || labelled().then_some("img")
            aria-label=move || labelled().then(|| label.get())
            aria-hidden=move || (!labelled()).then_some("true")
            class=class
        >
            {children()}
        </svg>
    }
}
