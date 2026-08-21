use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::textarea::Textarea;

const DEFAULT: &str = r#"<Textarea />"#;

const AUTO_RESIZE_MAX_HEIGHT_7_5REM: &str = r#"<Textarea auto_resize=true class="max-h-30" />"#;

const AUTO_RESIZE_FREE_HEIGHT: &str = r#"<Textarea auto_resize=true />"#;

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
            </div>
        </DocLayout>
    }
}
