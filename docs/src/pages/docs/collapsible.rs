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
    code::Code,
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
};

const DEFAULT: &str = r#"<Collapsible class="w-full max-w-md">
    <CollapsibleTrigger class="border px-3 py-2">
        "@yuri starred 3 repositories"
        <span class="text-muted-foreground">"⌄"</span>
    </CollapsibleTrigger>
    <CollapsibleContent>
        <div class="flex flex-col gap-2">
            <div class="rounded-lg border px-3 py-2 font-mono text-xs">"leptos-rs/leptos"</div>
            <div class="rounded-lg border px-3 py-2 font-mono text-xs">"rust-lang/rust"</div>
        </div>
    </CollapsibleContent>
</Collapsible>"#;

const CONTROLLED: &str = r#"<Button
    variant=ButtonVariant::Outline
    on_click=Callback::new(move |_| open.update(|o| *o = !*o))
>
    {move || if open.get() { "Hide details" } else { "Show details" }}
</Button>
<Collapsible open=open class="w-full max-w-md">
    <CollapsibleContent>
        <p class="rounded-lg border p-3 text-muted-foreground">"…"</p>
    </CollapsibleContent>
</Collapsible>"#;

const DISABLED: &str = r#"<Collapsible disabled=true class="w-full max-w-md">
    <CollapsibleTrigger class="border px-3 py-2">"Locked section"</CollapsibleTrigger>
    <CollapsibleContent>
        <p class="p-3">"Never reachable."</p>
    </CollapsibleContent>
</Collapsible>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Collapsible",
        description: "One panel that opens and closes, with no selection to share.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the wrapper's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Disables the trigger.",
            },
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Omit for an uncontrolled panel that owns its own state.",
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
        name: "CollapsibleTrigger",
        description: "Toggles the panel.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the trigger's classes.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Stops it toggling.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The trigger's label.",
            },
        ],
    },
    ApiEntry {
        name: "CollapsibleContent",
        description: "The panel, measured after it mounts so the animation has a height to run to.",
        props: &[
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
];

#[component]
pub fn Page() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <DocLayout title="Collapsible" description="A panel that opens and closes.">
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="The content is measured after it mounts and its height published as --nc-content-height, which is what the open and close keyframes animate to."
                    code=DEFAULT
                >
                    <Collapsible class="w-full max-w-md">
                        <CollapsibleTrigger class="rounded-lg border px-3 py-2">
                            "@yuri starred 3 repositories"
                            <span class="text-muted-foreground">"⌄"</span>
                        </CollapsibleTrigger>
                        <CollapsibleContent>
                            <div class="flex flex-col gap-2">
                                <div class="rounded-lg border px-3 py-2 font-mono text-xs">
                                    "leptos-rs/leptos"
                                </div>
                                <div class="rounded-lg border px-3 py-2 font-mono text-xs">
                                    "rust-lang/rust"
                                </div>
                                <div class="rounded-lg border px-3 py-2 font-mono text-xs">
                                    "tailwindlabs/tailwindcss"
                                </div>
                            </div>
                        </CollapsibleContent>
                    </Collapsible>
                </DemoSection>

                <DemoSection
                    title="Controlled"
                    description="Pass a signal and the trigger becomes optional — anything can open the panel."
                    code=CONTROLLED
                >
                    <div class="flex w-full max-w-md flex-col gap-3">
                        <Button
                            variant=ButtonVariant::Outline
                            class="w-fit"
                            on_click=Callback::new(move |_| open.update(|o| *o = !*o))
                        >
                            {move || if open.get() { "Hide details" } else { "Show details" }}
                        </Button>
                        <Collapsible open=open>
                            <CollapsibleContent>
                                <p class="rounded-lg border p-3 text-muted-foreground">
                                    "The panel keeps its content mounted while it closes, so the exit animation has something to run on — the same trick the dialog and the floating layers use."
                                </p>
                            </CollapsibleContent>
                        </Collapsible>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled collapsible dims its trigger and ignores clicks."
                    code=DISABLED
                >
                    <Collapsible disabled=true class="w-full max-w-md">
                        <CollapsibleTrigger class="rounded-lg border px-3 py-2">
                            "Locked section" <span class="text-muted-foreground">"⌄"</span>
                        </CollapsibleTrigger>
                        <CollapsibleContent>
                            <p class="p-3">"Never reachable."</p>
                        </CollapsibleContent>
                    </Collapsible>
                </DemoSection>

                <DemoSection
                    title="Under the accordion"
                    description="Accordion is this component plus a shared selection; see its page for the stacked version."
                >
                    <p class="text-sm text-muted-foreground">
                        "Each accordion item owns a " <Code>"CollapsibleContext"</Code>
                        ", so the panel machinery is not written twice."
                    </p>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
