//! A crop rectangle over an image: drag it to move, drag its handles to resize.
//!
//! The geometry is [`crate::utils::image::Crop`]'s and the arithmetic is tested there; what this
//! adds is the pointer work and the ARIA. The rectangle is in fractions of the image throughout,
//! which is what lets the `<img>` be any size on screen — the drag divides pixels by the frame's
//! measured width and the result means the same thing at any zoom.
//!
//! The image's *natural* size is read off its `load` event, because that is what an aspect ratio
//! has to be judged against: a square crop is only square once both axes are back in pixels.
//!
//! Exporting a still goes through the canvas in [`crate::utils::image`]. An animated GIF cannot —
//! a canvas sees one frame — so keep the original bytes and use [`crate::utils::gif::crop_gif`].

use crate::{
    primitives::drag::{DragPoint, use_drag},
    utils::image::{Crop, CropError, CropHandle, crop_to_data_url, normalised_aspect},
};
use leptos::{context::Provider, ev, prelude::*, wasm_bindgen::JsCast};
use leptos_node_ref::AnyNodeRef;
use web_sys::HtmlImageElement;

/// How far an arrow key moves the rectangle, as a fraction of the image.
const KEY_STEP: f64 = 0.01;

/// The smallest a crop may be dragged, as a fraction. Small enough not to be in the way, large
/// enough that the handles do not land on top of each other.
const MIN_SIZE: f64 = 0.05;

#[derive(Copy, Clone)]
pub struct ImageCropperContext {
    pub crop: RwSignal<Crop>,
    /// The image's natural size, once it has loaded. `(0, 0)` until then.
    pub natural: RwSignal<(f64, f64)>,
    /// The ratio the crop is held to, as width over height *in pixels*. `None` is free-form.
    pub aspect: Option<f64>,
    pub min_size: f64,
    /// The box the image is drawn in, which is what a pointer's pixels are measured against.
    pub frame_ref: AnyNodeRef,
    pub image_ref: AnyNodeRef,
    /// The handle being dragged, or `None` for the whole rectangle.
    pub dragging: RwSignal<Option<Option<CropHandle>>>,
    /// Whether the crop is *shown* as an ellipse. Purely a preview — the rectangle underneath is
    /// the same one, and it is still what gets cut.
    pub circular: bool,
}

impl ImageCropperContext {
    /// The held ratio in the crop's own coordinates, which is not the same number as the pixel
    /// one unless the image happens to be square.
    pub fn normalised_aspect(&self) -> Option<f64> {
        let (width, height) = self.natural.get();
        self.aspect
            .map(|aspect| normalised_aspect(aspect, width, height))
    }

    /// The frame's size on screen, for turning a pointer's pixels into fractions.
    fn frame_size(&self) -> Option<(f64, f64)> {
        let rect = self.frame_ref.get_untracked()?.get_bounding_client_rect();
        (rect.width() > 0.0 && rect.height() > 0.0).then_some((rect.width(), rect.height()))
    }

    /// Applies a drag, in pixels, to the rectangle the gesture started from.
    fn apply(&self, from: Crop, handle: Option<CropHandle>, dx: f64, dy: f64) {
        let Some((width, height)) = self.frame_size() else {
            return;
        };

        let (dx, dy) = (dx / width, dy / height);

        let next = match handle {
            Some(handle) => from.resized(handle, dx, dy, self.min_size, self.normalised_aspect()),
            None => from.moved_by(dx, dy),
        };

        if next != self.crop.get_untracked() {
            self.crop.set(next);
        }
    }

    /// Sets the crop to the largest rectangle of the held ratio, centred.
    pub fn reset(&self) {
        self.crop.set(match self.normalised_aspect() {
            Some(aspect) => Crop::centered(aspect),
            None => Crop::full(),
        });
    }

    /// Brings the crop back to the held ratio, keeping its middle and as much of its size as the
    /// ratio allows. Done for you when the image loads; call it if you write a rectangle in
    /// afterwards, since a ratio is a promise the component keeps rather than a suggestion.
    pub fn conform(&self) {
        let Some(aspect) = self.normalised_aspect() else {
            return;
        };

        let conformed = self.crop.get_untracked().conformed(aspect);
        if conformed != self.crop.get_untracked() {
            self.crop.set(conformed);
        }
    }

    /// The crop as a still image, at the source's natural resolution.
    ///
    /// One frame of an animated GIF, since that is all a canvas can see; for those, keep the
    /// bytes and use [`crate::utils::gif::crop_gif`].
    pub fn to_data_url(&self, mime: &str, quality: f64) -> Result<String, CropError> {
        let image = self
            .image_ref
            .get_untracked()
            .and_then(|element| element.dyn_into::<HtmlImageElement>().ok())
            .ok_or(CropError::Empty)?;

        crop_to_data_url(&image, self.crop.get_untracked(), mime, quality)
    }

    /// The crop in the source's own pixels: `(x, y, width, height)`, rounded, for a caller doing
    /// the cutting somewhere else — on a server, or through `crop_gif`.
    pub fn to_pixels(&self) -> (f64, f64, f64, f64) {
        let (width, height) = self.natural.get();
        let (x, y, w, h) = self.crop.get().to_pixels(width, height);

        (x.round(), y.round(), w.round(), h.round())
    }
}

pub fn use_image_cropper() -> ImageCropperContext {
    expect_context::<ImageCropperContext>()
}

#[component]
pub fn ImageCropperRoot(
    /// The crop rectangle. Read it to know what was chosen; write it to set one.
    #[prop(optional, into)]
    crop: RwSignal<Crop>,
    /// The ratio to hold, as width over height in pixels. `None` is free-form.
    #[prop(default = None, into)]
    aspect: Option<f64>,
    #[prop(default = MIN_SIZE)] min_size: f64,
    /// Draws the crop as an ellipse rather than a rectangle. What gets cut is unchanged — this is
    /// a preview, for the avatars and stamps that will be shown in a circle later.
    #[prop(default = false)]
    circular: bool,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = ImageCropperContext {
        crop,
        natural: RwSignal::new((0.0, 0.0)),
        aspect,
        min_size,
        frame_ref: AnyNodeRef::new(),
        image_ref: AnyNodeRef::new(),
        dragging: RwSignal::new(None),
        circular,
    };

    view! {
        <Provider value=ctx>
            <div
                data-slot="image-cropper"
                data-circular=circular.then_some("")
                data-dragging=move || ctx.dragging.get().is_some().then_some("")
                class=class
            >
                {children()}
            </div>
        </Provider>
    }
}

/// The image, and the frame everything else is positioned inside.
#[component]
pub fn ImageCropperImageRoot(
    #[prop(into)] src: Signal<String>,
    #[prop(optional, into)] alt: Signal<String>,
    /// Whether to ask for the image with CORS. Needed for anything cross-origin that will be
    /// exported: without it the canvas is tainted and reading it back throws.
    #[prop(default = true)]
    cross_origin: bool,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] image_class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_image_cropper();

    view! {
        <div node_ref=ctx.frame_ref data-slot="image-cropper-frame" class=class>
            <img
                node_ref=ctx.image_ref
                src=move || src.get()
                alt=move || alt.get()
                crossorigin=cross_origin.then_some("anonymous")
                // Its own drag would fight the crop's.
                draggable="false"
                on:load=move |e| {
                    if let Some(image) = e
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlImageElement>().ok())
                    {
                        let natural = (
                            image.natural_width() as f64,
                            image.natural_height() as f64,
                        );
                        ctx.natural.set(natural);

                        // A ratio cannot be applied before this moment: "square" is a statement
                        // about pixels, and until the image has loaded nobody knows how many of
                        // them there are per fraction. So the rectangle is *conformed* here —
                        // the caller keeps their position and as much of their size as the ratio
                        // allows, rather than having it replaced or, worse, left at the wrong
                        // shape until the first drag snapped it.
                        ctx.conform();
                    }
                }
                class=image_class
            />
            {children()}
        </div>
    }
}

/// The crop rectangle itself: drag it to move the whole thing.
#[component]
pub fn ImageCropperWindowRoot(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_image_cropper();
    let from = StoredValue::new(Crop::full());

    let drag = use_drag(move |point: DragPoint| {
        ctx.apply(from.get_value(), None, point.dx, point.dy);
    })
    .on_end(move |_| ctx.dragging.set(None));

    let on_keydown = move |e: ev::KeyboardEvent| {
        let step = if e.shift_key() {
            KEY_STEP * 5.0
        } else {
            KEY_STEP
        };

        let (dx, dy) = match e.key().as_str() {
            "ArrowLeft" => (-step, 0.0),
            "ArrowRight" => (step, 0.0),
            "ArrowUp" => (0.0, -step),
            "ArrowDown" => (0.0, step),
            _ => return,
        };

        e.prevent_default();
        let crop = ctx.crop.get_untracked();
        ctx.crop.set(crop.moved_by(dx, dy));
    };

    view! {
        <div
            data-slot="image-cropper-window"
            role="group"
            aria-label="Crop area. Arrow keys move it."
            tabindex="0"
            style=move || {
                let crop = ctx.crop.get();
                format!(
                    "left: {}%; top: {}%; width: {}%; height: {}%; touch-action: none;",
                    crop.x * 100.0,
                    crop.y * 100.0,
                    crop.width * 100.0,
                    crop.height * 100.0,
                )
            }
            on:pointerdown=move |e| {
                // A press on a handle is the handle's; it stops the event before this sees it.
                from.set_value(ctx.crop.get_untracked());
                ctx.dragging.set(Some(None));
                drag.start(&e);
            }
            on:keydown=on_keydown
            class=class
        >
            {children()}
        </div>
    }
}

/// One of the eight handles.
#[component]
pub fn ImageCropperHandleRoot(
    handle: CropHandle,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let ctx = use_image_cropper();
    let from = StoredValue::new(Crop::full());

    let drag = use_drag(move |point: DragPoint| {
        ctx.apply(from.get_value(), Some(handle), point.dx, point.dy);
    })
    .on_end(move |_| ctx.dragging.set(None));

    view! {
        <span
            data-slot="image-cropper-handle"
            data-handle=handle.as_str()
            data-corner=handle.is_corner().then_some("")
            aria-hidden="true"
            style="touch-action: none;"
            on:pointerdown=move |e| {
                // Otherwise the window under it would start a move at the same time.
                e.stop_propagation();
                from.set_value(ctx.crop.get_untracked());
                ctx.dragging.set(Some(Some(handle)));
                drag.start(&e);
            }
            class=class
        />
    }
}

/// The frame the thirds are drawn in. The lines themselves are the caller's, because how wide a
/// line is is a styling decision — and one with a trap in it: a line given a width in *percent*
/// is a fraction of a pixel on a small crop, which browsers round away entirely and bring back on
/// zoom.
#[component]
pub fn ImageCropperGridRoot(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <span data-slot="image-cropper-grid" aria-hidden="true" class=class>
            {children.map(|children| children())}
        </span>
    }
}
