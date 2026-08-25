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

const ACTION: &str = r#"let present = RwSignal::new(true);

// The toast is often raised by something on its way out — the row being
// deleted, the dialog being closed — so its signals belong to the provider
// rather than to whoever called show(). Pressing the action dismisses it:
// one still offering to undo would undo twice.
view! {
    <Button on:click=move |_| {
        present.set(false);
        toast
            .toast("Deleted 'Q3 forecast'")
            .description("Undo puts it back where it was.")
            .action("Undo", move || present.set(true))
            .show();
    }>"Delete something"</Button>

    <span>{move || if present.get() { "Present" } else { "Deleted" }}</span>
}"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "ToastProvider",
    description: "Owns the queue and renders it. Wrap the app in one; toasts are pushed with `use_toast()`.",
    props: &[
        Prop {
            name: "default_variant",
            ty: "Option<ToastVariant>",
            default: "None",
            description: "What a toast looks like unless it says otherwise.",
        },
        Prop {
            name: "default_size",
            ty: "Option<ToastSize>",
            default: "None",
            description: "What size a toast is unless it says otherwise.",
        },
        Prop {
            name: "default_position",
            ty: "Option<ToastPosition>",
            default: "None",
            description: "Which corner they stack in unless a toast says otherwise.",
        },
        Prop {
            name: "children",
            ty: "Children",
            default: "",
            description: "The app.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    let toast = use_toast();
    let present = RwSignal::new(true);

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
                    title="An action"
                    description="action() takes a label and a closure — the button in the toast, and what it does. Pressing it runs the closure and dismisses the toast, since the thing being announced has just been answered."
                    code=ACTION
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Button
                            variant=ButtonVariant::Outline
                            disabled=Signal::derive(move || !present.get())
                            on:click=move |_| {
                                present.set(false);
                                toast
                                    .toast("Deleted 'Q3 forecast'")
                                    .description("Undo puts it back where it was.")
                                    .action("Undo", move || present.set(true))
                                    .show();
                            }
                        >
                            "Delete something"
                        </Button>
                        <span class="text-sm text-muted-foreground">
                            {move || {
                                if present.get() {
                                    "'Q3 forecast' is present."
                                } else {
                                    "'Q3 forecast' is deleted — until the toast goes."
                                }
                            }}
                        </span>
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

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
