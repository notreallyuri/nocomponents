use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{checkbox::Checkbox, input::Input, label::Label};

const WITH_AN_INPUT: &str = r#"<Label attr:r#for="email">"Email"</Label>
<Input attr:id="email" attr:placeholder="you@example.com" />"#;

const WITH_A_CHECKBOX: &str = r#"{
    let checked = RwSignal::new(false);
    view! {
        <Label>
            <Checkbox checked=checked />
            "Accept terms and conditions"
        </Label>
    }
}"#;

const DISABLED_CONTROL: &str = r#"<Input attr:id="handle" attr:placeholder="@handle" attr:disabled=true />
<Label class="peer-disabled:opacity-50">"Username (unavailable)"</Label>"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Label" description="An accessible caption for a form control.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="With an input"
                    description="Point for at the control's id to tie the two together."
                    code=WITH_AN_INPUT
                >
                    <div class="flex max-w-sm flex-col gap-2">
                        <Label attr:r#for="email">"Email"</Label>
                        <Input attr:id="email" attr:placeholder="you@example.com" />
                    </div>
                </DemoSection>

                <DemoSection
                    title="With a checkbox"
                    description="Wrapping the control makes the whole label clickable."
                    code=WITH_A_CHECKBOX
                >
                    {
                        let checked = RwSignal::new(false);
                        view! {
                            <Label>
                                <Checkbox checked=checked />
                                "Accept terms and conditions"
                            </Label>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Disabled control"
                    description="A label dims itself next to a disabled peer."
                    code=DISABLED_CONTROL
                >
                    <div class="flex max-w-sm flex-col gap-2">
                        <Input attr:id="handle" attr:placeholder="@handle" attr:disabled=true />
                        <Label class="peer-disabled:opacity-50">"Username (unavailable)"</Label>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
