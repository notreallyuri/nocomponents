use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{checkbox::Checkbox, label::Label};

const DEFAULT: &str = r#"{
    let checked = RwSignal::new(false);
    view! {
        <div class="flex items-center gap-2">
            <Checkbox id="default" checked=checked />
            <Label>"Accept terms and conditions"</Label>
        </div>
    }
}"#;

const CHECKED_BY_DEFAULT: &str = r#"{
    let checked = RwSignal::new(true);
    view! {
        <div class="flex items-center gap-2">
            <Checkbox id="pre-checked" checked=checked />
            <Label>"Receive marketing emails"</Label>
        </div>
    }
}"#;

const DISABLED: &str = r#"{
    let checked = RwSignal::new(false);
    view! {
        <div class="flex items-center gap-2">
            <Checkbox
                id="disabled-unchecked"
                checked=checked
                disabled=true
            />
            <Label class="opacity-50">"Unavailable option"</Label>
        </div>
    }
}
{
    let checked = RwSignal::new(true);
    view! {
        <div class="flex items-center gap-2">
            <Checkbox id="disabled-checked" checked=checked disabled=true />
            <Label class="opacity-50">"Locked setting"</Label>
        </div>
    }
}"#;

const INDETERMINATE: &str = r#"{
    let items = RwSignal::new(vec![true, false, false]);
    let some = Signal::derive(move || {
        items.with(|items| items.iter().any(|on| *on) && !items.iter().all(|on| *on))
    });
    let all = RwSignal::new(false);
    Effect::new(move |_| all.set(items.with(|items| items.iter().all(|on| *on))));
    view! {
        <div class="flex flex-col gap-2">
            <div class="flex items-center gap-2">
                <Checkbox
                    id="select-all"
                    checked=all
                    indeterminate=some
                    attr:aria-label="Select all"
                    on:click=move |_| {
                        let next = !items.with_untracked(|items| items.iter().all(|on| *on));
                        items.update(|items| items.iter_mut().for_each(|on| *on = next));
                    }
                />
                <Label>"Select all"</Label>
            </div>
            <div class="ml-6 flex flex-col gap-2">
                {(0..3)
                    .map(|index| {
                        let on = RwSignal::new(items.with_untracked(|items| items[index]));
                        Effect::new(move |_| on.set(items.with(|items| items[index])));
                        view! {
                            <div class="flex items-center gap-2">
                                <Checkbox
                                    checked=on
                                    attr:aria-label=format!("Item {}", index + 1)
                                    on:click=move |_| {
                                        items.update(|items| items[index] = !items[index])
                                    }
                                />
                                <Label>{format!("Item {}", index + 1)}</Label>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}"#;

const API: &[ApiEntry] = &[ApiEntry {
    name: "Checkbox",
    description: "A box that is ticked or not. Renders a real checkbox role, not a styled input.",
    props: &[
        Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the box's own classes.",
        },
        Prop {
            name: "checked",
            ty: "Option<RwSignal<bool>>",
            default: "None",
            description: "The state, held by the caller. Omit it and the checkbox keeps its own.",
        },
        Prop {
            name: "default_checked",
            ty: "bool",
            default: "false",
            description: "Whether it starts checked. Ignored when `checked` is given.",
        },
        Prop {
            name: "on_checked_change",
            ty: "Option<Callback<bool>>",
            default: "None",
            description: "Called with the new value whenever it is checked or unchecked, including when the signal passed as `checked` is written from outside.",
        },
        Prop {
            name: "indeterminate",
            ty: "Signal<bool>",
            default: "false",
            description: "Shows \"some, not all\" over `checked`: `aria-checked=\"mixed\"` and `data-state=\"indeterminate\"`. Clicking an indeterminate box checks it and calls `on_checked_change(true)`, even when `checked` was already true; clearing `indeterminate` is the caller's. A form submits it as unchecked.",
        },
        Prop {
            name: "disabled",
            ty: "Signal<bool>",
            default: "false",
            description: "Stops it taking pointer events and marks it disabled.",
        },
        Prop {
            name: "id",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Given to the control, so a `Label` can point `for` at it. Inside a `Field` this is filled in.",
        },
        Prop {
            name: "name",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The name it is submitted under. A native `<input type=\"checkbox\">` kept beside the button carries it, so `FormData` sees the control; it is only rendered inside a `<form>` or when `name` or `form` is set, and it is `aria-hidden` and not a tab stop. Unchecked, nothing is submitted.",
        },
        Prop {
            name: "value",
            ty: "Signal<String>",
            default: "\"\"",
            description: "What it submits while checked. Empty is `\"on\"`, as a native checkbox has it.",
        },
        Prop {
            name: "required",
            ty: "Signal<bool>",
            default: "false",
            description: "The browser refuses to submit the form, and says so beside the control, while it is unchecked.",
        },
        Prop {
            name: "form",
            ty: "Signal<String>",
            default: "\"\"",
            description: "The `id` of a `<form>` it belongs to when it is not inside one. Empty means the one around it.",
        },
        Prop {
            name: "icon",
            ty: "Option<ViewFn>",
            default: "Check",
            description: "The glyph, in place of the check mark. Any view — an `Icon`, an SVG of your own; see Icons.",
        },
        Prop {
            name: "indeterminate_icon",
            ty: "Option<ViewFn>",
            default: "Minus",
            description: "The glyph shown while `indeterminate` is on, in place of the dash.",
        },
    ],
}];

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <DocLayout
            title="Checkbox"
            description="A control that allows the user to toggle between checked and unchecked."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="checked takes a signal, so the state stays with the caller."
                    code=DEFAULT
                >
                    {
                        let checked = RwSignal::new(false);
                        view! {
                            <div class="flex items-center gap-2">
                                <Checkbox id="default" checked=checked />
                                <Label>"Accept terms and conditions"</Label>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Checked by default"
                    description="Start the signal at true to render checked on mount."
                    code=CHECKED_BY_DEFAULT
                >
                    {
                        let checked = RwSignal::new(true);
                        view! {
                            <div class="flex items-center gap-2">
                                <Checkbox id="pre-checked" checked=checked />
                                <Label>"Receive marketing emails"</Label>
                            </div>
                        }
                    }
                </DemoSection>

                <DemoSection
                    title="Disabled"
                    description="A disabled checkbox keeps its state but ignores pointer and keyboard."
                    code=DISABLED
                >
                    <div class="flex flex-col gap-3">
                        {
                            let checked = RwSignal::new(false);
                            view! {
                                <div class="flex items-center gap-2">
                                    <Checkbox
                                        id="disabled-unchecked"
                                        checked=checked
                                        disabled=true
                                    />
                                    <Label class="opacity-50">"Unavailable option"</Label>
                                </div>
                            }
                        }
                        {
                            let checked = RwSignal::new(true);
                            view! {
                                <div class="flex items-center gap-2">
                                    <Checkbox id="disabled-checked" checked=checked disabled=true />
                                    <Label class="opacity-50">"Locked setting"</Label>
                                </div>
                            }
                        }
                    </div>
                </DemoSection>

                <DemoSection
                    title="Indeterminate"
                    description="`indeterminate` draws over `checked` for a parent box whose children are only partly on. A click checks it; the caller decides what the children do."
                    code=INDETERMINATE
                >
                    {
                        let items = RwSignal::new(vec![true, false, false]);
                        let some = Signal::derive(move || {
                            items
                                .with(|items| {
                                    items.iter().any(|on| *on) && !items.iter().all(|on| *on)
                                })
                        });
                        let all = RwSignal::new(false);
                        Effect::new(move |_| {
                            all.set(items.with(|items| items.iter().all(|on| *on)))
                        });
                        view! {
                            <div class="flex flex-col gap-2">
                                <div class="flex items-center gap-2">
                                    <Checkbox
                                        id="select-all"
                                        checked=all
                                        indeterminate=some
                                        attr:aria-label="Select all"
                                        on:click=move |_| {
                                            let next = !items
                                                .with_untracked(|items| items.iter().all(|on| *on));
                                            items
                                                .update(|items| items.iter_mut().for_each(|on| *on = next));
                                        }
                                    />
                                    <Label>"Select all"</Label>
                                </div>
                                <div class="ml-6 flex flex-col gap-2">
                                    {(0..3)
                                        .map(|index| {
                                            let on = RwSignal::new(
                                                items.with_untracked(|items| items[index]),
                                            );
                                            Effect::new(move |_| {
                                                on.set(items.with(|items| items[index]))
                                            });
                                            view! {
                                                <div class="flex items-center gap-2">
                                                    <Checkbox
                                                        checked=on
                                                        attr:aria-label=format!("Item {}", index + 1)
                                                        on:click=move |_| {
                                                            items.update(|items| items[index] = !items[index])
                                                        }
                                                    />
                                                    <Label>{format!("Item {}", index + 1)}</Label>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </div>
                        }
                    }
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
