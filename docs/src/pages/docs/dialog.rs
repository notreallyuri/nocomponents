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
                name: "default_open",
                ty: "bool",
                default: "false",
                description: "Whether the dialog starts open. Ignored when `open` is given.",
            },
            Prop {
                name: "on_open_change",
                ty: "Option<Callback<bool>>",
                default: "None",
                description: "Called with the new value whenever the dialog opens or closes, including when the signal passed as `open` is written from outside.",
            },
            Prop {
                name: "modal",
                ty: "bool",
                default: "true",
                description: "Non-modal leaves the page usable underneath: no overlay, no focus trap, and an outside pointer closes it.",
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
                ty: "Signal<ButtonVariant>",
                default: "Default",
                description: "Forwarded to the underlying `Button`.",
            },
            Prop {
                name: "size",
                ty: "Signal<ButtonSize>",
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
                ty: "Option<Callback<TriggerRender, AnyView>>",
                default: "None",
                description: "Render the trigger as something else. Handed the node ref to attach, the click to run and the resolved `disabled`; the dialog writes its ARIA onto that node.",
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
                ty: "Signal<bool>",
                default: "true",
                description: "The corner close button. Off for a panel that is itself the control, as the command palette is.",
            },
            Prop {
                name: "mobile_sheet",
                ty: "Signal<bool>",
                default: "false",
                description: "Below the `md` breakpoint (768px), present the panel as a bottom sheet instead of a centred panel: no positioning, `data-presentation=\"sheet\"`. At `md` and up nothing changes, and it is written as `\"floating\"`. The overlay is the scrim, and Escape and a tap on it close the dialog as before.",
            },
            Prop {
                name: "sheet_title",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The title row under the sheet's grabber. Empty means a grabber and no row. Ignored when the content is not a sheet.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The panel's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
            Prop {
                name: "close_icon",
                ty: "Option<ViewFn>",
                default: "X",
                description: "The glyph, in place of the close cross. Any view — an `Icon`, an SVG of your own; see Icons.",
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
        name: "DialogClose",
        description: "A button that closes the dialog — a footer's Cancel. The styled `Button`, outlined by default.",
        props: &[
            Prop {
                name: "size",
                ty: "Signal<ButtonSize>",
                default: "Default",
                description: "The button's size.",
            },
            Prop {
                name: "variant",
                ty: "Signal<ButtonVariant>",
                default: "Outline",
                description: "The button's variant.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
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
            description="A modal window that interrupts the user to focus on a specific task. Set `mobile_sheet` and it becomes a bottom sheet on phones; the Dropdown Menu page shows it."
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
