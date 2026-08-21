use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::aspect_ratio::AspectRatio;

const DEMO_16_9: &str = r#"<AspectRatio ratio=16.0 / 9.0 class="rounded-xl">
    <img src=IMAGE_URL alt="Landscape" />
</AspectRatio>"#;

const SQUARE: &str = r#"<AspectRatio ratio=1.0 class="rounded-xl">
    <img src=IMAGE_URL alt="Square crop" />
</AspectRatio>"#;

const PORTRAIT: &str = r#"<AspectRatio ratio=3.0 / 4.0 class="rounded-xl">
    <img src=IMAGE_URL alt="Portrait crop" />
</AspectRatio>"#;

const IMAGE_URL: &str = "https://w.wallhaven.cc/full/5d/wallhaven-5d23j9.png";

const API: &[ApiEntry] = &[ApiEntry {
    name: "AspectRatio",
    description: "Holds its child to a ratio, whatever the width is.",
    props: &[
        Prop {
            name: "ratio",
            ty: "f64",
            default: "1.0",
            description: "Width divided by height — `16.0 / 9.0` for widescreen, `1.0` for a square.",
        },
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the box's classes.",
        },
        Prop {
            name: "children",
            ty: "Children",
            default: "",
            description: "Usually an image or an iframe, which is stretched to fill.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Aspect Ratio"
            description="Constrains content to a given width-to-height ratio."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="16 / 9"
                    description="The box fills the width it is given and derives its height from the ratio."
                    code=DEMO_16_9
                >
                    <div class="max-w-md">
                        <AspectRatio ratio=16.0 / 9.0 class="rounded-xl">
                            <img src=IMAGE_URL alt="Landscape" />
                        </AspectRatio>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Square"
                    description="Pass ratio=1.0 for a square crop."
                    code=SQUARE
                >
                    <div class="w-48">
                        <AspectRatio ratio=1.0 class="rounded-xl">
                            <img src=IMAGE_URL alt="Square crop" />
                        </AspectRatio>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Portrait"
                    description="Any ratio works — the child is cropped to fill the box."
                    code=PORTRAIT
                >
                    <div class="w-40">
                        <AspectRatio ratio=3.0 / 4.0 class="rounded-xl">
                            <img src=IMAGE_URL alt="Portrait crop" />
                        </AspectRatio>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
