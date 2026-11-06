//! The sortable page: a list that reports a reorder, and a caller that applies it.

use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::prelude::*;
use nocomponents::components::{
    checkbox::Checkbox,
    sortable::{Sortable, SortableHandle, SortableItem, SortableMove, apply_move},
    switch::Switch,
};
use nocomponents::utils::types::Orientation;
use std::collections::HashSet;

const DEFAULT: &str = r#"// The list never touches its data. It says what moved and where, and
// `apply_move` is the whole integration.
let items = RwSignal::new(vec![
    "Sketch".to_string(),
    "Prototype".to_string(),
    "Review".to_string(),
    "Ship".to_string(),
]);

let on_move = move |moved: SortableMove| {
    items.update(|items| apply_move(items, &moved, |item| item.clone()));
};

<Sortable label="Launch steps" on_move=on_move>
    <For each=move || items.get() key=|item| item.clone() let:item>
        <SortableItem value=item.clone() label=item.clone()>
            <SortableHandle />
            <span>{item.clone()}</span>
        </SortableItem>
    </For>
</Sortable>"#;

const BLOCK: &str = r#"let items = RwSignal::new(/* "Cover", "Front", "Back", ... */);
let selected = RwSignal::new(HashSet::<String>::new());

let on_move = move |moved: SortableMove| {
    items.update(|items| apply_move(items, &moved, |item| item.clone()));
};

<Sortable label="Pages" selected=selected on_move=on_move>
    <For each=move || items.get() key=|item| item.clone() let:item>
        <SortableItem value=item.clone() label=item.clone()>
            <SortableHandle />
            <Checkbox on_checked_change={
                let item = item.clone();
                move |on| selected.update(|set| {
                    if on { set.insert(item.clone()); } else { set.remove(&item); }
                })
            } />
            <span>{item.clone()}</span>
        </SortableItem>
    </For>
</Sortable>"#;

const HORIZONTAL: &str = r#"let disabled = RwSignal::new(false);

<Sortable
    label="Tags"
    orientation=Orientation::Horizontal
    disabled=disabled
    on_move=on_move
>
    <For each=move || items.get() key=|item| item.clone() let:item>
        <SortableItem value=item.clone()>{item.clone()}</SortableItem>
    </For>
</Sortable>

<Switch checked=disabled />"#;

const SCROLLING: &str = r#"<Sortable
    label="Queue"
    class="max-h-56 overflow-y-auto pr-1"
    on_move=on_move
>
    <For each=move || items.get() key=|item| item.clone() let:item>
        <SortableItem value=item.clone() label=item.clone()>
            <SortableHandle />
            <span>{item.clone()}</span>
        </SortableItem>
    </For>
</Sortable>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Sortable",
        description: "The list. It owns the gesture, the live region and the instructions a screen reader hears, and nothing about the order — it reports a move and the caller applies it.",
        props: &[
            Prop {
                name: "on_move",
                ty: "Callback<SortableMove>",
                default: "",
                description: "Items were let go somewhere new. SortableMove { keys, to }: keys are the moved items in their current relative order, to is the index the block lands at in the list once those items have left it. Apply it with apply_move. Never fired for a drop back where it started.",
            },
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The list's accessible name.",
            },
            Prop {
                name: "orientation",
                ty: "Signal<Orientation>",
                default: "Vertical",
                description: "Which way the list runs: which arrow keys move an item, which way it scrolls. One axis only — a list, not a grid.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Nothing can be picked up, by pointer or keyboard. Handles and items go out of the tab order and say aria-disabled.",
            },
            Prop {
                name: "selected",
                ty: "Signal<HashSet<String>>",
                default: "{}",
                description: "The selected keys. Picking up one of them takes all of them, by pointer or keyboard, and they land as one block in their current relative order. The picked item carries a data-count of how many.",
            },
            Prop {
                name: "instructions",
                ty: "Signal<String>",
                default: "\"\"",
                description: "What handles are described by. Empty means the English default: Press Space or Enter to pick up an item, and so on.",
            },
            Prop {
                name: "announce",
                ty: "Option<Callback<SortableEvent, String>>",
                default: "None",
                description: "Replaces the live-region wording wholesale. SortableEvent is PickedUp, Moved, Dropped or Cancelled, each with the item's label, its one-based position, the total and how many are in the block. None keeps default_announcement.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the list's classes. The list is the thing that scrolls when it overflows: give it a height and overflow-y-auto and it scrolls while an item is held near its edge — the list, not the page, so it works inside a dialog that locks the body.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The items.",
            },
        ],
    },
    ApiEntry {
        name: "SortableItem",
        description: "One item. With a SortableHandle inside it, only the handle drags and takes focus; without one, the whole item does — and a press on a button, link or input inside it is never a drag.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "What a move reports it as. Unique within the list.",
            },
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "What the live region calls the item, and what the handle's name is built from. Without one, the item's text — which includes a handle's, so pass one when the handle has text.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "This item cannot be picked up. Others can still move past it.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes. It wears data-dragging, data-following, data-placeholder, data-anchor, data-count, data-selected and data-disabled.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The item's content.",
            },
        ],
    },
    ApiEntry {
        name: "SortableHandle",
        description: "The part of an item that starts a drag and takes keyboard focus. Space or Enter picks up, the arrows move, Home and End jump, Space or Enter drops, Escape cancels.",
        props: &[
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The handle's accessible name. Without one: Reorder, followed by the item's label.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the handle's classes.",
            },
            Prop {
                name: "children",
                ty: "Option<Children>",
                default: "a grip icon",
                description: "What the handle shows.",
            },
        ],
    },
];

fn steps(names: &[&str]) -> RwSignal<Vec<String>> {
    RwSignal::new(names.iter().map(|name| name.to_string()).collect())
}

#[component]
pub fn Page() -> impl IntoView {
    let items = steps(&["Sketch", "Prototype", "Review", "Ship"]);
    let last = RwSignal::new(None::<SortableMove>);
    let on_move = move |moved: SortableMove| {
        items.update(|items| apply_move(items, &moved, |item| item.clone()));
        last.set(Some(moved));
    };

    let pages = steps(&["Cover", "Front", "Back", "Index", "Notes"]);
    let selected = RwSignal::new(HashSet::<String>::new());
    let block_move = move |moved: SortableMove| {
        pages.update(|pages| apply_move(pages, &moved, |item| item.clone()));
    };

    let tags = steps(&["rust", "leptos", "wasm", "tailwind"]);
    let disabled = RwSignal::new(false);
    let tag_move = move |moved: SortableMove| {
        tags.update(|tags| apply_move(tags, &moved, |item| item.clone()));
    };

    let queue = steps(&[
        "Track 01", "Track 02", "Track 03", "Track 04", "Track 05", "Track 06", "Track 07",
        "Track 08", "Track 09", "Track 10", "Track 11", "Track 12",
    ]);
    let queue_move = move |moved: SortableMove| {
        queue.update(|queue| apply_move(queue, &moved, |item| item.clone()));
    };

    view! {
        <DocLayout
            title="Sortable"
            description="A list you can put in order with a pointer, a finger or the keyboard alone. The list reports the move; the order stays yours."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Drag a handle, or focus one and press Space, move it with the arrow keys, and press Space again to drop — Escape puts it back. Each step is announced to a screen reader. The list does not reorder anything itself: it reports SortableMove { keys, to } and apply_move does the rest. to counts the list without the moved items, so 0 is the front and the same call works for one item or a whole block."
                    code=DEFAULT
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <Sortable label="Launch steps" on_move=on_move>
                            <For each=move || items.get() key=|item| item.clone() let:item>
                                <SortableItem value=item.clone() label=item.clone()>
                                    <SortableHandle />
                                    <span>{item.clone()}</span>
                                </SortableItem>
                            </For>
                        </Sortable>

                        <p class="text-xs text-muted-foreground" data-testid="sortable-last">
                            {move || match last.get() {
                                None => "No move yet.".to_string(),
                                Some(moved) => {
                                    format!("Moved {} to {}", moved.keys.join(", "), moved.to)
                                }
                            }}
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Block drag"
                    description="Tick some items, then pick up any ticked one: they all go together, keeping their order, and land as one block. The held item shows how many are riding under it. The keyboard does the same — the block moves a slot at a time."
                    code=BLOCK
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <Sortable label="Pages" selected=selected on_move=block_move>
                            <For each=move || pages.get() key=|item| item.clone() let:item>
                                <SortableItem value=item.clone() label=item.clone()>
                                    <SortableHandle />
                                    <Checkbox on_checked_change={
                                        let item = item.clone();
                                        move |on| {
                                            selected
                                                .update(|set| {
                                                    if on {
                                                        set.insert(item.clone());
                                                    } else {
                                                        set.remove(&item);
                                                    }
                                                })
                                        }
                                    } />
                                    <span>{item.clone()}</span>
                                </SortableItem>
                            </For>
                        </Sortable>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Horizontal, and disabled"
                    description="Left and Right move an item along a horizontal list, and with no handle the whole item is what you pick up. Switching the list off takes every item out of the tab order."
                    code=HORIZONTAL
                >
                    <div class="flex w-full flex-col gap-3">
                        <Sortable
                            label="Tags"
                            orientation=Orientation::Horizontal
                            disabled=disabled
                            on_move=tag_move
                        >
                            <For each=move || tags.get() key=|item| item.clone() let:item>
                                <SortableItem value=item.clone()>{item.clone()}</SortableItem>
                            </For>
                        </Sortable>

                        <div class="flex items-center gap-2 text-xs text-muted-foreground">
                            <Switch checked=disabled />
                            "Disabled"
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Scrolling"
                    description="Hold an item near the top or bottom of the list and it scrolls. It is the list's own scroller that moves, never the page behind it, so the same list works inside a dialog whose body is locked. Give the list a height and overflow-y-auto; the nearest ancestor that actually scrolls is the one used."
                    code=SCROLLING
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <Sortable
                            label="Queue"
                            class="max-h-56 overflow-y-auto pr-1"
                            on_move=queue_move
                        >
                            <For each=move || queue.get() key=|item| item.clone() let:item>
                                <SortableItem value=item.clone() label=item.clone()>
                                    <SortableHandle />
                                    <span>{item.clone()}</span>
                                </SortableItem>
                            </For>
                        </Sortable>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
