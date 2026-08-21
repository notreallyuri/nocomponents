//! A crop rectangle, and the one canvas call that turns it into an image.
//!
//! **The rectangle is in normalised image coordinates** — 0 at the left or top edge of the image
//! and 1 at the right or bottom — and never in pixels. The `<img>` a cropper draws over is
//! responsive, so its pixel size is whatever the layout says this second; the output wants the
//! image's *natural* size; and a crop that has to be readable from a form wants neither. One set
//! of coordinates survives all three, and it is this one.
//!
//! Everything down to [`crop_to_data_url`] is arithmetic, and tested. The encoder is the
//! browser's: `drawImage` from the natural-size source rectangle onto a canvas of exactly that
//! size, then `toDataURL`. No image crate, no resampling of our own.
//!
//! **A canvas cannot crop an animated GIF.** `drawImage` copies whichever frame the `<img>` is
//! showing, and `toDataURL("image/gif")` is not implemented anywhere — it returns a PNG without
//! saying so. Check [`crate::utils::gif::is_gif`] on the original bytes first and send those
//! through [`crate::utils::gif::crop_gif`], which keeps every frame.

use leptos::{prelude::document, wasm_bindgen::JsCast, wasm_bindgen::JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

/// A crop rectangle, in fractions of the image it sits on.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Crop {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for Crop {
    fn default() -> Self {
        Self::full()
    }
}

/// Which corner or edge a drag has hold of.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CropHandle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl CropHandle {
    /// Every handle, clockwise from the top left — the order they are drawn in.
    pub const ALL: [CropHandle; 8] = [
        CropHandle::TopLeft,
        CropHandle::Top,
        CropHandle::TopRight,
        CropHandle::Right,
        CropHandle::BottomRight,
        CropHandle::Bottom,
        CropHandle::BottomLeft,
        CropHandle::Left,
    ];

    /// The four that move two edges at once. With a fixed aspect ratio these are the only ones
    /// worth offering: an edge on its own cannot honour a ratio without moving a side the pointer
    /// is not on.
    pub const CORNERS: [CropHandle; 4] = [
        CropHandle::TopLeft,
        CropHandle::TopRight,
        CropHandle::BottomRight,
        CropHandle::BottomLeft,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            CropHandle::TopLeft => "top-left",
            CropHandle::Top => "top",
            CropHandle::TopRight => "top-right",
            CropHandle::Right => "right",
            CropHandle::BottomRight => "bottom-right",
            CropHandle::Bottom => "bottom",
            CropHandle::BottomLeft => "bottom-left",
            CropHandle::Left => "left",
        }
    }

    pub fn moves_left(&self) -> bool {
        matches!(
            self,
            CropHandle::TopLeft | CropHandle::Left | CropHandle::BottomLeft
        )
    }

    pub fn moves_right(&self) -> bool {
        matches!(
            self,
            CropHandle::TopRight | CropHandle::Right | CropHandle::BottomRight
        )
    }

    pub fn moves_top(&self) -> bool {
        matches!(
            self,
            CropHandle::TopLeft | CropHandle::Top | CropHandle::TopRight
        )
    }

    pub fn moves_bottom(&self) -> bool {
        matches!(
            self,
            CropHandle::BottomLeft | CropHandle::Bottom | CropHandle::BottomRight
        )
    }

    pub fn is_corner(&self) -> bool {
        Self::CORNERS.contains(self)
    }
}

/// Which end of an axis a resize holds still. The opposite corner for a corner handle; the middle
/// for an edge handle, which has no opposite on the axis it does not move.
#[derive(Clone, Copy)]
enum Anchor {
    /// The low edge is fixed and the rectangle grows toward 1.
    Start(f64),
    /// The high edge is fixed and it grows toward 0.
    End(f64),
    /// The middle is fixed and it grows both ways.
    Center(f64),
}

impl Anchor {
    /// How long the rectangle may be on this axis before it leaves the image.
    fn max_len(&self) -> f64 {
        match self {
            Anchor::Start(start) => (1.0 - start).max(0.0),
            Anchor::End(end) => end.max(0.0),
            Anchor::Center(center) => 2.0 * center.min(1.0 - center).max(0.0),
        }
    }

    fn place(&self, len: f64) -> (f64, f64) {
        match self {
            Anchor::Start(start) => (*start, start + len),
            Anchor::End(end) => (end - len, *end),
            Anchor::Center(center) => (center - len / 2.0, center + len / 2.0),
        }
    }
}

/// Clamps without caring which bound is which: a minimum size larger than the room left over
/// would otherwise make `clamp` panic on an inverted range.
fn clamp_to(value: f64, low: f64, high: f64) -> f64 {
    value.max(low).min(high.max(low))
}

/// A pixel aspect ratio — what the finished image should be, width over height — expressed in the
/// normalised coordinates a [`Crop`] lives in.
///
/// The two are only the same on a square image. A square crop of a 2:1 photograph is half as wide
/// as it is tall once both are fractions of their own axis, which is why every aspect that reaches
/// [`Crop::resized`] has to come through here first.
pub fn normalised_aspect(pixel_aspect: f64, image_width: f64, image_height: f64) -> f64 {
    if image_width <= 0.0 || image_height <= 0.0 || !pixel_aspect.is_finite() {
        return pixel_aspect;
    }

    pixel_aspect * image_height / image_width
}

impl Crop {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// The whole image.
    pub const fn full() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    /// The largest rectangle of `aspect` that fits, centred. `aspect` is normalised — see
    /// [`normalised_aspect`].
    pub fn centered(aspect: f64) -> Self {
        if !(aspect.is_finite() && aspect > 0.0) {
            return Self::full();
        }

        let (width, height) = if aspect >= 1.0 {
            (1.0, 1.0 / aspect)
        } else {
            (aspect, 1.0)
        };

        Self::new((1.0 - width) / 2.0, (1.0 - height) / 2.0, width, height)
    }

    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Whether the rectangle has any area at all.
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    /// Brings a rectangle inside the image: shrunk if it is bigger, then shifted rather than
    /// trimmed, so a crop dragged off the edge keeps the size it had.
    pub fn clamped(self) -> Self {
        let width = self.width.clamp(0.0, 1.0);
        let height = self.height.clamp(0.0, 1.0);

        Self {
            x: clamp_to(self.x, 0.0, 1.0 - width),
            y: clamp_to(self.y, 0.0, 1.0 - height),
            width,
            height,
        }
    }

    /// Moves the whole rectangle. It stops at the edges of the image instead of leaving them.
    pub fn moved_by(self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            ..self
        }
        .clamped()
    }

    /// The nearest rectangle of `aspect` to this one: same centre, and as close to the same size
    /// as the ratio and the edges of the image allow.
    ///
    /// Shrinks rather than grows, so the result stays inside what was asked for. `aspect` is
    /// normalised — see [`normalised_aspect`] — which is why this cannot be done until the image
    /// has loaded and its proportions are known.
    pub fn conformed(self, aspect: f64) -> Self {
        if !(aspect.is_finite() && aspect > 0.0) {
            return self.clamped();
        }

        let (center_x, center_y) = self.center();

        let mut width = self.width.min(self.height * aspect);
        let mut height = width / aspect;

        // Then give way at the edges, keeping the ratio while doing it — the same rule a resize
        // follows.
        let scale = ((2.0 * center_x.min(1.0 - center_x)) / width)
            .min((2.0 * center_y.min(1.0 - center_y)) / height)
            .min(1.0);
        width *= scale;
        height *= scale;

        Self::new(
            center_x - width / 2.0,
            center_y - height / 2.0,
            width,
            height,
        )
        .clamped()
    }

    /// Drags one handle by `(dx, dy)`.
    ///
    /// The edges the handle owns move and the rest stay put, so the corner opposite a corner
    /// handle never shifts. `min` is the smallest the rectangle may become on either axis, and
    /// `aspect` — normalised, see [`normalised_aspect`] — holds the ratio: the rectangle then
    /// follows whichever axis the pointer pulled furthest and gives way at the edges of the image,
    /// which can leave it smaller than `min` if the ratio simply cannot fit there.
    pub fn resized(
        self,
        handle: CropHandle,
        dx: f64,
        dy: f64,
        min: f64,
        aspect: Option<f64>,
    ) -> Self {
        let min = min.clamp(0.0, 1.0);
        let (left, top, right, bottom) = (self.x, self.y, self.right(), self.bottom());

        // Free-form first: each moving edge stops `min` short of the one across from it, and at
        // the edge of the image.
        let left = if handle.moves_left() {
            clamp_to(left + dx, 0.0, right - min)
        } else {
            left
        };
        let right = if handle.moves_right() {
            clamp_to(right + dx, left + min, 1.0)
        } else {
            right
        };
        let top = if handle.moves_top() {
            clamp_to(top + dy, 0.0, bottom - min)
        } else {
            top
        };
        let bottom = if handle.moves_bottom() {
            clamp_to(bottom + dy, top + min, 1.0)
        } else {
            bottom
        };

        let Some(aspect) = aspect.filter(|aspect| aspect.is_finite() && *aspect > 0.0) else {
            return Self::new(left, top, right - left, bottom - top).clamped();
        };

        // What the drag holds still on each axis.
        let anchor_x = match (handle.moves_left(), handle.moves_right()) {
            (true, _) => Anchor::End(right),
            (_, true) => Anchor::Start(left),
            _ => Anchor::Center((left + right) / 2.0),
        };
        let anchor_y = match (handle.moves_top(), handle.moves_bottom()) {
            (true, _) => Anchor::End(bottom),
            (_, true) => Anchor::Start(top),
            _ => Anchor::Center((top + bottom) / 2.0),
        };

        // Follow the pointer: take whichever axis it pulled furthest and derive the other from
        // the ratio, rather than averaging the two into something that tracks neither.
        let mut width = (right - left).max(min);
        let mut height = (bottom - top).max(min);
        width = width.max(height * aspect);
        height = width / aspect;

        // Then give way at the edges of the image, keeping the ratio while doing it.
        let scale = (anchor_x.max_len() / width)
            .min(anchor_y.max_len() / height)
            .min(1.0);
        width *= scale;
        height *= scale;

        let (left, _) = anchor_x.place(width);
        let (top, _) = anchor_y.place(height);

        Self::new(left, top, width, height).clamped()
    }

    /// The source rectangle in an image's own pixels: `(x, y, width, height)`, which is exactly
    /// what `drawImage` takes.
    pub fn to_pixels(&self, image_width: f64, image_height: f64) -> (f64, f64, f64, f64) {
        (
            self.x * image_width,
            self.y * image_height,
            self.width * image_width,
            self.height * image_height,
        )
    }

    /// The reverse, for a crop that arrives already measured in pixels.
    pub fn from_pixels(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        image_width: f64,
        image_height: f64,
    ) -> Self {
        if image_width <= 0.0 || image_height <= 0.0 {
            return Self::full();
        }

        Self::new(
            x / image_width,
            y / image_height,
            width / image_width,
            height / image_height,
        )
        .clamped()
    }
}

/// Why a crop could not be turned into an image.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CropError {
    /// The image has not loaded, or the rectangle came out smaller than a pixel.
    Empty,
    /// The browser would not give us a 2D context.
    NoContext,
    /// The canvas is tainted, which happens when the image came from another origin without
    /// permission. Set `crossorigin="anonymous"` on the `<img>` *and* have the host send
    /// `Access-Control-Allow-Origin`; one without the other still taints it.
    Tainted,
}

/// Cuts `crop` out of `image` and returns it as a data URL.
///
/// The output is the crop at the image's natural resolution — no scaling, so nothing is resampled
/// on the way out. `mime` is a canvas type such as `image/png` or `image/jpeg`, and `quality`
/// (0–1) only means anything to the lossy ones.
///
/// For an animated GIF this returns one still frame, because that is all a canvas can see. Use
/// [`crate::utils::gif::crop_gif`] on the original bytes instead.
///
/// The only part of this module a test cannot reach: it needs a document, a real decoded image and
/// a canvas the browser is willing to read back.
pub fn crop_to_data_url(
    image: &HtmlImageElement,
    crop: Crop,
    mime: &str,
    quality: f64,
) -> Result<String, CropError> {
    let natural_width = image.natural_width() as f64;
    let natural_height = image.natural_height() as f64;
    let (x, y, width, height) = crop.clamped().to_pixels(natural_width, natural_height);

    // A canvas cannot be zero-sized, and half a pixel of crop is not an image.
    if width < 1.0 || height < 1.0 {
        return Err(CropError::Empty);
    }

    let canvas = document()
        .create_element("canvas")
        .ok()
        .and_then(|element| element.dyn_into::<HtmlCanvasElement>().ok())
        .ok_or(CropError::NoContext)?;

    canvas.set_width(width.round() as u32);
    canvas.set_height(height.round() as u32);

    let context = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|context| context.dyn_into::<CanvasRenderingContext2d>().ok())
        .ok_or(CropError::NoContext)?;

    context
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image, x, y, width, height, 0.0, 0.0, width, height,
        )
        .map_err(|_| CropError::Empty)?;

    canvas
        .to_data_url_with_type_and_encoder_options(
            mime,
            &JsValue::from_f64(quality.clamp(0.0, 1.0)),
        )
        .map_err(|_| CropError::Tainted)
}

/// What these bytes are, from their first few. Enough to label a blob correctly and — more to the
/// point — to tell a GIF from a still, which is the difference between cropping through a canvas
/// and cropping through [`crate::utils::gif::crop_gif`].
pub fn sniff_mime(bytes: &[u8]) -> Option<&'static str> {
    match bytes {
        [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, ..] => Some("image/png"),
        [0xff, 0xd8, 0xff, ..] => Some("image/jpeg"),
        [b'G', b'I', b'F', b'8', ..] => Some("image/gif"),
        [
            b'R',
            b'I',
            b'F',
            b'F',
            _,
            _,
            _,
            _,
            b'W',
            b'E',
            b'B',
            b'P',
            ..,
        ] => Some("image/webp"),
        _ => None,
    }
}

/// Wraps bytes in a blob and hands back a URL an `<img>` can load.
///
/// Not a data URL: those carry the whole file in the markup, base64-inflated by a third, and a
/// cropped GIF is not small. This is a pointer into memory instead — which is why it has to be
/// handed back with [`revoke_object_url`] when the image is finished with, or the bytes stay
/// alive for the life of the document.
pub fn to_object_url(bytes: &[u8], mime: &str) -> Option<String> {
    let array = js_sys::Uint8Array::from(bytes);
    let parts = js_sys::Array::of1(&array);

    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime);

    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options).ok()?;

    web_sys::Url::create_object_url_with_blob(&blob).ok()
}

/// Releases a URL from [`to_object_url`].
pub fn revoke_object_url(url: &str) {
    let _ = web_sys::Url::revoke_object_url(url);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn same(crop: Crop, x: f64, y: f64, width: f64, height: f64) -> bool {
        close(crop.x, x)
            && close(crop.y, y)
            && close(crop.width, width)
            && close(crop.height, height)
    }

    #[test]
    fn a_pixel_ratio_is_not_a_normalised_one() {
        // A square crop of a 2:1 image is half as wide as it is tall, in fractions.
        assert!(close(normalised_aspect(1.0, 2000.0, 1000.0), 0.5));
        // On a square image the two agree.
        assert!(close(normalised_aspect(1.0, 800.0, 800.0), 1.0));
        // 16:9 of a portrait image comes out wider than 16:9 of a square one.
        assert!(close(
            normalised_aspect(16.0 / 9.0, 1000.0, 2000.0),
            32.0 / 9.0
        ));
    }

    #[test]
    fn centres_the_largest_rectangle_that_fits() {
        assert!(same(Crop::centered(1.0), 0.0, 0.0, 1.0, 1.0));
        assert!(same(Crop::centered(2.0), 0.0, 0.25, 1.0, 0.5));
        assert!(same(Crop::centered(0.5), 0.25, 0.0, 0.5, 1.0));
        // Nonsense falls back to the whole image rather than to an empty rectangle.
        assert!(same(Crop::centered(0.0), 0.0, 0.0, 1.0, 1.0));
        assert!(same(Crop::centered(f64::NAN), 0.0, 0.0, 1.0, 1.0));
    }

    #[test]
    fn moving_stops_at_the_edges_without_shrinking() {
        let crop = Crop::new(0.4, 0.4, 0.2, 0.2);

        assert!(same(crop.moved_by(0.1, 0.1), 0.5, 0.5, 0.2, 0.2));
        // Shoved off the right and bottom: it keeps its size and rests against the edge.
        assert!(same(crop.moved_by(5.0, 5.0), 0.8, 0.8, 0.2, 0.2));
        assert!(same(crop.moved_by(-5.0, -5.0), 0.0, 0.0, 0.2, 0.2));
    }

    #[test]
    fn a_corner_moves_two_edges_and_leaves_the_opposite_one() {
        let crop = Crop::new(0.2, 0.2, 0.6, 0.6);

        let dragged = crop.resized(CropHandle::TopLeft, 0.1, 0.1, 0.05, None);
        assert!(same(dragged, 0.3, 0.3, 0.5, 0.5));
        // The corner it is anchored against has not moved.
        assert!(close(dragged.right(), crop.right()) && close(dragged.bottom(), crop.bottom()));

        let dragged = crop.resized(CropHandle::BottomRight, 0.1, 0.05, 0.05, None);
        assert!(same(dragged, 0.2, 0.2, 0.7, 0.65));
    }

    #[test]
    fn an_edge_moves_only_its_own_axis() {
        let crop = Crop::new(0.2, 0.2, 0.6, 0.6);

        let dragged = crop.resized(CropHandle::Right, 0.1, 0.4, 0.05, None);
        assert!(same(dragged, 0.2, 0.2, 0.7, 0.6), "{dragged:?}");

        let dragged = crop.resized(CropHandle::Top, 0.4, -0.1, 0.05, None);
        assert!(same(dragged, 0.2, 0.1, 0.6, 0.7), "{dragged:?}");
    }

    #[test]
    fn a_handle_stops_at_the_image_and_at_the_minimum() {
        let crop = Crop::new(0.2, 0.2, 0.6, 0.6);

        // Hauled past the far edge: it stops `min` short of it rather than inverting.
        let squashed = crop.resized(CropHandle::Left, 5.0, 0.0, 0.1, None);
        assert!(same(squashed, 0.7, 0.2, 0.1, 0.6), "{squashed:?}");

        // And past the edge of the image the other way.
        let stretched = crop.resized(CropHandle::Left, -5.0, 0.0, 0.1, None);
        assert!(same(stretched, 0.0, 0.2, 0.8, 0.6), "{stretched:?}");
    }

    #[test]
    fn a_ratio_holds_through_a_corner_drag() {
        let crop = Crop::centered(1.0).resized(CropHandle::BottomRight, 0.0, 0.0, 0.05, Some(1.0));
        assert!(close(crop.width / crop.height, 1.0));

        // Pulled mostly sideways: the height follows the width, and the top left stays put.
        let crop = Crop::new(0.1, 0.1, 0.4, 0.4);
        let dragged = crop.resized(CropHandle::BottomRight, 0.2, 0.0, 0.05, Some(1.0));
        assert!(same(dragged, 0.1, 0.1, 0.6, 0.6), "{dragged:?}");

        // A 2:1 ratio: twice as wide as it is tall, whichever axis was pulled.
        let dragged = crop.resized(CropHandle::BottomRight, 0.0, 0.2, 0.05, Some(2.0));
        assert!(close(dragged.width / dragged.height, 2.0), "{dragged:?}");
        assert!(
            close(dragged.x, 0.1) && close(dragged.y, 0.1),
            "{dragged:?}"
        );
    }

    #[test]
    fn a_ratio_gives_way_at_the_edge_rather_than_breaking() {
        // Hauled far past the corner of the image: it grows until one axis runs out, and the
        // ratio is still exact.
        let crop = Crop::new(0.1, 0.1, 0.2, 0.2);
        let dragged = crop.resized(CropHandle::BottomRight, 5.0, 5.0, 0.05, Some(2.0));

        assert!(close(dragged.width / dragged.height, 2.0), "{dragged:?}");
        assert!(
            dragged.right() <= 1.0 + 1e-9 && dragged.bottom() <= 1.0 + 1e-9,
            "{dragged:?}"
        );
        // The anchored corner has not moved.
        assert!(
            close(dragged.x, 0.1) && close(dragged.y, 0.1),
            "{dragged:?}"
        );
    }

    #[test]
    fn an_edge_with_a_ratio_grows_from_the_middle_of_the_other_axis() {
        // The axis the handle does not own has no opposite edge to anchor against, so it opens
        // both ways from where it was.
        let crop = Crop::new(0.2, 0.4, 0.6, 0.2);
        let dragged = crop.resized(CropHandle::Bottom, 0.0, 0.1, 0.05, Some(1.0));

        assert!(close(dragged.width / dragged.height, 1.0), "{dragged:?}");
        assert!(close(dragged.y, 0.4), "{dragged:?}");
        // Still centred on the same column it started in.
        assert!(close(dragged.center().0, crop.center().0), "{dragged:?}");
    }

    #[test]
    fn every_result_stays_inside_the_image() {
        let crop = Crop::new(0.3, 0.3, 0.4, 0.4);

        for handle in CropHandle::ALL {
            for (dx, dy) in [(2.0, 2.0), (-2.0, -2.0), (2.0, -2.0), (-2.0, 2.0)] {
                for aspect in [None, Some(1.0), Some(3.0), Some(0.25)] {
                    let dragged = crop.resized(handle, dx, dy, 0.05, aspect);

                    assert!(
                        dragged.x >= -1e-9
                            && dragged.y >= -1e-9
                            && dragged.right() <= 1.0 + 1e-9
                            && dragged.bottom() <= 1.0 + 1e-9
                            && dragged.width >= 0.0
                            && dragged.height >= 0.0,
                        "{handle:?} by ({dx}, {dy}) at {aspect:?} left the image: {dragged:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn conforming_holds_the_ratio_without_moving_the_middle() {
        // The case that started this: a square crop asked for on a 16:9 image. In fractions that
        // is *not* a square, and 0.7 × 0.7 stays 16:9 unless something says otherwise.
        let aspect = normalised_aspect(1.0, 2804.0, 1578.0);
        let conformed = Crop::new(0.15, 0.15, 0.7, 0.7).conformed(aspect);

        // Square once both axes are back in pixels, which is what "1.0" was asking for.
        let (_, _, width, height) = conformed.to_pixels(2804.0, 1578.0);
        assert!((width / height - 1.0).abs() < 1e-9, "{conformed:?}");

        // Same middle, and no bigger than it was.
        assert!(close(conformed.center().0, 0.5) && close(conformed.center().1, 0.5));
        assert!(conformed.width <= 0.7 + 1e-9 && conformed.height <= 0.7 + 1e-9);
    }

    #[test]
    fn conforming_settles_and_stays_inside() {
        for aspect in [0.25, 0.5628, 1.0, 2.0, 4.0] {
            for crop in [
                Crop::full(),
                Crop::new(0.0, 0.0, 0.3, 0.9),
                Crop::new(0.8, 0.8, 0.2, 0.2),
                Crop::new(0.45, 0.45, 0.1, 0.1),
            ] {
                let once = crop.conformed(aspect);
                let twice = once.conformed(aspect);

                assert!(
                    (once.width / once.height - aspect).abs() < 1e-9,
                    "{crop:?} at {aspect} came out {once:?}"
                );
                assert!(
                    once.x >= -1e-9
                        && once.y >= -1e-9
                        && once.right() <= 1.0 + 1e-9
                        && once.bottom() <= 1.0 + 1e-9,
                    "{once:?} left the image"
                );
                // Conforming something already conformed changes nothing, so applying it on every
                // load cannot walk the rectangle away.
                assert!(
                    same(twice, once.x, once.y, once.width, once.height),
                    "{once:?}"
                );
            }
        }

        // A rectangle already of the right shape is left exactly where it is.
        let square = Crop::new(0.2, 0.3, 0.4, 0.4);
        assert!(same(square.conformed(1.0), 0.2, 0.3, 0.4, 0.4));
    }

    #[test]
    fn conforming_the_whole_image_is_the_largest_centred_rectangle() {
        // Which makes it the one code path: no separate "fit it for the first time" rule.
        for aspect in [0.5, 1.0, 2.5] {
            let conformed = Crop::full().conformed(aspect);
            let centered = Crop::centered(aspect);

            assert!(
                same(
                    conformed,
                    centered.x,
                    centered.y,
                    centered.width,
                    centered.height
                ),
                "{aspect}: {conformed:?} vs {centered:?}"
            );
        }
    }

    #[test]
    fn converts_to_the_source_rectangle_drawimage_wants() {
        let crop = Crop::new(0.25, 0.5, 0.5, 0.25);
        let (x, y, width, height) = crop.to_pixels(1600.0, 1200.0);

        assert_eq!((x, y, width, height), (400.0, 600.0, 800.0, 300.0));
        assert!(same(
            Crop::from_pixels(400.0, 600.0, 800.0, 300.0, 1600.0, 1200.0),
            0.25,
            0.5,
            0.5,
            0.25
        ));
    }

    #[test]
    fn a_crop_that_arrives_out_of_bounds_is_brought_back() {
        assert!(same(
            Crop::new(-0.2, 0.9, 0.5, 0.5).clamped(),
            0.0,
            0.5,
            0.5,
            0.5
        ));
        // Bigger than the image: shrunk to fit rather than hanging off it.
        assert!(same(
            Crop::new(0.5, 0.5, 2.0, 2.0).clamped(),
            0.0,
            0.0,
            1.0,
            1.0
        ));
        assert!(Crop::new(0.0, 0.0, 0.0, 0.5).is_empty());
    }
}
