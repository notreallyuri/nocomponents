use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    alert_dialog::{
        AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogDescription,
        AlertDialogFooter, AlertDialogHeader, AlertDialogPortal, AlertDialogTitle,
        AlertDialogTrigger,
    },
    button::{Button, ButtonVariant},
};

const DESTRUCTIVE_CONFIRMATION: &str = r#"<AlertDialog>
    <AlertDialogTrigger variant=ButtonVariant::Destructive>
        "Delete project"
    </AlertDialogTrigger>
    <AlertDialogPortal>
        <AlertDialogHeader>
            <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
            <AlertDialogDescription>
                "This permanently deletes the project and everything in it. This action cannot be undone."
            </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter class="mt-4">
            <AlertDialogCancel>"Cancel"</AlertDialogCancel>
            <AlertDialogAction>"Delete"</AlertDialogAction>
        </AlertDialogFooter>
    </AlertDialogPortal>
</AlertDialog>"#;

const CONTROLLED: &str = r#"{
    let open = RwSignal::new(false);
    view! {
        <div class="flex items-center gap-3">
            <Button
                variant=ButtonVariant::Outline
                on:click=move |_| open.set(true)
            >
                "Sign out everywhere"
            </Button>
            <span class="text-sm text-muted-foreground">
                {move || if open.get() { "Open" } else { "Closed" }}
            </span>
            <AlertDialog open=open>
                <AlertDialogPortal>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"End all sessions?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "You will be signed out on every device, including this one."
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter class="mt-4">
                        <AlertDialogCancel>"Stay signed in"</AlertDialogCancel>
                        <AlertDialogAction>"Sign out"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogPortal>
            </AlertDialog>
        </div>
    }
}"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Alert Dialog"
            description="A modal that interrupts the user with an important choice, requiring an explicit response."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Destructive confirmation"
                    description="Unlike a dialog, dismissing is deliberate — cancel or confirm."
                    code=DESTRUCTIVE_CONFIRMATION
                >
                    <AlertDialog>
                        <AlertDialogTrigger variant=ButtonVariant::Destructive>
                            "Delete project"
                        </AlertDialogTrigger>
                        <AlertDialogPortal>
                            <AlertDialogHeader>
                                <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                                <AlertDialogDescription>
                                    "This permanently deletes the project and everything in it. This action cannot be undone."
                                </AlertDialogDescription>
                            </AlertDialogHeader>
                            <AlertDialogFooter class="mt-4">
                                <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                                <AlertDialogAction>"Delete"</AlertDialogAction>
                            </AlertDialogFooter>
                        </AlertDialogPortal>
                    </AlertDialog>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Open state managed externally, so any control can raise the prompt."
                    code=CONTROLLED
                >
                    {
                        let open = RwSignal::new(false);
                        view! {
                            <div class="flex items-center gap-3">
                                <Button
                                    variant=ButtonVariant::Outline
                                    on:click=move |_| open.set(true)
                                >
                                    "Sign out everywhere"
                                </Button>
                                <span class="text-sm text-muted-foreground">
                                    {move || if open.get() { "Open" } else { "Closed" }}
                                </span>
                                <AlertDialog open=open>
                                    <AlertDialogPortal>
                                        <AlertDialogHeader>
                                            <AlertDialogTitle>"End all sessions?"</AlertDialogTitle>
                                            <AlertDialogDescription>
                                                "You will be signed out on every device, including this one."
                                            </AlertDialogDescription>
                                        </AlertDialogHeader>
                                        <AlertDialogFooter class="mt-4">
                                            <AlertDialogCancel>"Stay signed in"</AlertDialogCancel>
                                            <AlertDialogAction>"Sign out"</AlertDialogAction>
                                        </AlertDialogFooter>
                                    </AlertDialogPortal>
                                </AlertDialog>
                            </div>
                        }
                    }
                </DemoSection>
            </div>
        </DocLayout>
    }
}
