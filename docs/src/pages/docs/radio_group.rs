use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{
        label::Label,
        radio_group::{RadioGroup, RadioGroupItem},
    },
    utils::types::Orientation,
};

const DEFAULT: &str = r#"<RadioGroup>
    <Label class="gap-2">
        <RadioGroupItem value="comfortable" />
        "Comfortable"
    </Label>
    <Label class="gap-2">
        <RadioGroupItem value="compact" />
        "Compact"
    </Label>
    <Label class="gap-2">
        <RadioGroupItem value="spacious" />
        "Spacious"
    </Label>
</RadioGroup>"#;

const HORIZONTAL: &str = r#"<RadioGroup orientation=Orientation::Horizontal>
    <Label class="gap-2">
        <RadioGroupItem value="card" />
        "Card"
    </Label>
    <Label class="gap-2">
        <RadioGroupItem value="paypal" />
        "PayPal"
    </Label>
    <Label class="gap-2">
        <RadioGroupItem value="transfer" />
        "Bank transfer"
    </Label>
</RadioGroup>"#;

const DISABLED: &str = r#"<RadioGroup>
    <Label class="gap-2">
        <RadioGroupItem value="standard" />
        "Standard delivery"
    </Label>
    <Label class="gap-2 opacity-50">
        <RadioGroupItem value="overnight" disabled=true />
        "Overnight (unavailable)"
    </Label>
    <Label class="gap-2">
        <RadioGroupItem value="pickup" />
        "Collect in store"
    </Label>
</RadioGroup>"#;

const CONTROLLED: &str = r#"<RadioGroup value=plan>
    <Label class="gap-2">
        <RadioGroupItem value="free" />
        "Free"
    </Label>
    <Label class="gap-2">
        <RadioGroupItem value="pro" />
        "Pro"
    </Label>
</RadioGroup>
<span class="text-sm text-muted-foreground">
    "Selected: " {move || plan.get().unwrap_or_else(|| "none".to_string())}
</span>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "RadioGroup",
        description: "A set of radios where exactly one may be chosen. One tab stop, and the arrows both move and select.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the group's classes.",
            },
            Prop {
                name: "orientation",
                ty: "Orientation",
                default: "Orientation::Vertical",
                description: "Radios stack by default, the way a native group does; the arrow keys follow suit.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables every radio in the group.",
            },
            Prop {
                name: "value",
                ty: "Option<RwSignal<Option<String>>>",
                default: "None",
                description: "Omit for an uncontrolled group that owns its own selection.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Items, usually each beside a `Label`.",
            },
        ],
    },
    ApiEntry {
        name: "RadioGroupItem",
        description: "One radio. Focusing it with the arrows also chooses it.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "What choosing this sets the group's value to.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the radio's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Skipped by the arrow keys as well as the pointer.",
            },
            Prop {
                name: "id",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Given to the control, so a `Label` can point `for` at it.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let plan = RwSignal::new(Some("pro".to_string()));

    view! {
        <DocLayout
            title="Radio Group"
            description="A set of options where exactly one can be chosen."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The group is a single tab stop; the arrow keys move between options and select as they go."
                    code=DEFAULT
                >
                    <RadioGroup>
                        <Label class="gap-2">
                            <RadioGroupItem value="comfortable" />
                            "Comfortable"
                        </Label>
                        <Label class="gap-2">
                            <RadioGroupItem value="compact" />
                            "Compact"
                        </Label>
                        <Label class="gap-2">
                            <RadioGroupItem value="spacious" />
                            "Spacious"
                        </Label>
                    </RadioGroup>
                </DemoSection>

                <DemoSection
                    title="Horizontal"
                    description="A horizontal group takes Left and Right instead of Up and Down."
                    code=HORIZONTAL
                >
                    <RadioGroup orientation=Orientation::Horizontal>
                        <Label class="gap-2">
                            <RadioGroupItem value="card" />
                            "Card"
                        </Label>
                        <Label class="gap-2">
                            <RadioGroupItem value="paypal" />
                            "PayPal"
                        </Label>
                        <Label class="gap-2">
                            <RadioGroupItem value="transfer" />
                            "Bank transfer"
                        </Label>
                    </RadioGroup>
                </DemoSection>

                <DemoSection
                    title="Disabled option"
                    description="A disabled radio cannot be chosen and the arrow keys skip past it."
                    code=DISABLED
                >
                    <RadioGroup>
                        <Label class="gap-2">
                            <RadioGroupItem value="standard" />
                            "Standard delivery"
                        </Label>
                        <Label class="gap-2 opacity-50">
                            <RadioGroupItem value="overnight" disabled=true />
                            "Overnight (unavailable)"
                        </Label>
                        <Label class="gap-2">
                            <RadioGroupItem value="pickup" />
                            "Collect in store"
                        </Label>
                    </RadioGroup>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Pass a signal to read or preset the choice; Tab lands on whichever option is checked."
                    code=CONTROLLED
                >
                    <div class="flex flex-col gap-3">
                        <RadioGroup value=plan>
                            <Label class="gap-2">
                                <RadioGroupItem value="free" />
                                "Free"
                            </Label>
                            <Label class="gap-2">
                                <RadioGroupItem value="pro" />
                                "Pro"
                            </Label>
                        </RadioGroup>
                        <span class="text-sm text-muted-foreground">
                            "Selected: " {move || plan.get().unwrap_or_else(|| "none".to_string())}
                        </span>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
