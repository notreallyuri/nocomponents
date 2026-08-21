use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{checkbox::Checkbox, label::Label};

const DEFAULT: &str = r#"{
    let checked = RwSignal::new(false);
    view! {
        <div class="flex items-center gap-2">
            <Checkbox id="default" checked=checked />
            <Label>"Accept terms and conditions"</Label>
        </div>
    }
}"#;

const CHECKED_BY_DEFAULT: &str = r#"{
    let checked = RwSignal::new(true);
    view! {
        <div class="flex items-center gap-2">
            <Checkbox id="pre-checked" checked=checked />
            <Label>"Receive marketing emails"</Label>
        </div>
    }
}"#;

const DISABLED: &str = r#"{
    let checked = RwSignal::new(false);
    view! {
        <div class="flex items-center gap-2">
            <Checkbox
                id="disabled-unchecked"
                checked=checked
                disabled=true
            />
            <Label class="opacity-50">"Unavailable option"</Label>
        </div>
    }
}
{
    let checked = RwSignal::new(true);
    view! {
        <div class="flex items-center gap-2">
            <Checkbox id="disabled-checked" checked=checked disabled=true />
            <Label class="opacity-50">"Locked setting"</Label>
        </div>
    }
}"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Checkbox"
            description="A control that allows the user to toggle between checked and unchecked."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="checked takes a signal, so the state stays with the caller."
                    code=DEFAULT
                >
                    {
                        let checked = RwSignal::new(false);
                        view! {
                            <div class="flex items-center gap-2">
                                <Checkbox id="default" checked=checked />
                                <Label>"Accept terms and conditions"</Label>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Checked by default"
                    description="Start the signal at true to render checked on mount."
                    code=CHECKED_BY_DEFAULT
                >
                    {
                        let checked = RwSignal::new(true);
                        view! {
                            <div class="flex items-center gap-2">
                                <Checkbox id="pre-checked" checked=checked />
                                <Label>"Receive marketing emails"</Label>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled checkbox keeps its state but ignores pointer and keyboard."
                    code=DISABLED
                >
                    <div class="flex flex-col gap-3">
                        {
                            let checked = RwSignal::new(false);
                            view! {
                                <div class="flex items-center gap-2">
                                    <Checkbox
                                        id="disabled-unchecked"
                                        checked=checked
                                        disabled=true
                                    />
                                    <Label class="opacity-50">"Unavailable option"</Label>
                                </div>
                            }
                        }
                        {
                            let checked = RwSignal::new(true);
                            view! {
                                <div class="flex items-center gap-2">
                                    <Checkbox id="disabled-checked" checked=checked disabled=true />
                                    <Label class="opacity-50">"Locked setting"</Label>
                                </div>
                            }
                        }
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
