use super::Block;
use leptos::{html, prelude::*};
use nocomponents::{
    components::{
        avatar::{Avatar, AvatarFallback, AvatarImage},
        button::{Button, ButtonSize, ButtonVariant},
        dialog::{
            Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPortal, DialogTitle,
            DialogTrigger,
        },
        image_cropper::ImageCropper,
    },
    primitives::image_cropper::use_image_cropper,
    utils::image::{Crop, read_bytes, revoke_object_url, sniff_mime, to_object_url},
};

const AVATAR: &[u8] = include_bytes!("../../../../assets/avatar.jpg");

const AVATAR_CROP: &str = r#"let source = RwSignal::new(sample);
let picked = RwSignal::new(None::<String>);
let file_input = NodeRef::<html::Input>::new();

// The only place an animated GIF exists: an `<img>` cannot hand its bytes back
// and a canvas sees one frame, so whoever wants to crop one has to keep them.
let bytes = StoredValue::new(AVATAR.to_vec());

let take_file = move |_| {
    let Some(file) = file_input.get().and_then(|i| i.files()).and_then(|f| f.get(0)) else {
        return;
    };

    // Same file twice in a row fires no `change` at all unless the value is cleared.
    if let Some(input) = file_input.get() {
        input.set_value("");
    }

    // Reading a file is asynchronous, so the rest happens when the bytes arrive.
    // One read serves both jobs — the URL the `<img>` loads is made from the same
    // bytes the crop is cut from, so the two can never disagree.
    read_bytes(&file, move |file_bytes| {
        let mime = sniff_mime(&file_bytes).unwrap_or("image/png");
        let Some(url) = to_object_url(&file_bytes, mime) else { return };

        // The previous one is handed back the moment it stops being the source;
        // the last one goes in `on_cleanup`. A blob URL nobody revokes lives as
        // long as the document does.
        if let Some(previous) = picked.get_untracked() {
            revoke_object_url(&previous);
        }
        picked.set(Some(url.clone()));
        bytes.set_value(file_bytes);
        source.set(url);
        crop.set(Crop::new(0.1, 0.1, 0.8, 0.8));
        open.set(true);
    });
};

view! {
    <input type="file" accept="image/*" class="sr-only" node_ref=file_input on:change=take_file />
    <Button on:click=move |_| { file_input.get().map(|input| input.click()); }>
        "Pick an image…"
    </Button>

    <Dialog open=open>
        <DialogTrigger variant=ButtonVariant::Ghost>"Crop this one"</DialogTrigger>
        <DialogPortal class="sm:max-w-md">
            <DialogHeader>
                <DialogTitle>"Crop your photo"</DialogTitle>
            </DialogHeader>
            <ImageCropper src=source crop=crop aspect=Some(1.0) circular=true cross_origin=false>
                // Inside the cropper, not in the footer where it looks like it belongs:
                // `use_image_cropper()` only resolves under the root that provides it.
                <DialogFooter class="mt-4">
                    <Button variant=ButtonVariant::Outline on:click=move |_| open.set(false)>
                        "Cancel"
                    </Button>
                    <ApplyCrop source=bytes onto=picture close=open />
                </DialogFooter>
            </ImageCropper>
        </DialogPortal>
    </Dialog>
}

// …and the button itself. `Some(bytes)` is what decides a GIF keeps moving —
// `crop` checks whether it has any, then whether they are one, and routes to
// `crop_gif` or to the canvas. Passing `None` asks for a still whatever it is.
let ctx = use_image_cropper();
let cut = source.with_value(|bytes| ctx.crop(Some(bytes), "image/png", 0.92))?;
let url = cut.to_object_url();"#;

pub const BLOCKS: &[Block] = &[Block {
    title: "Choosing an avatar",
    description: "Pick an image of your own — an animated GIF included — and it takes over the \
                  cropper. Nothing is uploaded or stored: the file is read once, in the browser, \
                  and what comes out is handed back on `on_cleanup`, so the picture lasts exactly \
                  as long as this page is open. Those bytes are the point. An `<img>` cannot give \
                  them back and a canvas sees one frame, so keeping them is the whole difference \
                  between an avatar that moves and one that quietly stops — and it is `crop` that \
                  notices, not the call site. The composition is three deep: a dialog holding the \
                  cropper, an avatar holding the result. The seam is where the confirm button \
                  goes — it reads as a footer control, but it needs `use_image_cropper()`, which \
                  only resolves inside the cropper's own root, so the footer is rendered *as the \
                  cropper's children*. The drag works inside the dialog's focus trap because the \
                  gesture is pointer events on the surface, not a tab stop.",
    code: AVATAR_CROP,
    view: || view! { <AvatarCropper /> }.into_any(),
}];

#[component]
fn ApplyCrop(
    /// The bytes the image on screen came from. Handing them over is what keeps an animated
    /// avatar animated; without them this would quietly cut one frame out of it.
    source: StoredValue<Vec<u8>>,
    onto: RwSignal<Option<String>>,
    close: RwSignal<bool>,
) -> impl IntoView {
    let ctx = use_image_cropper();

    view! {
        <Button on:click=move |_| {
            let cut = source.with_value(|bytes| ctx.crop(Some(bytes), "image/png", 0.92));
            if let Ok(cut) = cut {
                if let Some(previous) = onto.get_untracked() {
                    revoke_object_url(&previous);
                }
                onto.set(cut.to_object_url());
            }
            close.set(false);
        }>"Use photo"</Button>
    }
}

#[component]
fn AvatarCropper() -> impl IntoView {
    let sample = StoredValue::new(
        to_object_url(AVATAR, sniff_mime(AVATAR).unwrap_or("image/jpeg")).unwrap_or_default(),
    );
    let source = RwSignal::new(sample.get_value());
    let picked = RwSignal::new(None::<String>);

    // The bytes behind whatever is on screen. An `<img>` cannot hand them back and a canvas
    // cannot see past the frame it is showing, so this is the only place an animated GIF exists
    // — and holding it is the whole difference between an avatar that moves and one that does
    // not. It starts as the sample and is replaced by whatever gets picked.
    let bytes = StoredValue::new(AVATAR.to_vec());

    let open = RwSignal::new(false);
    let crop = RwSignal::new(Crop::new(0.25, 0.05, 0.5, 0.9));
    let picture = RwSignal::new(None::<String>);
    let file_input = NodeRef::<html::Input>::new();

    on_cleanup(move || {
        revoke_object_url(&sample.get_value());
        for url in [picked.get_untracked(), picture.get_untracked()]
            .into_iter()
            .flatten()
        {
            revoke_object_url(&url);
        }
    });

    let take_file = move |_| {
        let Some(file) = file_input
            .get()
            .and_then(|input| input.files())
            .and_then(|files| files.get(0))
        else {
            return;
        };

        // Picking the same file twice fires no `change` unless the value is cleared first.
        if let Some(input) = file_input.get() {
            input.set_value("");
        }

        // Reading a file is asynchronous, so the rest of this happens when the bytes arrive
        // rather than now. One read serves both jobs: the URL the `<img>` loads is made from
        // the same bytes the crop will be cut from, so the two can never disagree.
        read_bytes(&file, move |file_bytes| {
            let mime = sniff_mime(&file_bytes).unwrap_or("image/png");
            let Some(url) = to_object_url(&file_bytes, mime) else {
                return;
            };

            // The previous one is handed back the moment it stops being the source; the last
            // one goes in `on_cleanup`. A blob URL nobody revokes lives as long as the document.
            if let Some(previous) = picked.get_untracked() {
                revoke_object_url(&previous);
            }
            picked.set(Some(url.clone()));
            bytes.set_value(file_bytes);
            source.set(url);
            // A new image is a new crop; `aspect` squares it up again when it loads.
            crop.set(Crop::new(0.1, 0.1, 0.8, 0.8));
            open.set(true);
        });
    };

    view! {
        <div class="flex items-center gap-4">
            <Avatar class="size-16">
                {move || match picture.get() {
                    Some(url) => view! { <AvatarImage src=url alt="Your avatar" /> }.into_any(),
                    None => view! { <AvatarFallback>"YO"</AvatarFallback> }.into_any(),
                }}
            </Avatar>

            <div class="flex flex-col items-start gap-1">
                <div class="flex items-center gap-1">
                    <input
                        type="file"
                        accept="image/*"
                        class="sr-only"
                        node_ref=file_input
                        on:change=take_file
                    />
                    <Button
                        variant=ButtonVariant::Outline
                        size=ButtonSize::Sm
                        on:click=move |_| {
                            if let Some(input) = file_input.get() {
                                input.click();
                            }
                        }
                    >
                        "Pick an image…"
                    </Button>

                    <Dialog open=open>
                        <DialogTrigger variant=ButtonVariant::Ghost size=ButtonSize::Sm>
                            "Crop this one"
                        </DialogTrigger>
                        <DialogPortal class="sm:max-w-md">
                            <DialogHeader>
                                <DialogTitle>"Crop your photo"</DialogTitle>
                                <DialogDescription>
                                    "Drag the square to frame it. The circle is the preview; the
                                    square underneath is what gets cut."
                                </DialogDescription>
                            </DialogHeader>
                            <ImageCropper
                                src=source
                                alt="The photograph being cropped"
                                crop=crop
                                aspect=Some(1.0)
                                circular=true
                                cross_origin=false
                            >
                                <DialogFooter class="mt-4">
                                    <Button
                                        variant=ButtonVariant::Outline
                                        on:click=move |_| open.set(false)
                                    >
                                        "Cancel"
                                    </Button>
                                    <ApplyCrop source=bytes onto=picture close=open />
                                </DialogFooter>
                            </ImageCropper>
                        </DialogPortal>
                    </Dialog>
                </div>

                <p class="text-xs text-muted-foreground">
                    "Cut at the source's own resolution, not the size it was shown at — and it
                    never leaves the page."
                </p>
            </div>
        </div>
    }
}
