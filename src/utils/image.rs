//! A crop rectangle, and the one canvas call that turns it into an image.
//!
//! **The rectangle is in normalised image coordinates** — 0 at the left or top edge of the image
//! and 1 at the right or bottom — and never in pixels. The `<img>` a cropper draws over is
//! responsive, so its pixel size is whatever the layout says this second; the output wants the
//! image's *natural* size; and a crop that has to be readable from a form wants neither. One set
//! of coordinates survives all three, and it is this one.
//!
//! Everything down to [`crop_image`] is arithmetic, and tested. The encoder is the browser's:
//! `drawImage` from the natural-size source rectangle onto a canvas of exactly that size, then
//! `toDataURL`. No image crate, no resampling of our own.
//!
//! **A canvas cannot crop an animated GIF.** `drawImage` copies whichever frame the `<img>` is
//! showing, and `toDataURL("image/gif")` is not implemented anywhere — it returns a PNG without
//! saying so. That is a fact about canvases, though, not a second API: [`crop_image`] takes the
//! original bytes and routes a GIF through [`crate::utils::gif::crop_gif`] itself, so a caller
//! that accepts both kinds of upload writes one call and not a type check.

use crate::utils::gif::{self, GifError};
use leptos::{
    prelude::document,
    task::spawn_local,
    wasm_bindgen::{JsCast, JsValue},
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

pub use nocomponents_core::image::*;

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
    /// The canvas produced a data URL that could not be read back into bytes. Nothing a caller
    /// can do about it, and nothing that has been seen to happen — it is here because unwrapping
    /// the browser's own output is not something to `unwrap`.
    Unreadable,
    /// The source was an animated GIF, and re-cropping it failed. Carries the reason.
    Gif(GifError),
}

/// A finished crop: the file itself, and what it turned out to be.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cropped {
    pub bytes: Vec<u8>,
    /// What was produced, which is not always what was asked for. A canvas silently hands back a
    /// PNG when it does not know the type it was given — `toDataURL("image/gif")` does exactly
    /// that — and an animated source comes back as `image/gif` whatever `mime` said. This is read
    /// off the output rather than echoed from the request, so it is the truth.
    pub mime: String,
}

impl Cropped {
    /// A URL an `<img>` can load. Hand it back to [`revoke_object_url`] when the image is
    /// finished with, exactly as with [`to_object_url`] — which is what this is.
    pub fn to_object_url(&self) -> Option<String> {
        to_object_url(&self.bytes, &self.mime)
    }
}

/// Cuts `crop` out of an image and hands back the file, animation and all.
///
/// This is the one to call. Whether a GIF keeps moving is decided in two places, and neither of
/// them is at the call site:
///
/// 1. `source` is the bytes the image was decoded from, when the caller still has them. An `<img>`
///    cannot give them back and a canvas cannot see past the frame it is showing, so those bytes
///    are the only place an animation exists — having them *is* the capability. Passing `None`
///    is how a caller says "a still, whatever this is", which is the right answer for an avatar
///    with a size budget.
/// 2. Given the bytes, what they actually are decides the path: a GIF goes through
///    [`crate::utils::gif::crop_gif`], which keeps every frame, its palette and its timing;
///    anything else goes through the canvas below.
///
/// So an app that lets people upload either writes one call and gets the right thing, instead of
/// sniffing the type itself and maintaining two branches at every crop.
///
/// `mime` and `quality` describe the *still* path only — an animated source ignores both, since
/// there is nothing to re-encode it as.
pub fn crop_image(
    image: &HtmlImageElement,
    source: Option<&[u8]>,
    crop: Crop,
    mime: &str,
    quality: f64,
) -> Result<Cropped, CropError> {
    if let Some(bytes) = source.filter(|bytes| gif::is_gif(bytes)) {
        return Ok(Cropped {
            bytes: gif::crop_gif(bytes, crop).map_err(CropError::Gif)?,
            mime: "image/gif".to_string(),
        });
    }

    let url = crop_to_data_url(image, crop, mime, quality)?;
    let (produced, bytes) = from_data_url(&url).ok_or(CropError::Unreadable)?;

    Ok(Cropped {
        bytes,
        // An empty media type in a data URL means `text/plain`, which a canvas will never mean.
        mime: if produced.is_empty() {
            mime.to_string()
        } else {
            produced
        },
    })
}

/// Cuts `crop` out of `image` and returns it as a data URL.
///
/// The output is the crop at the image's natural resolution — no scaling, so nothing is resampled
/// on the way out. `mime` is a canvas type such as `image/png` or `image/jpeg`, and `quality`
/// (0–1) only means anything to the lossy ones.
///
/// The canvas path on its own: for an animated GIF this returns one still frame, because that is
/// all a canvas can see. [`crop_image`] is the one that keeps the animation.
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

/// Reads a `Blob` — a picked `File` is one — and hands its bytes to `on_bytes`.
///
/// The other end of [`to_object_url`], and the piece an avatar picker needs before it can crop
/// anything: a `<input type="file">` gives a `File`, and everything else in this module works in
/// bytes. Reading one is asynchronous in every browser, so this takes a callback rather than
/// pretending otherwise; `on_bytes` runs once, on the main thread, or never if the read fails —
/// a file that was moved or unplugged between the pick and the read.
///
/// ```ignore
/// read_bytes(&file, move |bytes| {
///     let mime = sniff_mime(&bytes).unwrap_or("image/png");
///     source.set(to_object_url(&bytes, mime).unwrap_or_default());
///     picked.set(Some(bytes));
/// });
/// ```
pub fn read_bytes(blob: &Blob, on_bytes: impl FnOnce(Vec<u8>) + 'static) {
    // The await is kept in here rather than handed to the caller: every consumer of this would
    // otherwise write the same `spawn_local` around the same three lines, which is the layer this
    // module exists to remove.
    let reading = blob.array_buffer();

    spawn_local(async move {
        let Ok(buffer) = JsFuture::from(reading).await else {
            return;
        };
        let Ok(buffer) = buffer.dyn_into::<js_sys::ArrayBuffer>() else {
            return;
        };

        on_bytes(js_sys::Uint8Array::new(&buffer).to_vec());
    });
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
