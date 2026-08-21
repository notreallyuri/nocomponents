use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{
    button::{Button, ButtonVariant},
    spinner::{Spinner, SpinnerSize, SpinnerVariant},
};

const VARIANTS: &str = r#"<Spinner />
<Spinner variant=SpinnerVariant::Lined />"#;

const SIZES: &str = r#"<Spinner size=SpinnerSize::Xs />
<Spinner size=SpinnerSize::Sm />
<Spinner />
<Spinner size=SpinnerSize::Lg />"#;

const IN_A_BUTTON: &str = r#"<Button attr:disabled=true>
    <Spinner />
    "Saving"
</Button>
<Button variant=ButtonVariant::Outline attr:disabled=true>
    <Spinner variant=SpinnerVariant::Lined />
    "Loading"
</Button>"#;

const INHERITS_COLOR: &str = r#"<Spinner class="text-primary" />
<Spinner class="text-destructive" />
<Spinner class="text-muted-foreground" />"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "Spinner",
    description: "A busy indicator for something with no measurable progress.",
    props: &[
        Prop {
            name: "variant",
            ty: "SpinnerVariant",
            default: "Default",
            description: "One of: Default, Lined.",
        },
        Prop {
            name: "size",
            ty: "SpinnerSize",
            default: "Default",
            description: "One of: Xs, Sm, Default, Lg, Icon.",
        },
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the size classes.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout title="Spinner" description="Indicates that something is in progress.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Variants"
                    description="A circular arc or a spoked ring; both spin."
                    code=VARIANTS
                >
                    <div class="flex items-center gap-6">
                        <Spinner />
                        <Spinner variant=SpinnerVariant::Lined />
                    </div>
                </DemoSection>

                <DemoSection title="Sizes" description="Four sizes from Xs to Lg." code=SIZES>
                    <div class="flex items-center gap-6">
                        <Spinner size=SpinnerSize::Xs />
                        <Spinner size=SpinnerSize::Sm />
                        <Spinner />
                        <Spinner size=SpinnerSize::Lg />
                    </div>
                </DemoSection>

                <DemoSection
                    title="In a button"
                    description="Pair with disabled while work is in flight."
                    code=IN_A_BUTTON
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Button attr:disabled=true>
                            <Spinner />
                            "Saving"
                        </Button>
                        <Button variant=ButtonVariant::Outline attr:disabled=true>
                            <Spinner variant=SpinnerVariant::Lined />
                            "Loading"
                        </Button>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Inherits color"
                    description="The icon uses currentColor."
                    code=INHERITS_COLOR
                >
                    <div class="flex items-center gap-6">
                        <Spinner class="text-primary" />
                        <Spinner class="text-destructive" />
                        <Spinner class="text-muted-foreground" />
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
