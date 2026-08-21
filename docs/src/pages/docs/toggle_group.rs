use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::{
    components::{
        toggle::{ToggleSize, ToggleVariant},
        toggle_group::{ToggleGroup, ToggleGroupItem},
    },
    primitives::toggle_group::ToggleGroupType,
    utils::types::Orientation,
};

const SINGLE: &str = r#"<ToggleGroup variant=ToggleVariant::Outline>
    <ToggleGroupItem value="left">"Left"</ToggleGroupItem>
    <ToggleGroupItem value="center">"Center"</ToggleGroupItem>
    <ToggleGroupItem value="right">"Right"</ToggleGroupItem>
</ToggleGroup>"#;

const MULTIPLE: &str = r#"<ToggleGroup kind=ToggleGroupType::Multiple variant=ToggleVariant::Outline>
    <ToggleGroupItem value="bold">"Bold"</ToggleGroupItem>
    <ToggleGroupItem value="italic">"Italic"</ToggleGroupItem>
    <ToggleGroupItem value="underline">"Underline"</ToggleGroupItem>
</ToggleGroup>"#;

const SIZES: &str = r#"<ToggleGroup size=ToggleSize::Sm>
    <ToggleGroupItem value="a">"Sm"</ToggleGroupItem>
    <ToggleGroupItem value="b">"Sm"</ToggleGroupItem>
</ToggleGroup>
<ToggleGroup size=ToggleSize::Lg>
    <ToggleGroupItem value="a">"Lg"</ToggleGroupItem>
    <ToggleGroupItem value="b">"Lg"</ToggleGroupItem>
</ToggleGroup>"#;

const VERTICAL: &str = r#"<ToggleGroup orientation=Orientation::Vertical variant=ToggleVariant::Outline>
    <ToggleGroupItem value="top">"Top"</ToggleGroupItem>
    <ToggleGroupItem value="middle">"Middle"</ToggleGroupItem>
    <ToggleGroupItem value="bottom">"Bottom"</ToggleGroupItem>
</ToggleGroup>"#;

const CONTROLLED: &str = r#"{
    let value = RwSignal::new(vec!["bold".to_string()]);
    view! {
        <div class="flex items-center gap-4">
            <ToggleGroup
                kind=ToggleGroupType::Multiple
                value=value
                variant=ToggleVariant::Outline
            >
                <ToggleGroupItem value="bold">"Bold"</ToggleGroupItem>
                <ToggleGroupItem value="italic">"Italic"</ToggleGroupItem>
            </ToggleGroup>
            <span class="text-sm text-muted-foreground">
                {move || {
                    let v = value.get();
                    if v.is_empty() { "none".to_string() } else { v.join(", ") }
                }}
            </span>
        </div>
    }
}"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Toggle Group"
            description="A set of two-state buttons that can be toggled on or off together."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Single"
                    description="At most one item stays pressed; clicking the pressed item clears it."
                    code=SINGLE
                >
                    <ToggleGroup variant=ToggleVariant::Outline>
                        <ToggleGroupItem value="left">"Left"</ToggleGroupItem>
                        <ToggleGroupItem value="center">"Center"</ToggleGroupItem>
                        <ToggleGroupItem value="right">"Right"</ToggleGroupItem>
                    </ToggleGroup>
                </DemoSection>

                <DemoSection
                    title="Multiple"
                    description="Any number of items can be pressed."
                    code=MULTIPLE
                >
                    <ToggleGroup kind=ToggleGroupType::Multiple variant=ToggleVariant::Outline>
                        <ToggleGroupItem value="bold">"Bold"</ToggleGroupItem>
                        <ToggleGroupItem value="italic">"Italic"</ToggleGroupItem>
                        <ToggleGroupItem value="underline">"Underline"</ToggleGroupItem>
                    </ToggleGroup>
                </DemoSection>

                <DemoSection
                    title="Sizes"
                    description="The group sets the size once and every item inherits it."
                    code=SIZES
                >
                    <div class="flex flex-col items-start gap-3">
                        <ToggleGroup size=ToggleSize::Sm>
                            <ToggleGroupItem value="a">"Sm"</ToggleGroupItem>
                            <ToggleGroupItem value="b">"Sm"</ToggleGroupItem>
                        </ToggleGroup>
                        <ToggleGroup size=ToggleSize::Lg>
                            <ToggleGroupItem value="a">"Lg"</ToggleGroupItem>
                            <ToggleGroupItem value="b">"Lg"</ToggleGroupItem>
                        </ToggleGroup>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Vertical"
                    description="A vertical orientation stacks the items."
                    code=VERTICAL
                >
                    <ToggleGroup orientation=Orientation::Vertical variant=ToggleVariant::Outline>
                        <ToggleGroupItem value="top">"Top"</ToggleGroupItem>
                        <ToggleGroupItem value="middle">"Middle"</ToggleGroupItem>
                        <ToggleGroupItem value="bottom">"Bottom"</ToggleGroupItem>
                    </ToggleGroup>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Pass a signal to read or preset the selection."
                    code=CONTROLLED
                >
                    {
                        let value = RwSignal::new(vec!["bold".to_string()]);
                        view! {
                            <div class="flex items-center gap-4">
                                <ToggleGroup
                                    kind=ToggleGroupType::Multiple
                                    value=value
                                    variant=ToggleVariant::Outline
                                >
                                    <ToggleGroupItem value="bold">"Bold"</ToggleGroupItem>
                                    <ToggleGroupItem value="italic">"Italic"</ToggleGroupItem>
                                </ToggleGroup>
                                <span class="text-sm text-muted-foreground">
                                    {move || {
                                        let v = value.get();
                                        if v.is_empty() { "none".to_string() } else { v.join(", ") }
                                    }}
                                </span>
                            </div>
                        }
                    }
                </DemoSection>
            </div>
        </DocLayout>
    }
}
