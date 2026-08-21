use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
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

const API: &[ApiEntry] = &[ApiEntry {
    name: "Checkbox",
    description: "A box that is ticked or not. Renders a real checkbox role, not a styled input.",
    props: &[
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the box's own classes.",
        },
        Prop {
            name: "checked",
            ty: "RwSignal<bool>",
            default: "",
            description: "The state, read and written. Required: the checkbox does not own it.",
        },
        Prop {
            name: "disabled",
            ty: "Signal<bool>",
            default: "false",
            description: "Stops it taking pointer events and marks it disabled.",
        },
        Prop {
            name: "id",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Given to the control, so a `Label` can point `for` at it. Inside a `Field` this is filled in.",
        },
    ],
}];

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

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
