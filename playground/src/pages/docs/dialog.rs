use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};
use leptos::prelude::*;
use nocomponents::components::{
    button::{Button, ButtonVariant},
    dialog::{
        Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPortal, DialogTitle,
        DialogTrigger,
    },
    input::Input,
    label::Label,
};

const CONFIRMATION: &str = r#"<Dialog>
    <DialogTrigger variant=ButtonVariant::Destructive>
        "Delete account"
    </DialogTrigger>
    <DialogPortal>
        <DialogHeader>
            <DialogTitle>"Are you absolutely sure?"</DialogTitle>
            <DialogDescription>
                "This action cannot be undone. Your account and all associated data will be permanently removed."
            </DialogDescription>
        </DialogHeader>
        <DialogFooter class="mt-4">
            <Button variant=ButtonVariant::Outline>"Cancel"</Button>
            <Button variant=ButtonVariant::Destructive>"Delete account"</Button>
        </DialogFooter>
    </DialogPortal>
</Dialog>"#;

const FORM: &str = r#"<Dialog>
    <DialogTrigger variant=ButtonVariant::Outline>"Edit profile"</DialogTrigger>
    <DialogPortal>
        <DialogHeader>
            <DialogTitle>"Edit profile"</DialogTitle>
            <DialogDescription>
                "Update your public information. Click save when you're done."
            </DialogDescription>
        </DialogHeader>
        <div class="grid gap-4 py-4">
            <div class="grid grid-cols-4 items-center gap-4">
                <Label class="text-right">"Name"</Label>
                <Input class="col-span-3" attr:placeholder="Your name" />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
                <Label class="text-right">"Username"</Label>
                <Input class="col-span-3" attr:placeholder="@handle" />
            </div>
        </div>
        <DialogFooter>
            <Button>"Save changes"</Button>
        </DialogFooter>
    </DialogPortal>
</Dialog>"#;

const CONTROLLED: &str = r#"{
    let open = RwSignal::new(false);
    view! {
        <div class="flex items-center gap-3">
            <Button on:click=move |_| open.set(true)>"Open dialog"</Button>
            <span class="text-sm text-muted-foreground">
                {move || if open.get() { "Open" } else { "Closed" }}
            </span>
            <Dialog open=open>
                <DialogPortal>
                    <DialogHeader>
                        <DialogTitle>"Controlled dialog"</DialogTitle>
                        <DialogDescription>
                            "This dialog's open state is managed by an external signal."
                        </DialogDescription>
                    </DialogHeader>
                    <DialogFooter class="mt-4">
                        <Button on:click=move |_| open.set(false)>"Close"</Button>
                    </DialogFooter>
                </DialogPortal>
            </Dialog>
        </div>
    }
}"#;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Dialog"
            description="A modal window that interrupts the user to focus on a specific task."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Confirmation"
                    description="The overlay and Escape both close it; focus is held inside the content while it is open."
                    code=CONFIRMATION
                >
                    <Dialog>
                        <DialogTrigger variant=ButtonVariant::Destructive>
                            "Delete account"
                        </DialogTrigger>
                        <DialogPortal>
                            <DialogHeader>
                                <DialogTitle>"Are you absolutely sure?"</DialogTitle>
                                <DialogDescription>
                                    "This action cannot be undone. Your account and all associated data will be permanently removed."
                                </DialogDescription>
                            </DialogHeader>
                            <DialogFooter class="mt-4">
                                <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                                <Button variant=ButtonVariant::Destructive>"Delete account"</Button>
                            </DialogFooter>
                        </DialogPortal>
                    </Dialog>
                </DemoSection>

                <DemoSection
                    title="Form"
                    description="A dialog that collects user input."
                    code=FORM
                >
                    <Dialog>
                        <DialogTrigger variant=ButtonVariant::Outline>"Edit profile"</DialogTrigger>
                        <DialogPortal>
                            <DialogHeader>
                                <DialogTitle>"Edit profile"</DialogTitle>
                                <DialogDescription>
                                    "Update your public information. Click save when you're done."
                                </DialogDescription>
                            </DialogHeader>
                            <div class="grid gap-4 py-4">
                                <div class="grid grid-cols-4 items-center gap-4">
                                    <Label class="text-right">"Name"</Label>
                                    <Input class="col-span-3" attr:placeholder="Your name" />
                                </div>
                                <div class="grid grid-cols-4 items-center gap-4">
                                    <Label class="text-right">"Username"</Label>
                                    <Input class="col-span-3" attr:placeholder="@handle" />
                                </div>
                            </div>
                            <DialogFooter>
                                <Button>"Save changes"</Button>
                            </DialogFooter>
                        </DialogPortal>
                    </Dialog>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Open state managed externally."
                    code=CONTROLLED
                >
                    {
                        let open = RwSignal::new(false);
                        view! {
                            <div class="flex items-center gap-3">
                                <Button on:click=move |_| open.set(true)>"Open dialog"</Button>
                                <span class="text-sm text-muted-foreground">
                                    {move || if open.get() { "Open" } else { "Closed" }}
                                </span>
                                <Dialog open=open>
                                    <DialogPortal>
                                        <DialogHeader>
                                            <DialogTitle>"Controlled dialog"</DialogTitle>
                                            <DialogDescription>
                                                "This dialog's open state is managed by an external signal."
                                            </DialogDescription>
                                        </DialogHeader>
                                        <DialogFooter class="mt-4">
                                            <Button on:click=move |_| open.set(false)>"Close"</Button>
                                        </DialogFooter>
                                    </DialogPortal>
                                </Dialog>
                            </div>
                        }
                    }
                </DemoSection>
            </div>
        </DocLayout>
    }
}
