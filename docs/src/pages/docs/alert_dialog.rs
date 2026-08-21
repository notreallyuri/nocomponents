use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
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

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "AlertDialog",
        description: "A dialog that interrupts on purpose, for something that cannot be undone. Not dismissed by an outside click or by Escape.",
        props: &[
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Pass one to drive it from outside; omit it and the root owns the signal.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A trigger and a portal.",
            },
        ],
    },
    ApiEntry {
        name: "AlertDialogTrigger",
        description: "Opens it. A `Button` unless `render` says otherwise.",
        props: &[
            Prop {
                name: "size",
                ty: "ButtonSize",
                default: "Default",
                description: "One of: Xs, Sm, Default, Lg, Icon, IconSm, IconXs, IconLg.",
            },
            Prop {
                name: "variant",
                ty: "ButtonVariant",
                default: "Default",
                description: "One of: Default, Outline, Ghost, Secondary, Link, Destructive.",
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
                ty: "Option<Callback<Callback<ev::MouseEvent>, AnyView>>",
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
        name: "AlertDialogPortal",
        description: "The overlay and the panel, rendered at the end of the document.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged onto the panel, not the overlay.",
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
        name: "AlertDialogHeader",
        description: "Stacks the title over the description.",
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
        name: "AlertDialogFooter",
        description: "The actions: cancel on the left, the irreversible one on the right.",
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
                description: "Usually a cancel and an action.",
            },
        ],
    },
    ApiEntry {
        name: "AlertDialogTitle",
        description: "Claims the id the panel points `aria-labelledby` at.",
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
        name: "AlertDialogDescription",
        description: "Claims the id for `aria-describedby`. This is where the consequence goes.",
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
        name: "AlertDialogAction",
        description: "The button that goes through with it. Closes the dialog after its handler runs.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
            Prop {
                name: "on_click",
                ty: "Option<Callback<ev::MouseEvent>>",
                default: "None",
                description: "Called before the dialog closes.",
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
        name: "AlertDialogCancel",
        description: "The way out. Closes without calling anything.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
            Prop {
                name: "on_click",
                ty: "Option<Callback<ev::MouseEvent>>",
                default: "None",
                description: "Called before the dialog closes.",
            },
            Prop {
                name: "children",
                ty: "Option<ChildrenFn>",
                default: "None",
                description: "The label.",
            },
        ],
    },
];

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

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
