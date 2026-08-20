use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    button::{Button, ButtonVariant},
    progress::Progress,
};

const DEFAULT: &str = r#"<Progress value=default_val class="w-full max-w-md" />"#;

const INDETERMINATE: &str = r#"<Progress value=none_val class="w-full max-w-md" />"#;

const CONTROLLED: &str = r#"<Progress value=progress />
<Button
    variant=ButtonVariant::Outline
    class="w-fit cursor-pointer"
    on_click=move |_| {
        progress
            .update(|p| {
                if let Some(val) = p {
                    *p = Some(if *val >= 100.0 { 0.0 } else { *val + 10.0 });
                }
            });
    }
>
    "Increase Progress"
</Button>"#;

#[component]
pub fn Page() -> impl IntoView {
    let default_val = Some(60.0);
    let none_val: Option<f64> = None;
    let progress = RwSignal::new(Some(10.0));

    view! {
        <DocLayout
            title="Progress"
            description="Displays an indicator showing the completion progress of a task, typically displayed as a progress bar."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="value is read against max, which defaults to 100."
                    code=DEFAULT
                >
                    <Progress value=default_val class="w-full max-w-md" />
                </DemoSection>

                <DemoSection
                    title="Indeterminate"
                    description="When the exact progress is unknown, the track pulses."
                    code=INDETERMINATE
                >
                    <Progress value=none_val class="w-full max-w-md" />
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Bind the value to a signal to animate the progress dynamically."
                    code=CONTROLLED
                >
                    <div class="flex flex-col gap-4 w-full max-w-md">
                        <Progress value=progress />
                        <Button
                            variant=ButtonVariant::Outline
                            class="w-fit cursor-pointer"
                            on_click=move |_| {
                                progress
                                    .update(|p| {
                                        if let Some(val) = p {
                                            *p = Some(if *val >= 100.0 { 0.0 } else { *val + 10.0 });
                                        }
                                    });
                            }
                        >
                            "Increase Progress"
                        </Button>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
