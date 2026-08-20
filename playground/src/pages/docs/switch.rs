use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
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
    view! { <Switch checked=off disabled=true /> }
}
{
    let on = RwSignal::new(true);
    view! { <Switch checked=on disabled=true /> }
}"#;

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
            </div>
        </DocLayout>
    }
}
