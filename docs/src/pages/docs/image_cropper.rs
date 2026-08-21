use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        image_cropper::ImageCropper,
    },
    primitives::image_cropper::use_image_cropper,
    utils::{
        gif::crop_gif,
        image::{Crop, revoke_object_url, sniff_mime, to_object_url},
    },
};

const AVATAR: &[u8] = include_bytes!("../../../assets/avatar.jpg");
const SAMPLE: &[u8] = include_bytes!("../../../assets/sample.gif");

const DEFAULT: &str = r#"{
    let crop = RwSignal::new(Crop::full());

    view! {
        <ImageCropper src=src crop=crop />
        <p>{move || format!("{:?}", crop.get())}</p>
    }
}"#;

const ASPECT: &str = r#"// A pixel ratio, not a normalised one — the component knows the image's
// natural size and converts. With a ratio held, only the corners are offered,
// and `circular` rounds the preview off without changing what is cut.
<ImageCropper src=src crop=crop aspect=Some(1.0) circular=true>
    // Inside the cropper, so it can reach `use_image_cropper()`.
    <ExportButton onto=picture />
</ImageCropper>

// …and the button itself:
let ctx = use_image_cropper();
let url = ctx.to_data_url("image/png", 0.92)?;"#;

const EXPORT: &str = r#"// A canvas sees one frame of a GIF, so this goes through the bytes:
// every frame cropped at the palette-index level, nothing decoded to pixels.
let animation = crop_gif(SAMPLE, crop.get())?;
let url = to_object_url(&animation, "image/gif");

// The source is carried in the binary and served as a blob, which is
// same-origin — so nothing here depends on a third party's CORS policy.
const SAMPLE: &[u8] = include_bytes!("../../../assets/sample.gif");"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "ImageCropper",
        description: "An image with a crop rectangle over it. Drag the rectangle to move it, its handles to resize; the arrows move it too, with shift for larger steps.",
        props: &[
            Prop {
                name: "src",
                ty: "Signal<String>",
                default: "",
                description: "The image. An object URL from the file the reader picked is the usual thing.",
            },
            Prop {
                name: "alt",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The image's alternative text.",
            },
            Prop {
                name: "crop",
                ty: "RwSignal<Crop>",
                default: "the whole image",
                description: "The rectangle, in fractions of the image — never pixels, so it survives the `<img>` being any size on screen. Read it, or write one in.",
            },
            Prop {
                name: "aspect",
                ty: "Option<f64>",
                default: "None",
                description: "A ratio to hold, width over height *in pixels*. The component converts it against the image's natural size, so 1.0 is square on any photograph. Applied when the image loads — until then nobody knows how many pixels there are per fraction — and the crop you supplied is conformed to it rather than replaced. With one set, the edge handles are dropped: an edge cannot honour a ratio without moving a side the pointer is not on.",
            },
            Prop {
                name: "grid",
                ty: "bool",
                default: "true",
                description: "The thirds drawn over the crop.",
            },
            Prop {
                name: "circular",
                ty: "bool",
                default: "false",
                description: "Shows the crop as a circle, for something that will be displayed in one later. A preview only — the rectangle underneath is unchanged and is still what gets cut, so pair it with `aspect=Some(1.0)` unless an ellipse is what you meant. Exposed as `data-circular` on the root for anyone styling the unstyled layer.",
            },
            Prop {
                name: "cross_origin",
                ty: "bool",
                default: "true",
                description: "Asks for the image with CORS. Needed for anything cross-origin that will be exported: without it — and without the host allowing it — the canvas is tainted and reading it back throws.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the cropper's classes.",
            },
        ],
    },
    ApiEntry {
        name: "use_image_cropper()",
        description: "The context, for anything rendered inside the cropper. Cutting is a method rather than a prop, because when to cut is the caller's decision.",
        props: &[
            Prop {
                name: "crop",
                ty: "RwSignal<Crop>",
                default: "",
                description: "The same rectangle the component was given.",
            },
            Prop {
                name: "natural",
                ty: "RwSignal<(f64, f64)>",
                default: "(0, 0)",
                description: "The image's natural size, read off its load event. What a pixel aspect ratio is judged against.",
            },
            Prop {
                name: "to_data_url(mime, quality)",
                ty: "fn -> Result<String, CropError>",
                default: "",
                description: "The crop as a still, at the source's natural resolution. `CropError::Tainted` is the cross-origin case.",
            },
            Prop {
                name: "to_pixels()",
                ty: "fn -> (f64, f64, f64, f64)",
                default: "",
                description: "The crop in the source's own pixels, for cutting it somewhere else — on a server, or through `crop_gif`.",
            },
            Prop {
                name: "reset()",
                ty: "fn",
                default: "",
                description: "Back to the largest rectangle of the held ratio, centred.",
            },
        ],
    },
    ApiEntry {
        name: "utils::gif::crop_gif",
        description: "Crops every frame of an animated GIF and returns a new one. A canvas cannot do this — `drawImage` sees one frame and `toDataURL(\"image/gif\")` is not implemented anywhere — so this works on the original bytes instead.",
        props: &[
            Prop {
                name: "bytes",
                ty: "&[u8]",
                default: "",
                description: "The original file. Keep it around: the `<img>` cannot give it back.",
            },
            Prop {
                name: "crop",
                ty: "Crop",
                default: "",
                description: "The same rectangle, in the same fractions.",
            },
            Prop {
                name: "returns",
                ty: "Result<Vec<u8>, GifError>",
                default: "",
                description: "A new GIF. Every frame is cropped at the palette-index level rather than decoded to pixels, so there is no quantisation: the palettes, delays, disposal methods, transparency and loop count are the originals.",
            },
        ],
    },
];

#[component]
fn ExportButton(onto: RwSignal<Option<String>>) -> impl IntoView {
    let ctx = use_image_cropper();

    view! {
        <Button
            variant=ButtonVariant::Outline
            size=ButtonSize::Sm
            on:click=move |_| {
                if let Ok(url) = ctx.to_data_url("image/png", 0.92) {
                    onto.set(Some(url));
                }
            }
        >
            "Cut it out"
        </Button>
    }
}

#[component]
pub fn Page() -> impl IntoView {
    let photo = StoredValue::new(
        to_object_url(AVATAR, sniff_mime(AVATAR).unwrap_or("image/jpeg")).unwrap_or_default(),
    );
    let animated = StoredValue::new(
        to_object_url(SAMPLE, sniff_mime(SAMPLE).unwrap_or("image/gif")).unwrap_or_default(),
    );
    on_cleanup(move || {
        revoke_object_url(&photo.get_value());
        revoke_object_url(&animated.get_value());
    });

    let free = RwSignal::new(Crop::new(0.15, 0.15, 0.6, 0.6));
    let square = RwSignal::new(Crop::new(0.15, 0.15, 0.7, 0.7));
    let cutting = RwSignal::new(Crop::new(0.2, 0.1, 0.5, 0.7));

    let picture = RwSignal::new(None::<String>);
    let animation = RwSignal::new(None::<String>);
    let cut_size = RwSignal::new(None::<String>);

    let cut = move |_| {
        let crop = cutting.get_untracked();

        match crop_gif(SAMPLE, crop) {
            Ok(bytes) => {
                if let Some(previous) = animation.get_untracked() {
                    revoke_object_url(&previous);
                }
                cut_size.set(Some(format!("{} bytes", bytes.len())));
                animation.set(to_object_url(&bytes, "image/gif"));
            }
            Err(error) => {
                cut_size.set(Some(format!("{error:?}")));
                animation.set(None);
            }
        }
    };

    view! {
        <DocLayout
            title="Image Cropper"
            description="A crop rectangle over an image — and a way to cut an animated GIF without flattening it."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The rectangle is in fractions of the image, never pixels, so it means the same thing whatever size the image is drawn at. Drag it to move, take a handle to resize; focus it and the arrows move it, shift for larger steps."
                    code=DEFAULT
                >
                    <div class="flex flex-wrap items-start gap-6">
                        <div class="w-72">
                            <ImageCropper
                                src=photo.get_value()
                                alt="A photograph to crop"
                                crop=free
                                cross_origin=false
                            />
                        </div>
                        <p class="font-mono text-xs whitespace-pre text-muted-foreground">
                            {move || {
                                let crop = free.get();
                                format!(
                                    "x {:.2}  y {:.2}\nw {:.2}  h {:.2}",
                                    crop.x,
                                    crop.y,
                                    crop.width,
                                    crop.height,
                                )
                            }}
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="A held ratio, and the cut"
                    description="`aspect` is a pixel ratio — 1.0 is square on the finished image, whatever shape the source is — and the component converts it against the image's natural size, so this stays square on a 16:9 photograph. It is applied when the image loads, not when the first handle moves. `circular` rounds the preview off for an avatar; the rectangle underneath is unchanged and is still what gets cut. Only the corners are offered while a ratio is held, and the button inside the cropper is how it reaches `use_image_cropper()`."
                    code=ASPECT
                >
                    <div class="flex flex-wrap items-start gap-6">
                        <div class="w-72">
                            <ImageCropper
                                src=photo.get_value()
                                alt="A photograph to crop square"
                                crop=square
                                aspect=Some(1.0)
                                circular=true
                                cross_origin=false
                            >
                                <div class="mt-3">
                                    <ExportButton onto=picture />
                                </div>
                            </ImageCropper>
                        </div>

                        <div class="flex flex-col gap-2">
                            {move || {
                                picture
                                    .get()
                                    .map(|url| {
                                        view! {
                                            <img
                                                src=url
                                                alt="The cropped photograph"
                                                class="size-32 rounded-full border object-cover"
                                            />
                                        }
                                    })
                            }}
                            <p class="max-w-56 text-xs text-muted-foreground">
                                "Cut through a canvas at the source's own resolution. The image is
                                carried in the binary and served as a blob, so it is same-origin and
                                the canvas is never tainted."
                            </p>
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Cutting a GIF"
                    description="This is the reason there is a `utils::gif` at all. A canvas can only ever hand back one frame — `drawImage` copies whatever the image is showing, and `toDataURL(\"image/gif\")` quietly returns a PNG. So an animated GIF is cropped through its bytes instead: every frame clipped at the palette-index level, which keeps the palettes, the delays and the loop exactly as they were. Crop it, then watch the result — it is still moving."
                    code=EXPORT
                >
                    <div class="flex flex-wrap items-start gap-6">
                        <div class="w-72">
                            <ImageCropper
                                src=animated.get_value()
                                alt="A bouncing ball, animated"
                                crop=cutting
                                cross_origin=false
                            />
                        </div>

                        <div class="flex flex-col gap-3">
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm on:click=cut>
                                "Crop the GIF"
                            </Button>

                            <p class="font-mono text-xs text-muted-foreground">
                                {move || {
                                    cut_size.get().unwrap_or_else(|| "Not cut yet.".to_string())
                                }}
                            </p>

                            {move || {
                                animation
                                    .get()
                                    .map(|url| {
                                        view! {
                                            <div class="flex flex-col gap-1">
                                                <span class="text-xs text-muted-foreground">
                                                    "Still animating:"
                                                </span>
                                                <img
                                                    src=url
                                                    alt="The cropped animation"
                                                    class="rounded-md border bg-muted"
                                                />
                                            </div>
                                        }
                                    })
                            }}
                        </div>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
