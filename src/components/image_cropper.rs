use crate::{
    cn,
    primitives::image_cropper::{
        ImageCropperGridRoot, ImageCropperHandleRoot, ImageCropperImageRoot, ImageCropperRoot,
        ImageCropperWindowRoot, use_image_cropper,
    },
    utils::image::{Crop, CropHandle},
};
use leptos::prelude::*;

/// An image with a crop rectangle over it.
///
/// One component, like `Calendar`: the frame, the rectangle and its handles are the same three
/// pieces every time. `primitives::image_cropper` has them apart, and `use_image_cropper()` is how
/// anything inside reaches the crop — including the export, which is a method on the context
/// rather than a prop, since when to cut is the caller's decision.
#[component]
pub fn ImageCropper(
    #[prop(into)] src: Signal<String>,
    #[prop(optional, into)] alt: Signal<String>,
    /// The crop rectangle, in fractions of the image. Read it, or write one in.
    #[prop(optional, into)]
    crop: RwSignal<Crop>,
    /// The ratio to hold, width over height in pixels. `None` is free-form.
    #[prop(default = None, into)]
    aspect: Option<f64>,
    /// The thirds over the crop, which is what most people line a photograph up against.
    #[prop(default = true)]
    grid: bool,
    /// Shows the crop as a circle. Only a preview — the rectangle underneath is unchanged and is
    /// still what gets cut, so pair it with `aspect=Some(1.0)` unless an ellipse is what you meant.
    #[prop(default = false)]
    circular: bool,
    #[prop(default = true)] cross_origin: bool,
    #[prop(optional, into)] class: Signal<String>,
    /// Rendered under the image, inside the cropper's context — which is how a toolbar reaches
    /// the crop and the export without the component having to grow a prop for each of them.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <ImageCropperRoot
            crop=crop
            aspect=aspect
            circular=circular
            // A group, so the parts below can follow `data-circular` without each being told
            // about it.
            class=move || cn!("group/cropper w-full select-none", class.get())
        >
            <ImageCropperImageRoot
                src=src
                alt=alt
                cross_origin=cross_origin
                class="relative w-full overflow-hidden rounded-lg bg-muted"
                image_class="block w-full"
            >
                <Shade />
                <Window aspect=aspect grid=grid />
            </ImageCropperImageRoot>

            {children.map(|children| children())}
        </ImageCropperRoot>
    }
}

/// The dimming outside the crop. One element with a vast spread shadow: the alternative is four
/// boxes to keep in step with the rectangle's edges.
#[component]
fn Shade() -> impl IntoView {
    let ctx = use_image_cropper();

    view! {
        <div
            aria-hidden="true"
            class="pointer-events-none absolute group-data-circular/cropper:rounded-full"
            style=move || {
                let crop = ctx.crop.get();
                format!(
                    "left: {}%; top: {}%; width: {}%; height: {}%; box-shadow: 0 0 0 9999px rgb(0 0 0 / 0.5);",
                    crop.x * 100.0,
                    crop.y * 100.0,
                    crop.width * 100.0,
                    crop.height * 100.0,
                )
            }
        />
    }
}

#[component]
fn Window(aspect: Option<f64>, grid: bool) -> impl IntoView {
    // With a ratio held, an edge handle would have to move a side the pointer is not on, so only
    // the corners are offered.
    let handles = if aspect.is_some() {
        CropHandle::CORNERS.as_slice()
    } else {
        CropHandle::ALL.as_slice()
    };

    view! {
        <ImageCropperWindowRoot class="absolute cursor-move outline-none ring-1 ring-white/90 group-data-circular/cropper:rounded-full focus-visible:ring-3 focus-visible:ring-ring">
            {grid
                .then(|| {
                    view! {
                        // Four hairlines rather than gradient stops. A stop pair at 33.33% and
                        // 33.5% is 0.17% of the crop's width — half a pixel on a wide one and a
                        // fifth of a pixel on a narrow one, which is why the thirds used to fade
                        // out and come back as the cropper was resized or the page zoomed. `w-px`
                        // is one pixel at any size.
                        <ImageCropperGridRoot class="pointer-events-none absolute inset-0 overflow-hidden group-data-circular/cropper:rounded-full">
                            <span class="absolute inset-y-0 left-1/3 w-px bg-white/35" />
                            <span class="absolute inset-y-0 left-2/3 w-px bg-white/35" />
                            <span class="absolute inset-x-0 top-1/3 h-px bg-white/35" />
                            <span class="absolute inset-x-0 top-2/3 h-px bg-white/35" />
                        </ImageCropperGridRoot>
                    }
                })}

            {handles
                .iter()
                .map(|handle| {
                    view! {
                        <ImageCropperHandleRoot
                            handle=*handle
                            class=cn!(
                                "absolute bg-white shadow-[0_0_0_1px_rgba(0,0,0,0.35)]",
                                // Corners are square blocks at the corners; edges are bars along
                                // the middle of their own side.
                                "data-corner:size-3 data-corner:rounded-xs",
                                "data-[handle=top-left]:-top-1.5 data-[handle=top-left]:-left-1.5 data-[handle=top-left]:cursor-nwse-resize",
                                "data-[handle=top-right]:-top-1.5 data-[handle=top-right]:-right-1.5 data-[handle=top-right]:cursor-nesw-resize",
                                "data-[handle=bottom-right]:-right-1.5 data-[handle=bottom-right]:-bottom-1.5 data-[handle=bottom-right]:cursor-nwse-resize",
                                "data-[handle=bottom-left]:-bottom-1.5 data-[handle=bottom-left]:-left-1.5 data-[handle=bottom-left]:cursor-nesw-resize",
                                "data-[handle=top]:-top-1 data-[handle=top]:left-1/2 data-[handle=top]:h-2 data-[handle=top]:w-6 data-[handle=top]:-translate-x-1/2 data-[handle=top]:cursor-ns-resize",
                                "data-[handle=bottom]:-bottom-1 data-[handle=bottom]:left-1/2 data-[handle=bottom]:h-2 data-[handle=bottom]:w-6 data-[handle=bottom]:-translate-x-1/2 data-[handle=bottom]:cursor-ns-resize",
                                "data-[handle=left]:top-1/2 data-[handle=left]:-left-1 data-[handle=left]:h-6 data-[handle=left]:w-2 data-[handle=left]:-translate-y-1/2 data-[handle=left]:cursor-ew-resize",
                                "data-[handle=right]:top-1/2 data-[handle=right]:-right-1 data-[handle=right]:h-6 data-[handle=right]:w-2 data-[handle=right]:-translate-y-1/2 data-[handle=right]:cursor-ew-resize",
                            )
                        />
                    }
                })
                .collect_view()}
        </ImageCropperWindowRoot>
    }
}
