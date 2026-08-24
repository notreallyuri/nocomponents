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
    utils::image::{Crop, revoke_object_url, sniff_mime, to_object_url},
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
    <ExportButton source=Some(AVATAR) onto=picture />
</ImageCropper>

// …and the button itself:
let ctx = use_image_cropper();
let cut = ctx.crop(source, "image/png", 0.92)?;
let url = cut.to_object_url();"#;

const EXPORT: &str = r#"// The same button as the demo above, and the same call inside it. What
// differs is the bytes it is handed: `crop` checks whether it has any, then
// whether they are a GIF, and routes to `crop_gif` — every frame cropped at
// the palette-index level, nothing decoded to pixels — or to the canvas.
<ImageCropper src=src crop=crop cross_origin=false>
    <ExportButton source=Some(SAMPLE) onto=animation label="Crop the GIF" />
</ImageCropper>

// `cut.mime` is read off what came out, not off what was asked for: this one
// reports `image/gif` while the photograph above reports `image/png`.
let cut = ctx.crop(source, "image/png", 0.92)?;

// The source is carried in the binary and served as a blob, which is
// same-origin — so nothing here depends on a third party's CORS policy.
const SAMPLE: &[u8] = include_bytes!("../../../assets/sample.gif");"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "ImageCropper",
        description: "An image with a crop rectangle over it. Drag the rectangle to move it, its handles to resize — hold shift while resizing to keep the shape it had when you grabbed it, alt to keep that shape about the middle instead of the far corner. Shift while moving holds the drag to one of eight directions. The arrows move it too, with shift for larger steps.",
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
                name: "crop(source, mime, quality)",
                ty: "fn -> Result<Cropped, CropError>",
                default: "",
                description: "The crop as a file, at the source's natural resolution. `source` is the bytes the image was decoded from, when the caller still has them — that is what decides whether an animated GIF stays animated, so an app taking either kind of upload writes this one call instead of sniffing the type and branching. `None` asks for a still whatever the source is. `mime` and `quality` describe the still path only.",
            },
            Prop {
                name: "to_data_url(mime, quality)",
                ty: "fn -> Result<String, CropError>",
                default: "",
                description: "The crop as a data URL, for an `<img src>`. The still path only — a canvas sees one frame, so an animated source loses its animation here. `CropError::Tainted` is the cross-origin case.",
            },
            Prop {
                name: "to_pixels()",
                ty: "fn -> (f64, f64, f64, f64)",
                default: "",
                description: "The crop in the source's own pixels, for cutting it somewhere else — on a server, or in a worker.",
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
        description: "Crops every frame of an animated GIF and returns a new one. A canvas cannot do this — `drawImage` sees one frame and `toDataURL(\"image/gif\")` is not implemented anywhere — so this works on the original bytes instead. `crop(source, …)` above calls it for you when the bytes turn out to be a GIF; reach for it directly only when there is no cropper in the picture.",
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

/// One button for both demos, which is the point: the still and the animation differ by what is
/// handed to `source`, not by which function gets called.
#[component]
fn ExportButton(
    /// The bytes the image was decoded from. `Some` is what lets an animated source stay
    /// animated; `None` asks for a still whatever it is.
    #[prop(default = None)]
    source: Option<&'static [u8]>,
    onto: RwSignal<Option<String>>,
    #[prop(default = None)] note: Option<RwSignal<Option<String>>>,
    #[prop(default = "Cut it out")] label: &'static str,
) -> impl IntoView {
    let ctx = use_image_cropper();

    view! {
        <Button
            variant=ButtonVariant::Outline
            size=ButtonSize::Sm
            on:click=move |_| {
                match ctx.crop(source, "image/png", 0.92) {
                    Ok(cut) => {
                        // Both paths end at an object URL, so both are handed back the same way.
                        if let Some(previous) = onto.get_untracked() {
                            revoke_object_url(&previous);
                        }
                        if let Some(note) = note {
                            note.set(Some(format!("{} bytes of {}", cut.bytes.len(), cut.mime)));
                        }
                        onto.set(cut.to_object_url());
                    }
                    Err(error) => {
                        if let Some(note) = note {
                            note.set(Some(format!("{error:?}")));
                        }
                    }
                }
            }
        >
            {label}
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
    let picture = RwSignal::new(None::<String>);
    let animation = RwSignal::new(None::<String>);

    on_cleanup(move || {
        revoke_object_url(&photo.get_value());
        revoke_object_url(&animated.get_value());

        // The cuts are object URLs too, whichever path made them.
        for url in [picture.get_untracked(), animation.get_untracked()]
            .into_iter()
            .flatten()
        {
            revoke_object_url(&url);
        }
    });

    let free = RwSignal::new(Crop::new(0.15, 0.15, 0.6, 0.6));
    let square = RwSignal::new(Crop::new(0.15, 0.15, 0.7, 0.7));
    let cutting = RwSignal::new(Crop::new(0.2, 0.1, 0.5, 0.7));

    let cut_size = RwSignal::new(None::<String>);

    view! {
        <DocLayout
            title="Image Cropper"
            description="A crop rectangle over an image — and a way to cut an animated GIF without flattening it."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The rectangle is in fractions of the image, never pixels, so it means the same thing whatever size the image is drawn at. Drag it to move, take a handle to resize. Hold **shift** while resizing and it keeps the shape it had when you grabbed it — the ratio comes from where the drag started, so it scales instead of creeping towards square, and the corner across from your pointer stays put. Hold **alt** and it keeps that shape about the *middle*: the centre stays where it is and both sides move, so the rectangle grows out of itself rather than away from the far corner, which is the gesture you want for something already framed in the centre. Pulling a corner out by a tenth widens it by two, so it meets the edge of the image in half the distance and gives way there. Moving the whole rectangle there is no shape to keep, so **shift** does the other thing it does everywhere and holds the drag to one of eight directions — the four axes and the four diagonals; push the pointer off the line it picked and nothing happens rather than sliding along it. All of it is read as you drag, so you can take or release a key halfway through. Focus it and the arrows move it, shift for larger steps."
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
                                    <ExportButton source=Some(AVATAR) onto=picture />
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
                    description="This is the reason there is a `utils::gif` at all. A canvas can only ever hand back one frame — `drawImage` copies whatever the image is showing, and `toDataURL(\"image/gif\") quietly returns a PNG. So an animated GIF is cropped through its bytes instead: every frame clipped at the palette-index level, which keeps the palettes, the delays and the loop exactly as they were. The button is the same one as the demo above and the call inside it is the same call — `crop` is handed the bytes, sees a GIF, and takes that route on its own. Crop it, then watch the result: still moving, and it reports `image/gif` rather than the `image/png` that was asked for."
                    code=EXPORT
                >
                    <div class="flex flex-wrap items-start gap-6">
                        <div class="w-72">
                            <ImageCropper
                                src=animated.get_value()
                                alt="A bouncing ball, animated"
                                crop=cutting
                                cross_origin=false
                            >
                                <div class="mt-3">
                                    <ExportButton
                                        source=Some(SAMPLE)
                                        onto=animation
                                        note=Some(cut_size)
                                        label="Crop the GIF"
                                    />
                                </div>
                            </ImageCropper>
                        </div>

                        <div class="flex flex-col gap-3">
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
