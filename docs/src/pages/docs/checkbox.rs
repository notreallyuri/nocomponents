use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::{
    components::{checkbox::Checkbox, label::Label},
    utils::selection::RangeSelection,
};

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

const RANGE: &str = r#"let rows = RangeSelection::new(8usize, |index| index.to_string());

view! {
    <div class="flex flex-col gap-2">
        {(0..8)
            .map(|index| {
                view! {
                    <div class="flex items-center gap-2">
                        <Checkbox
                            checked=rows.checked(index)
                            on_click=rows.on_click(index)
                            attr:aria-label=format!("Row {}", index + 1)
                        />
                        <Label>{format!("Row {}", index + 1)}</Label>
                    </div>
                }
            })
            .collect_view()}
    </div>
}"#;

const API: &[ApiEntry] = &[
    ApiEntry {
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
                name: "on_click",
                ty: "Option<Callback<ev::MouseEvent>>",
                default: "None",
                description: "Run on every click, before the box toggles, with the click itself. For what the event knows and the change does not: shift-click range selection reads `shift_key()` here, and the change arrives right after. Not run while the box is disabled.",
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
    },
    ApiEntry {
        name: "RangeSelection",
        description: "A Copy handle in nocomponents::utils::selection that keeps the anchor and the selected keys, so a Shift-click selecting a range is the library's and not the caller's. A plain click sets the one row and moves the anchor to it; a Shift-click sets every row from the anchor to the clicked one to the state the clicked row just took, and moves the anchor there. With no anchor a Shift-click is a plain click. Rows are named by key, so the anchor survives a reorder, and the indices need not be in the page. Space on a focused box counts: the browser reports the key's Shift on the click it produces.",
        props: &[
            Prop {
                name: "RangeSelection::new(count, key)",
                ty: "(impl Into<Signal<usize>>, Fn(usize) -> String)",
                default: "",
                description: "A selection over a list of count rows whose row at an index is named by key. RangeSelection::with_selected takes a set of keys you already hold.",
            },
            Prop {
                name: "selected",
                ty: "RwSignal<HashSet<String>>",
                default: "",
                description: "The selected keys. Pass it as `selected` to Sortable or SortableWindow, read it, or write it.",
            },
            Prop {
                name: "checked(index)",
                ty: "RwSignal<bool>",
                default: "",
                description: "For a Checkbox's `checked`: the row's state, which follows the selection, so the rows a Shift-click reaches change with it.",
            },
            Prop {
                name: "on_click(index)",
                ty: "Callback<ev::MouseEvent>",
                default: "",
                description: "For a Checkbox's `on_click`: sets the row to the state the click gives it, and with Shift held the run from the anchor.",
            },
            Prop {
                name: "select(index, checked, shift)",
                ty: "fn(usize, bool, bool)",
                default: "",
                description: "The rule itself, for a control that is not a Checkbox. toggle(index, shift) takes the opposite of what the row has.",
            },
            Prop {
                name: "is_selected(index)",
                ty: "fn(usize) -> bool",
                default: "",
                description: "Whether the row is selected. Tracked. len() and is_empty() count them; anchor() is where the next Shift-click measures from.",
            },
            Prop {
                name: "select_all() / clear()",
                ty: "fn()",
                default: "",
                description: "Select every row of the list, or none. Both forget the anchor.",
            },
        ],
    },
];

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
                    title="Range selection"
                    description="RangeSelection keeps the anchor, so a caller writes two props. Click a row, then Shift-click another to set everything between; Space with Shift held does the same from the keyboard. The anchor is the row clicked last, so Shift-clicking a checked row clears the run back to it."
                    code=RANGE
                >
                    {
                        let rows = RangeSelection::new(8usize, |index| index.to_string());
                        view! {
                            <div class="flex flex-col gap-2">
                                {(0..8)
                                    .map(|index| {
                                        view! {
                                            <div class="flex items-center gap-2">
                                                <Checkbox
                                                    checked=rows.checked(index)
                                                    on_click=rows.on_click(index)
                                                    attr:aria-label=format!("Row {}", index + 1)
                                                />
                                                <Label>{format!("Row {}", index + 1)}</Label>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                                <p class="text-xs text-muted-foreground" data-testid="range-count">
                                    {move || format!("{} selected", rows.len())}
                                </p>
                            </div>
                        }
                    }
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
