use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::avatar::{
    Avatar, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
};

const SIZES: &str = r#"<Avatar size=AvatarSize::Sm>
    <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
    <AvatarFallback>"U"</AvatarFallback>
</Avatar>
<Avatar>
    <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
    <AvatarFallback>"U"</AvatarFallback>
</Avatar>
<Avatar size=AvatarSize::Lg>
    <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
    <AvatarFallback>"U"</AvatarFallback>
</Avatar>"#;

const FALLBACK: &str = r#"<Avatar>
    <AvatarImage src="broken-url" alt="User" />
    <AvatarFallback>"YR"</AvatarFallback>
</Avatar>
<Avatar>
    <AvatarImage src="broken-url" alt="User" />
    <AvatarFallback delay_ms=150>"AB"</AvatarFallback>
</Avatar>"#;

const GROUP: &str = r#"<AvatarGroup>
    <Avatar>
        <AvatarImage src=AVATAR_URL alt="User 1" class="object-cover" />
        <AvatarFallback>"A"</AvatarFallback>
    </Avatar>
    <Avatar>
        <AvatarImage src="broken-url" alt="User 2" />
        <AvatarFallback>"B"</AvatarFallback>
    </Avatar>
    <Avatar>
        <AvatarImage src="broken-url" alt="User 3" />
        <AvatarFallback>"C"</AvatarFallback>
    </Avatar>
    <AvatarGroupCount>"+4"</AvatarGroupCount>
</AvatarGroup>"#;

const AVATAR_URL: &str = "https://w.wallhaven.cc/full/5d/wallhaven-5d23j9.png";

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Avatar",
        description: "A person, as a picture with something to fall back on.",
        props: &[
            Prop {
                name: "size",
                ty: "AvatarSize",
                default: "Default",
                description: "One of: Default, Sm, Lg.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the size classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "An image, a fallback, and optionally a badge.",
            },
        ],
    },
    ApiEntry {
        name: "AvatarImage",
        description: "The picture. Hidden until it loads, so the fallback is never replaced by a broken image.",
        props: &[
            Prop {
                name: "src",
                ty: "String",
                default: "",
                description: "The image URL.",
            },
            Prop {
                name: "alt",
                ty: "String",
                default: "\"\"",
                description: "Alternative text.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the image classes.",
            },
        ],
    },
    ApiEntry {
        name: "AvatarFallback",
        description: "What stands in until the image loads, or for good if it never does.",
        props: &[
            Prop {
                name: "delay_ms",
                ty: "Option<u64>",
                default: "None",
                description: "Wait this long before showing, so a fast image does not flash initials first.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the fallback classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "Usually initials.",
            },
        ],
    },
    ApiEntry {
        name: "AvatarBadge",
        description: "A dot in the corner, for presence or a count.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the positioning classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually nothing, or a number.",
            },
        ],
    },
    ApiEntry {
        name: "AvatarGroup",
        description: "Overlapping avatars, in the order they are written.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the overlap classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Avatars, and optionally a count at the end.",
            },
        ],
    },
    ApiEntry {
        name: "AvatarGroupCount",
        description: "The \"+3\" that stands for the avatars not shown.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the avatar classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a number.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Avatar"
            description="An image element with a fallback for missing or loading states."
        >
            <div class="flex flex-col gap-8">
                <DemoSection title="Sizes" description="Sm, the default, and Lg." code=SIZES>
                    <div class="flex items-end gap-4">
                        <Avatar size=AvatarSize::Sm>
                            <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
                            <AvatarFallback>"U"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
                            <AvatarFallback>"U"</AvatarFallback>
                        </Avatar>
                        <Avatar size=AvatarSize::Lg>
                            <AvatarImage src=AVATAR_URL alt="User" class="object-cover" />
                            <AvatarFallback>"U"</AvatarFallback>
                        </Avatar>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Fallback"
                    description="Shown while the image is loading or on error."
                    code=FALLBACK
                >
                    <div class="flex items-center gap-4">
                        <Avatar>
                            <AvatarImage src="broken-url" alt="User" />
                            <AvatarFallback>"YR"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src="broken-url" alt="User" />
                            <AvatarFallback delay_ms=150>"AB"</AvatarFallback>
                        </Avatar>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Group"
                    description="Members overlap in order; AvatarGroupCount closes the row with an overflow count."
                    code=GROUP
                >
                    <AvatarGroup>
                        <Avatar>
                            <AvatarImage src=AVATAR_URL alt="User 1" class="object-cover" />
                            <AvatarFallback>"A"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src="broken-url" alt="User 2" />
                            <AvatarFallback>"B"</AvatarFallback>
                        </Avatar>
                        <Avatar>
                            <AvatarImage src="broken-url" alt="User 3" />
                            <AvatarFallback>"C"</AvatarFallback>
                        </Avatar>
                        <AvatarGroupCount>"+4"</AvatarGroupCount>
                    </AvatarGroup>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
