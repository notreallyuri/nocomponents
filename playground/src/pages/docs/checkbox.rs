use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{checkbox::Checkbox, label::Label};

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Checkbox"
            description="A control that allows the user to toggle between checked and unchecked."
        >
            <div class="flex flex-col gap-8">
                <DemoSection title="Default">
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

                <DemoSection title="Checked by default">
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

                <DemoSection title="Disabled">
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
