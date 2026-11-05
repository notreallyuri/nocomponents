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
            ty: "Option<RwSignal<bool>>",
            default: "None",
            description: "The state, held by the caller. Omit it and the switch keeps its own.",
        },
        Prop {
            name: "default_checked",
            ty: "bool",
            default: "false",
            description: "Whether it starts checked. Ignored when `checked` is given.",
        },
        Prop {
            name: "on_checked_change",
            ty: "Option<Callback<bool>>",
            default: "None",
            description: "Called with the new value whenever it is switched, including when the signal passed as `checked` is written from outside.",
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
        Prop {
            name: "name",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The name it is submitted under. A native `<input type=\"checkbox\">` kept beside the button carries it, so `FormData` sees the control; it is only rendered inside a `<form>` or when `name` or `form` is set, and it is `aria-hidden` and not a tab stop. Unchecked, nothing is submitted.",
        },
        Prop {
            name: "value",
            ty: "Signal<String>",
            default: "\"\"",
            description: "What it submits while checked. Empty is `\"on\"`, as a native checkbox has it.",
        },
        Prop {
            name: "required",
            ty: "Signal<bool>",
            default: "false",
            description: "The browser refuses to submit the form, and says so beside the control, while it is unchecked.",
        },
        Prop {
            name: "form",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The `id` of a `<form>` it belongs to when it is not inside one. Empty means the one around it.",
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
