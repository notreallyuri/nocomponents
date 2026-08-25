use crate::{
    cn,
    primitives::image_cropper::{
        ImageCropperGridRoot, ImageCropperHandleRoot, ImageCropperImageRoot, ImageCropperRoot,
        ImageCropperWindowRoot, use_image_cropper,
    },
    utils::image::{Crop, CropHandle},
};
use leptos::prelude::*;

#[component]
pub fn ImageCropper(
    #[prop(into)] src: Signal<String>,
    #[prop(optional, into)] alt: Signal<String>,
    #[prop(optional, into)] crop: RwSignal<Crop>,
    #[prop(default = None, into)] aspect: Option<f64>,
    #[prop(default = true)] grid: bool,
    #[prop(default = false)] circular: bool,
    #[prop(default = true)] cross_origin: bool,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <ImageCropperRoot
            crop=crop
            aspect=aspect
            circular=circular
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
