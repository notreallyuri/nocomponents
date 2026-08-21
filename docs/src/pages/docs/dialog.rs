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

/// Read off the `#[component]` signatures in `src/components/dialog.rs`, in declaration order.
const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Dialog",
        description: "The root. Owns the open state and provides it to every part below.",
        props: &[
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Pass one to drive the dialog from outside; omit it and the root owns the signal.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The trigger and the portal.",
            },
        ],
    },
    ApiEntry {
        name: "DialogTrigger",
        description: "Opens it. A `Button` unless `render` says otherwise.",
        props: &[
            Prop {
                name: "variant",
                ty: "ButtonVariant",
                default: "Default",
                description: "Forwarded to the underlying `Button`.",
            },
            Prop {
                name: "size",
                ty: "ButtonSize",
                default: "Default",
                description: "Forwarded to the underlying `Button`.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's own classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Rendered as the `disabled` attribute.",
            },
            Prop {
                name: "render",
                ty: "Option<Callback<Callback<MouseEvent>, AnyView>>",
                default: "None",
                description: "Render the trigger as something else. Handed the click handler, which must reach the element.",
            },
            Prop {
                name: "children",
                ty: "Option<ChildrenFn>",
                default: "None",
                description: "The label.",
            },
        ],
    },
    ApiEntry {
        name: "DialogPortal",
        description: "The overlay and the panel, rendered at the end of the document. Stays mounted for 150ms after closing so the exit animation can run.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged onto the panel, not the overlay.",
            },
            Prop {
                name: "show_close",
                ty: "bool",
                default: "true",
                description: "The corner close button. Off for a panel that is itself the control, as the command palette is.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The panel's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
    ApiEntry {
        name: "DialogHeader",
        description: "Stacks the title over the description, clear of the close button.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually a title and a description.",
            },
        ],
    },
    ApiEntry {
        name: "DialogTitle",
        description: "Claims the id the panel points `aria-labelledby` at, so it is what the dialog is announced as.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the heading classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The title text.",
            },
        ],
    },
    ApiEntry {
        name: "DialogDescription",
        description: "Claims the id for `aria-describedby`. Leave it out and the dialog does not claim to have one.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the muted text classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The description text.",
            },
        ],
    },
    ApiEntry {
        name: "DialogFooter",
        description: "The actions, reversed into a column on a narrow viewport so the primary one is nearest the thumb.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the layout classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Usually buttons.",
            },
        ],
    },
];

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

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
