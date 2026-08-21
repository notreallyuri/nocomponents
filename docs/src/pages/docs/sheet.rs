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
        button::{Button, ButtonVariant},
        input::Input,
        label::Label,
        sheet::{
            Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader,
            SheetTitle, SheetTrigger,
        },
    },
    utils::types::Side,
};

const DEFAULT: &str = r#"<Sheet>
    <SheetTrigger variant=ButtonVariant::Outline>"Open sheet"</SheetTrigger>
    <SheetContent>
        <SheetHeader>
            <SheetTitle>"Edit profile"</SheetTitle>
            <SheetDescription>"Change your details and save when you are done."</SheetDescription>
        </SheetHeader>
        <div class="flex flex-col gap-3">
            <div class="flex flex-col gap-1.5">
                <Label>"Name"</Label>
                <Input attr:placeholder="Your name" />
            </div>
        </div>
        <SheetFooter>
            <SheetClose>"Cancel"</SheetClose>
            <Button>"Save"</Button>
        </SheetFooter>
    </SheetContent>
</Sheet>"#;

const SIDES: &str = r#"<Sheet>
    <SheetTrigger variant=ButtonVariant::Outline>"Left"</SheetTrigger>
    <SheetContent side=Side::Left>
        <SheetHeader>
            <SheetTitle>"From the left"</SheetTitle>
        </SheetHeader>
    </SheetContent>
</Sheet>"#;

const CONTROLLED: &str = r#"<Button on_click=Callback::new(move |_| open.set(true))>"Open"</Button>
<Sheet open=open>
    <SheetContent side=Side::Bottom>
        <SheetHeader>
            <SheetTitle>"Controlled"</SheetTitle>
            <SheetDescription>"Open state managed externally."</SheetDescription>
        </SheetHeader>
    </SheetContent>
</Sheet>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Sheet",
        description: "A dialog that arrives from an edge. The dialog's modality and focus trap, plus one `side`.",
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
                description: "A trigger and a content.",
            },
        ],
    },
    ApiEntry {
        name: "SheetTrigger",
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
        name: "SheetContent",
        description: "The panel, pinned to an edge and sliding in from it.",
        props: &[
            Prop {
                name: "side",
                ty: "Side",
                default: "Side::Right",
                description: "One of: Left, Right, Bottom, Top.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the panel's classes.",
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
        name: "SheetHeader",
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
        name: "SheetTitle",
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
        name: "SheetDescription",
        description: "Claims the id for `aria-describedby`.",
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
        name: "SheetFooter",
        description: "The actions, pushed to the bottom of the panel.",
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
    ApiEntry {
        name: "SheetClose",
        description: "A button that closes the sheet, for a footer's \"Cancel\" or \"Done\".",
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
                default: "ButtonVariant::Outline",
                description: "One of: Default, Outline, Ghost, Secondary, Link, Destructive.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the button's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The label.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <DocLayout title="Sheet" description="A dialog that arrives from an edge.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The dialog primitive underneath, so Escape, the focus trap and the overlay all behave the same — a sheet only changes which edge it is pinned to."
                    code=DEFAULT
                >
                    <Sheet>
                        <SheetTrigger variant=ButtonVariant::Outline>"Open sheet"</SheetTrigger>
                        <SheetContent>
                            <SheetHeader>
                                <SheetTitle>"Edit profile"</SheetTitle>
                                <SheetDescription>
                                    "Change your details and save when you are done."
                                </SheetDescription>
                            </SheetHeader>
                            <div class="flex flex-col gap-3">
                                <div class="flex flex-col gap-1.5">
                                    <Label>"Name"</Label>
                                    <Input attr:placeholder="Your name" />
                                </div>
                                <div class="flex flex-col gap-1.5">
                                    <Label>"Username"</Label>
                                    <Input attr:placeholder="@handle" />
                                </div>
                            </div>
                            <SheetFooter>
                                <SheetClose>"Cancel"</SheetClose>
                                <Button>"Save"</Button>
                            </SheetFooter>
                        </SheetContent>
                    </Sheet>
                </DemoSection>

                <DemoSection
                    title="Sides"
                    description="Each side pins the sheet to that edge and slides it in from there."
                    code=SIDES
                >
                    <div class="flex flex-wrap gap-3">
                        <Sheet>
                            <SheetTrigger variant=ButtonVariant::Outline>"Left"</SheetTrigger>
                            <SheetContent side=Side::Left>
                                <SheetHeader>
                                    <SheetTitle>"From the left"</SheetTitle>
                                    <SheetDescription>"Pinned to the left edge."</SheetDescription>
                                </SheetHeader>
                            </SheetContent>
                        </Sheet>
                        <Sheet>
                            <SheetTrigger variant=ButtonVariant::Outline>"Right"</SheetTrigger>
                            <SheetContent side=Side::Right>
                                <SheetHeader>
                                    <SheetTitle>"From the right"</SheetTitle>
                                    <SheetDescription>"The default."</SheetDescription>
                                </SheetHeader>
                            </SheetContent>
                        </Sheet>
                        <Sheet>
                            <SheetTrigger variant=ButtonVariant::Outline>"Top"</SheetTrigger>
                            <SheetContent side=Side::Top>
                                <SheetHeader>
                                    <SheetTitle>"From the top"</SheetTitle>
                                    <SheetDescription>
                                        "Full width, height from content."
                                    </SheetDescription>
                                </SheetHeader>
                            </SheetContent>
                        </Sheet>
                        <Sheet>
                            <SheetTrigger variant=ButtonVariant::Outline>"Bottom"</SheetTrigger>
                            <SheetContent side=Side::Bottom>
                                <SheetHeader>
                                    <SheetTitle>"From the bottom"</SheetTitle>
                                    <SheetDescription>
                                        "Where a phone would put it."
                                    </SheetDescription>
                                </SheetHeader>
                            </SheetContent>
                        </Sheet>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Open state managed externally, so any control can raise the sheet."
                    code=CONTROLLED
                >
                    <div class="flex items-center gap-3">
                        <Button on_click=Callback::new(move |_| open.set(true))>"Open"</Button>
                        <span class="text-sm text-muted-foreground">
                            {move || if open.get() { "Open" } else { "Closed" }}
                        </span>
                        <Sheet open=open>
                            <SheetContent side=Side::Bottom>
                                <SheetHeader>
                                    <SheetTitle>"Controlled"</SheetTitle>
                                    <SheetDescription>
                                        "Open state managed externally."
                                    </SheetDescription>
                                </SheetHeader>
                                <SheetFooter>
                                    <SheetClose>"Done"</SheetClose>
                                </SheetFooter>
                            </SheetContent>
                        </Sheet>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
