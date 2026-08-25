//! The dropzone page: two ways in, one way out.

use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::{prelude::*, web_sys::File};
use nocomponents::components::{
    badge::{Badge, BadgeVariant},
    dropzone::{Dropzone, DropzoneDescription, DropzoneLabel, DropzoneTrigger},
};

const DEFAULT: &str = r#"let files = RwSignal::new(Vec::<String>::new());

<Dropzone on_files=Callback::new(move |dropped: Vec<File>| {
    files.set(dropped.iter().map(|f| f.name()).collect());
})>
    <DropzoneLabel>"Drop files here"</DropzoneLabel>
    <DropzoneDescription>
        "or " <DropzoneTrigger>"browse"</DropzoneTrigger> " for them"
    </DropzoneDescription>
</Dropzone>"#;

const ACCEPT: &str = r#"// The same grammar the `accept` attribute uses. The browser applies it to
// the picker; a drop goes around the picker, so the zone applies it again —
// and hands back what it turned away.
<Dropzone
    accept="image/*,.pdf"
    on_files=Callback::new(move |taken: Vec<File>| { … })
    on_rejected=Callback::new(move |turned_away: Vec<File>| { … })
>
    <DropzoneLabel>"Images and PDFs"</DropzoneLabel>
</Dropzone>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Dropzone",
        description: "The target itself. Dropping and picking both arrive as one Vec<File>.",
        props: &[
            Prop {
                name: "on_files",
                ty: "Callback<Vec<File>>",
                default: "",
                description: "Every arrival, however it arrived. Never called with an empty list.",
            },
            Prop {
                name: "accept",
                ty: "Signal<String>",
                default: "\"\"",
                description: "A comma-separated list of .ext, type/subtype or type/*. Empty takes anything.",
            },
            Prop {
                name: "on_rejected",
                ty: "Option<Callback<Vec<File>>>",
                default: "None",
                description: "Files turned away by accept, whether dropped or picked. A file dialog's accept only decides what it offers — \"All files\" gets past it — so a pick is checked too.",
            },
            Prop {
                name: "multiple",
                ty: "bool",
                default: "true",
                description: "Whether the picker takes more than one file. A drop of several is still several.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Refuses drops and picks, and leaves the tab order.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the zone's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "What the zone says when it is empty.",
            },
        ],
    },
    ApiEntry {
        name: "DropzoneTrigger",
        description: "Opens the picker from inside the zone — for the \"or browse\" a drop target usually offers.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the trigger's classes.",
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
        name: "DropzoneLabel",
        description: "The zone's headline.",
        props: &[Prop {
            name: "children",
            ty: "Children",
            default: "",
            description: "The text.",
        }],
    },
    ApiEntry {
        name: "DropzoneDescription",
        description: "The quieter line under it.",
        props: &[Prop {
            name: "children",
            ty: "Children",
            default: "",
            description: "The text.",
        }],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let taken = RwSignal::new(Vec::<String>::new());
    // Its own pair: the two demos are separate zones, and sharing a signal would have a drop on
    // one of them quietly rewrite the other's list.
    let filtered = RwSignal::new(Vec::<String>::new());
    let rejected = RwSignal::new(Vec::<String>::new());

    view! {
        <DocLayout
            title="Dropzone"
            description="A drop target for files, and the file picker behind it. Both arrive the same way."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Drop files on it, click it, or tab to it and press Enter — all three open the same way in. Picking the same file twice running is two events, because the input is cleared after each one."
                    code=DEFAULT
                >
                    <div class="flex w-full max-w-md flex-col gap-3">
                        <Dropzone on_files=Callback::new(move |dropped: Vec<File>| {
                            taken.set(dropped.iter().map(|f| f.name()).collect());
                        })>
                            <DropzoneLabel>"Drop files here"</DropzoneLabel>
                            <DropzoneDescription>
                                "or " <DropzoneTrigger>"browse"</DropzoneTrigger> " for them"
                            </DropzoneDescription>
                        </Dropzone>
                        <Show when=move || !taken.get().is_empty()>
                            <div class="flex flex-wrap gap-1.5">
                                {move || {
                                    taken
                                        .get()
                                        .into_iter()
                                        .map(|name| view! { <Badge>{name}</Badge> })
                                        .collect_view()
                                }}
                            </div>
                        </Show>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Only some kinds"
                    description="accept is a suggestion everywhere else and a rule here. A file dialog only uses it to decide what to offer — every dialog has an \"All files\" escape — and a drop never consults it at all, so the zone checks both ways in and hands back what it turned away."
                    code=ACCEPT
                >
                    <div class="flex w-full max-w-md flex-col gap-3">
                        <Dropzone
                            accept="image/*,.pdf"
                            on_files=Callback::new(move |dropped: Vec<File>| {
                                filtered.set(dropped.iter().map(|f| f.name()).collect());
                            })
                            on_rejected=Callback::new(move |turned: Vec<File>| {
                                rejected.set(turned.iter().map(|f| f.name()).collect());
                            })
                        >
                            <DropzoneLabel>"Images and PDFs"</DropzoneLabel>
                            <DropzoneDescription>
                                "Anything else is handed back"
                            </DropzoneDescription>
                        </Dropzone>
                        <Show when=move || !filtered.get().is_empty()>
                            <div class="flex flex-wrap gap-1.5">
                                {move || {
                                    filtered
                                        .get()
                                        .into_iter()
                                        .map(|name| view! { <Badge>{name}</Badge> })
                                        .collect_view()
                                }}
                            </div>
                        </Show>
                        <Show when=move || !rejected.get().is_empty()>
                            <div class="flex flex-wrap gap-1.5">
                                {move || {
                                    rejected
                                        .get()
                                        .into_iter()
                                        .map(|name| {
                                            view! {
                                                <Badge variant=BadgeVariant::Destructive>{name}</Badge>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </div>
                        </Show>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
