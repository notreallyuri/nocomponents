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
            description: "Two-way binding. Pass this or `value`, not both. A reset of the surrounding `<form>` (or the one named by `form`) writes `default_value` into it; see `default_value`.",
        },
        Prop {
            name: "default_value",
            ty: "Signal<String>",
            default: "\"\"",
            description: "What a reset of the form puts back, read at the moment of the reset — so a value that arrived after mount is honoured. Empty means unset, and the current `value` is used instead. Never the `model`'s own earlier contents. The result is written to the control and to `model`.",
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
            name: "name",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The name it is submitted under in a form's data. Empty leaves it out.",
        },
        Prop {
            name: "required",
            ty: "Signal<bool>",
            default: "false",
            description: "The browser refuses to submit the form while it is empty.",
        },
        Prop {
            name: "readonly",
            ty: "Signal<bool>",
            default: "false",
            description: "Focusable and selectable but not editable; still submitted.",
        },
        Prop {
            name: "placeholder",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Hint text shown while empty.",
        },
        Prop {
            name: "autocomplete",
            ty: "Signal<String>",
            default: "\"\"",
            description: "A browser autofill hint. Empty leaves the attribute off.",
        },
        Prop {
            name: "rows",
            ty: "Option<u32>",
            default: "None",
            description: "The visible height in lines.",
        },
        Prop {
            name: "form",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The `id` of a `<form>` it belongs to when it is not inside one. Empty means the one around it.",
        },
        Prop {
            name: "auto_resize",
            ty: "bool",
            default: "false",
            description: "Grow with the text instead of scrolling, re-measured on every input.",
        },
        Prop {
            name: "on_input",
            ty: "Option<Callback<String>>",
            default: "None",
            description: "Every edit, with the text as it stands.",
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
