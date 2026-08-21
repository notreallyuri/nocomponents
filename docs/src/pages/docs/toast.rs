use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    button::{Button, ButtonVariant},
    toast::{ToastPosition, ToastVariant, use_toast},
};

const VARIANTS: &str = r#"<Button
    variant=ButtonVariant::Outline
    on:click=move |_| {
        toast
            .toast("Saved successfully")
            .description("Your changes have been applied.")
            .show();
    }
>
    "Default"
</Button>
<Button
    variant=ButtonVariant::Destructive
    on:click=move |_| {
        toast
            .toast("Something went wrong")
            .description("Please try again later.")
            .variant(ToastVariant::Destructive)
            .show();
    }
>
    "Destructive"
</Button>"#;

const POSITIONS: &str = r#"<Button
    variant=ButtonVariant::Outline
    on:click=move |_| {
        toast.toast("Top left").position(ToastPosition::TopLeft).show();
    }
>
    "Top left"
</Button>
<Button
    variant=ButtonVariant::Outline
    on:click=move |_| {
        toast.toast("Top center").position(ToastPosition::TopCenter).show();
    }
>
    "Top center"
</Button>
<Button
    variant=ButtonVariant::Outline
    on:click=move |_| {
        toast.toast("Top right").position(ToastPosition::TopRight).show();
    }
>
    "Top right"
</Button>
<Button
    variant=ButtonVariant::Outline
    on:click=move |_| {
        toast
            .toast("Bottom left")
            .position(ToastPosition::BottomLeft)
            .show();
    }
>
    "Bottom left"
</Button>
<Button
    variant=ButtonVariant::Outline
    on:click=move |_| {
        toast
            .toast("Bottom center")
            .position(ToastPosition::BottomCenter)
            .show();
    }
>
    "Bottom center"
</Button>
<Button
    variant=ButtonVariant::Outline
    on:click=move |_| {
        toast
            .toast("Bottom right")
            .position(ToastPosition::BottomRight)
            .show();
    }
>
    "Bottom right"
</Button>"#;

const STACKING: &str = r#"<Button on:click=move |_| {
    toast
        .toast("First")
        .description("Toast 1 of 3.")
        .position(ToastPosition::BottomRight)
        .show();
    toast
        .toast("Second")
        .description("Toast 2 of 3.")
        .position(ToastPosition::BottomRight)
        .show();
    toast
        .toast("Third")
        .description("Toast 3 of 3.")
        .position(ToastPosition::BottomRight)
        .show();
}>"Stack 3 toasts"</Button>"#;

#[component]
pub fn Page() -> impl IntoView {
    let toast = use_toast();

    view! {
        <DocLayout
            title="Toast"
            description="Brief non-disruptive notifications that appear and disappear automatically."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Variants"
                    description="toast() opens a builder — chain a description or a variant, then show()."
                    code=VARIANTS
                >
                    <div class="flex flex-wrap gap-3">
                        <Button
                            variant=ButtonVariant::Outline
                            on:click=move |_| {
                                toast
                                    .toast("Saved successfully")
                                    .description("Your changes have been applied.")
                                    .show();
                            }
                        >
                            "Default"
                        </Button>
                        <Button
                            variant=ButtonVariant::Destructive
                            on:click=move |_| {
                                toast
                                    .toast("Something went wrong")
                                    .description("Please try again later.")
                                    .variant(ToastVariant::Destructive)
                                    .show();
                            }
                        >
                            "Destructive"
                        </Button>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Positions"
                    description="position() places a toast at any of the six corners and edges."
                    code=POSITIONS
                >
                    <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
                        <Button
                            variant=ButtonVariant::Outline
                            on:click=move |_| {
                                toast.toast("Top left").position(ToastPosition::TopLeft).show();
                            }
                        >
                            "Top left"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            on:click=move |_| {
                                toast.toast("Top center").position(ToastPosition::TopCenter).show();
                            }
                        >
                            "Top center"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            on:click=move |_| {
                                toast.toast("Top right").position(ToastPosition::TopRight).show();
                            }
                        >
                            "Top right"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            on:click=move |_| {
                                toast
                                    .toast("Bottom left")
                                    .position(ToastPosition::BottomLeft)
                                    .show();
                            }
                        >
                            "Bottom left"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            on:click=move |_| {
                                toast
                                    .toast("Bottom center")
                                    .position(ToastPosition::BottomCenter)
                                    .show();
                            }
                        >
                            "Bottom center"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            on:click=move |_| {
                                toast
                                    .toast("Bottom right")
                                    .position(ToastPosition::BottomRight)
                                    .show();
                            }
                        >
                            "Bottom right"
                        </Button>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Stacking"
                    description="Fire multiple toasts to the same position to see them stack."
                    code=STACKING
                >
                    <div class="flex gap-3">
                        <Button on:click=move |_| {
                            toast
                                .toast("First")
                                .description("Toast 1 of 3.")
                                .position(ToastPosition::BottomRight)
                                .show();
                            toast
                                .toast("Second")
                                .description("Toast 2 of 3.")
                                .position(ToastPosition::BottomRight)
                                .show();
                            toast
                                .toast("Third")
                                .description("Toast 3 of 3.")
                                .position(ToastPosition::BottomRight)
                                .show();
                        }>"Stack 3 toasts"</Button>
                    </div>
                </DemoSection>
            </div>
        </DocLayout>
    }
}
