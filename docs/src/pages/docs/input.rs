use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    input::{Input, InputType},
    label::Label,
};

const DEFAULT: &str = r#"<Input attr:placeholder="Enter text..." />"#;

const TYPES: &str = r#"<div class="flex flex-col gap-1.5">
    <Label>"Email"</Label>
    <Input input_type=InputType::Email attr:placeholder="you@example.com" />
</div>
<div class="flex flex-col gap-1.5">
    <Label>"Password"</Label>
    <Input
        input_type=InputType::Password
        attr:placeholder="••••••••"
    />
</div>
<div class="flex flex-col gap-1.5">
    <Label>"Number"</Label>
    <Input input_type=InputType::Number attr:placeholder="0" />
</div>"#;

const CONTROLLED: &str = r#"{
    let value = RwSignal::new(String::new());
    view! {
        <div class="flex flex-col gap-3 max-w-sm">
            <Input model=value attr:placeholder="Type something..." />
            <p class="text-sm text-muted-foreground">
                "Value: " {move || value.get()}
            </p>
        </div>
    }
}"#;

const DISABLED: &str = r#"<Input attr:disabled=true value="Cannot edit this" class="max-w-sm" />"#;

const FILE: &str = r#"<Input input_type=InputType::File class="max-w-sm" />"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Input" description="A single-line text field for user input.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="A bare field — extra attributes pass through with attr:."
                    code=DEFAULT
                >
                    <Input attr:placeholder="Enter text..." />
                </DemoSection>

                <DemoSection
                    title="Types"
                    description="input_type maps to the native type, so the browser keeps its own keyboard and validation."
                    code=TYPES
                >
                    <div class="flex flex-col gap-3 max-w-sm">
                        <div class="flex flex-col gap-1.5">
                            <Label>"Email"</Label>
                            <Input input_type=InputType::Email attr:placeholder="you@example.com" />
                        </div>
                        <div class="flex flex-col gap-1.5">
                            <Label>"Password"</Label>
                            <Input
                                input_type=InputType::Password
                                attr:placeholder="••••••••"
                            />
                        </div>
                        <div class="flex flex-col gap-1.5">
                            <Label>"Number"</Label>
                            <Input input_type=InputType::Number attr:placeholder="0" />
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Value bound to a signal via the model prop."
                    code=CONTROLLED
                >
                    {
                        let value = RwSignal::new(String::new());
                        view! {
                            <div class="flex flex-col gap-3 max-w-sm">
                                <Input model=value attr:placeholder="Type something..." />
                                <p class="text-sm text-muted-foreground">
                                    "Value: " {move || value.get()}
                                </p>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled field dims, fills its background and refuses the pointer."
                    code=DISABLED
                >
                    <Input attr:disabled=true value="Cannot edit this" class="max-w-sm" />
                </DemoSection>

                <DemoSection
                    title="File"
                    description="The native file control, restyled through file: variants."
                    code=FILE
                >
                    <Input input_type=InputType::File class="max-w-sm" />
                </DemoSection>
            </div>
        </DocLayout>
    }
}
