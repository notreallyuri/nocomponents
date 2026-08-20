use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::toggle::{Toggle, ToggleSize, ToggleVariant};

const VARIANTS: &str = r#"<Toggle>"Default"</Toggle>
<Toggle variant=ToggleVariant::Outline>"Outline"</Toggle>"#;

const CONTROLLED: &str = r#"{
    let pressed = RwSignal::new(true);
    view! {
        <div class="flex items-center gap-3">
            <Toggle variant=ToggleVariant::Outline pressed=pressed>
                "Airplane mode"
            </Toggle>
            <span class="text-sm text-muted-foreground">
                {move || if pressed.get() { "On" } else { "Off" }}
            </span>
        </div>
    }
}"#;

const SIZES: &str = r#"<Toggle size=ToggleSize::Xs>"Xs"</Toggle>
<Toggle size=ToggleSize::Sm>"Sm"</Toggle>
<Toggle>"Default"</Toggle>
<Toggle size=ToggleSize::Lg>"Lg"</Toggle>"#;

const DISABLED: &str = r#"<Toggle disabled=true>"Off"</Toggle>
<Toggle variant=ToggleVariant::Outline disabled=true>
    "Outline"
</Toggle>"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Toggle" description="A two-state button that can be on or off.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Variants"
                    description="Click one — a pressed toggle keeps a filled background."
                    code=VARIANTS
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Toggle>"Default"</Toggle>
                        <Toggle variant=ToggleVariant::Outline>"Outline"</Toggle>
                    </div>
                </DemoSection>

                <DemoSection title="Sizes" description="Xs through Lg." code=SIZES>
                    <div class="flex flex-wrap items-center gap-3">
                        <Toggle size=ToggleSize::Xs>"Xs"</Toggle>
                        <Toggle size=ToggleSize::Sm>"Sm"</Toggle>
                        <Toggle>"Default"</Toggle>
                        <Toggle size=ToggleSize::Lg>"Lg"</Toggle>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled toggle keeps its pressed state but ignores clicks."
                    code=DISABLED
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Toggle disabled=true>"Off"</Toggle>
                        <Toggle variant=ToggleVariant::Outline disabled=true>
                            "Outline"
                        </Toggle>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Pass a signal to own the pressed state; omit it and the toggle keeps its own."
                    code=CONTROLLED
                >
                    {
                        let pressed = RwSignal::new(true);
                        view! {
                            <div class="flex items-center gap-3">
                                <Toggle variant=ToggleVariant::Outline pressed=pressed>
                                    "Airplane mode"
                                </Toggle>
                                <span class="text-sm text-muted-foreground">
                                    {move || if pressed.get() { "On" } else { "Off" }}
                                </span>
                            </div>
                        }
                    }
                </DemoSection>
            </div>
        </DocLayout>
    }
}
