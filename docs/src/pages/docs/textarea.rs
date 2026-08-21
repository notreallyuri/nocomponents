use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::textarea::Textarea;

const DEFAULT: &str = r#"<Textarea />"#;

const AUTO_RESIZE_MAX_HEIGHT_7_5REM: &str = r#"<Textarea auto_resize=true class="max-h-30" />"#;

const AUTO_RESIZE_FREE_HEIGHT: &str = r#"<Textarea auto_resize=true />"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "Textarea",
    description: "A multi-line text field.",
    props: &[
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the field's own classes.",
        },
        Prop {
            name: "model",
            ty: "Option<RwSignal<String>>",
            default: "None",
            description: "Two-way binding. Pass this or `value`, not both.",
        },
        Prop {
            name: "value",
            ty: "Signal<String>",
            default: "\"\"",
            description: "One-way: the field shows it and never writes back.",
        },
        Prop {
            name: "id",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Overridden by an enclosing `Field`, which mints its own.",
        },
        Prop {
            name: "disabled",
            ty: "Signal<bool>",
            default: "false",
            description: "Stops it taking pointer events and marks it disabled.",
        },
        Prop {
            name: "auto_resize",
            ty: "bool",
            default: "false",
            description: "Grow with the text instead of scrolling, re-measured on every input.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Text Area" description="A multi-line text field for user input.">
            <div class="flex flex-col gap-9">
                <DemoSection
                    title="Default"
                    description="A fixed minimum height the user can drag to resize."
                    code=DEFAULT
                >
                    <Textarea />
                </DemoSection>

                <DemoSection
                    title="Auto Resize (max height: 7.5rem)"
                    description="auto_resize grows the field with its content; a max height caps it and hands back the scrollbar."
                    code=AUTO_RESIZE_MAX_HEIGHT_7_5REM
                >
                    <div class="flex flex-col gap-2">
                        <Textarea auto_resize=true class="max-h-30" />
                    </div>
                </DemoSection>

                <DemoSection
                    title="Auto Resize (Free height)"
                    description="Without a cap the field keeps growing as the text does."
                    code=AUTO_RESIZE_FREE_HEIGHT
                >
                    <div class="flex flex-col gap-2">
                        <Textarea auto_resize=true />
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
