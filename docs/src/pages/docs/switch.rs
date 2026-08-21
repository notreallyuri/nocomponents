use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{label::Label, switch::Switch};

const DEFAULT: &str = r#"{
    let checked = RwSignal::new(false);
    view! { <Switch checked=checked /> }
}"#;

const WITH_LABEL: &str = r#"{
    let airplane = RwSignal::new(false);
    view! {
        <div class="flex items-center justify-between max-w-xs">
            <div class="flex flex-col gap-0.5">
                <Label>"Airplane mode"</Label>
                <span class="text-xs text-muted-foreground">
                    "Disable all wireless connections."
                </span>
            </div>
            <Switch checked=airplane />
        </div>
    }
}
{
    let notifs = RwSignal::new(true);
    view! {
        <div class="flex items-center justify-between max-w-xs">
            <div class="flex flex-col gap-0.5">
                <Label>"Push notifications"</Label>
                <span class="text-xs text-muted-foreground">
                    "Receive alerts on your device."
                </span>
            </div>
            <Switch checked=notifs />
        </div>
    }
}"#;

const DISABLED: &str = r#"{
    let off = RwSignal::new(false);
    let on = RwSignal::new(true);

    view! { 
        <Switch checked=off disabled=true /> 
        <Switch checked=on disabled=true />
    }
}"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "Switch",
    description: "An on/off control that applies immediately, unlike a checkbox in a form.",
    props: &[
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the track's own classes.",
        },
        Prop {
            name: "checked",
            ty: "RwSignal<bool>",
            default: "",
            description: "The state, read and written. Required: the switch does not own it.",
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
        <DocLayout title="Switch" description="A toggle control for binary on/off settings.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="checked takes a signal; the thumb slides to match it."
                    code=DEFAULT
                >
                    {
                        let checked = RwSignal::new(false);
                        view! { <Switch checked=checked /> }
                    }
                </DemoSection>

                <DemoSection
                    title="With label"
                    description="Pair with a Label and a line of help text for a settings row."
                    code=WITH_LABEL
                >
                    <div class="flex flex-col gap-4">
                        {
                            let airplane = RwSignal::new(false);
                            view! {
                                <div class="flex items-center justify-between max-w-xs">
                                    <div class="flex flex-col gap-0.5">
                                        <Label>"Airplane mode"</Label>
                                        <span class="text-xs text-muted-foreground">
                                            "Disable all wireless connections."
                                        </span>
                                    </div>
                                    <Switch checked=airplane />
                                </div>
                            }
                        }
                        {
                            let notifs = RwSignal::new(true);
                            view! {
                                <div class="flex items-center justify-between max-w-xs">
                                    <div class="flex flex-col gap-0.5">
                                        <Label>"Push notifications"</Label>
                                        <span class="text-xs text-muted-foreground">
                                            "Receive alerts on your device."
                                        </span>
                                    </div>
                                    <Switch checked=notifs />
                                </div>
                            }
                        }
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="Disabled in both states — the switch dims and stops responding."
                    code=DISABLED
                >
                    <div class="flex items-center gap-4">
                        {
                            let off = RwSignal::new(false);
                            view! { <Switch checked=off disabled=true /> }
                        }
                        {
                            let on = RwSignal::new(true);
                            view! { <Switch checked=on disabled=true /> }
                        }
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
