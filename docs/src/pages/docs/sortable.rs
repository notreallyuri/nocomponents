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
    sortable::{
        Sortable, SortableHandle, SortableItem, SortableItemVariant, SortableMove, SortableWindow,
        SortableWindowHandle, SortableWindowMove, apply_move,
    },
    switch::Switch,
};
use nocomponents::utils::{selection::RangeSelection, types::Orientation};
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

const SIDEWAYS: &str = r#"let selected = RwSignal::new(HashSet::<String>::new());

<Sortable
    label="Crates"
    orientation=Orientation::Horizontal
    selected=selected
    on_move=on_move
>
    <For each=move || items.get() key=|item| item.clone() let:item>
        <SortableItem value=item.clone() label=item.clone()>
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

const SCROLLING: &str = r#"let selected = RwSignal::new(HashSet::<String>::new());

<Sortable
    label="Queue"
    class="max-h-56 overflow-y-auto pr-2"
    selected=selected
    on_move=on_move
>
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

const PLAIN: &str = r#"<Sortable label="Pinned" on_move=on_move class="gap-0.5">
    <For each=move || items.get() key=|item| item.clone() let:item>
        <SortableItem
            value=item.clone()
            label=item.clone()
            variant=SortableItemVariant::Plain
            class="group/row"
        >
            <SortableHandle class="size-5 opacity-0 group-hover/row:opacity-100 focus-visible:opacity-100" />
            <span>{item.clone()}</span>
        </SortableItem>
    </For>
</Sortable>"#;

const WINDOW: &str = r##"// The list holds no rows of its own: it asks for row `i` when `i` is in view.
let items = RwSignal::new((1..=5_000).collect::<Vec<u32>>());
let selecting = RwSignal::new(false);

// The selection is by key, so it survives a reorder and reaches rows that are not in the page.
let range = RangeSelection::new(
    Signal::derive(move || items.with(Vec::len)),
    move |i| items.with(|items| items[i].to_string()),
);

let on_move = move |moved: SortableWindowMove| {
    items.update(|items| moved.apply(items));
};

<SortableWindow
    label="Tracks"
    class="h-96"
    count=move || items.with(Vec::len)
    row_height=44.0
    key=move |i: usize| items.with(|items| items[i].to_string())
    item_label=move |i: usize| items.with(|items| format!("Track {:04}", items[i]))
    selected=range.selected
    row=move |i: usize| {
        let n = items.with(|items| items[i]);
        view! {
            {move || selecting.get().then(|| view! {
                <Checkbox
                    class="size-5 rounded-none"
                    checked=range.checked(i)
                    on_click=range.on_click(i)
                />
            })}
            <SortableWindowHandle />
            <span class="tabular-nums">{format!("Track {n:04}")}</span>
        }
        .into_any()
    }
    on_move=on_move
/>"##;

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
                description: "The selected keys. Picking up one of them takes all of them, by pointer or keyboard, and they land as one block in their current relative order. The picked item becomes the pile and carries a data-count of how many; the others in the block are data-hidden, not drawn, and the items that stay close up around the one gap the block lands in.",
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
                name: "progress",
                ty: "Option<Callback<SortableProgress, String>>",
                default: "None",
                description: "Replaces the wording of the polite live region, which says what the move is while it lasts and is empty otherwise. SortableProgress is the item's label, how many are in the block, the one-based position the block's first item would land at, and the total. None keeps default_progress: Moving 4 items. Destination position 12 of 5,000.",
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
                name: "variant",
                ty: "SortableItemVariant",
                default: "Card",
                description: "Card is a bordered, shadowed surface. Plain has no border, shadow or background at rest — a row that takes an accent background on hover, the look of a menu or sidebar entry.",
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
                description: "Merged over the item's classes. It wears data-dragging (in the block), data-following (the picked item, while a pointer holds it), data-placeholder (a keyboard move of one item), data-anchor, data-count (the pile, when the block is more than one), data-hidden (in the block but not the pile: not drawn), data-selected and data-disabled.",
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
            Prop {
                name: "icon",
                ty: "Option<ViewFn>",
                default: "Grip",
                description: "The glyph, in place of the grip (unused when `children` are given). Any view — an `Icon`, an SVG of your own; see Icons.",
            },
        ],
    },
    ApiEntry {
        name: "SortableWindow",
        description: "A sortable list for thousands of rows: only the rows in view (and a few more either side) are in the page, whatever the count. Every row is the same fixed height, which is what makes a row's place a multiplication — nothing is measured, so a drag costs the rows on screen and not the list. The list owns its scroller, so give it a height. Vertical only; a list of uneven rows wants Sortable. Give it `selected` and a selected row takes the whole selection with it as one block, rows that are not in the page included. It reports a SortableWindowMove and the order stays yours.",
        props: &[
            Prop {
                name: "count",
                ty: "Signal<usize>",
                default: "",
                description: "How many rows the list has.",
            },
            Prop {
                name: "row_height",
                ty: "f64",
                default: "",
                description: "The height of every row, in pixels. Fixed: row i sits at i times this.",
            },
            Prop {
                name: "overscan",
                ty: "usize",
                default: "6",
                description: "Rows kept in the page above and below the ones in view.",
            },
            Prop {
                name: "key",
                ty: "Callback<usize, String>",
                default: "",
                description: "Names the row at an index in the reported move. Reads your data, so it follows a reorder.",
            },
            Prop {
                name: "item_label",
                ty: "Option<Callback<usize, String>>",
                default: "the key",
                description: "What a screen reader is told the row at an index is called, and what its handle's name is built from. Taken from data, not from the page, since the row may not be rendered.",
            },
            Prop {
                name: "row",
                ty: "Callback<usize, AnyView>",
                default: "",
                description: "The content of the row at an index. Rows are positional: the node at index 41 stays the node at index 41, and this is re-run when a signal it reads changes, which is how a reorder reaches the rows in view.",
            },
            Prop {
                name: "row_class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over every row's classes. The rows wear data-index, data-dragging (the picked row), data-following (riding the pointer), data-anchor (the one picked up), data-count (on the anchor, when it holds a block: the row is a stack, so style it as a pile with a badge), data-selected and data-disabled, and carry aria-setsize and aria-posinset since most of the list is not in the page.",
            },
            Prop {
                name: "selected",
                ty: "Signal<HashSet<String>>",
                default: "empty",
                description: "The keys, as `key` names them, that are selected: the same prop Sortable takes. Picking up one of them takes all of them, in their current order, as one block; the rest of the rows collapse under the held one and the others close up around the gap. Found by key over the whole list once, at the pick-up, so a selected row that is not rendered moves too. Keyboard pick-up takes the block as well, and the announcements say how many. RangeSelection keeps this set, and the anchor of its Shift-clicks.",
            },
            Prop {
                name: "on_move",
                ty: "Callback<SortableWindowMove>",
                default: "",
                description: "A row, or a block, was let go somewhere new: a SortableWindowMove. moved.apply(&mut items) is the whole integration, for one row or a block; moved.as_move() is the same report as a SortableMove.",
            },
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The list's accessible name.",
            },
            Prop {
                name: "instructions",
                ty: "Signal<String>",
                default: "\"\"",
                description: "What a handle is described by. Empty means the English default.",
            },
            Prop {
                name: "announce",
                ty: "Option<Callback<SortableEvent, String>>",
                default: "default_announcement",
                description: "Replaces the English wording of the live region.",
            },
            Prop {
                name: "progress",
                ty: "Option<Callback<SortableProgress, String>>",
                default: "default_progress",
                description: "Replaces the English sentence of the polite live region, which is re-said as the destination changes and cleared on a drop. Say the list's own noun here and in announce.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Takes every row out of the tab order and refuses a pick-up.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the scroller's classes. Give it a height (h-96, max-h-[60vh]): the window is as many rows as that holds. It wears data-dragging while a move is in progress.",
            },
        ],
    },
    ApiEntry {
        name: "SortableWindowMove",
        description: "What on_move is handed. `to` counts the list without the moved rows, as SortableMove's does, so 0 is the front and the same call works for a row or a block. A block lands as one run at `to` in the order it had.",
        props: &[
            Prop {
                name: "key",
                ty: "String",
                default: "",
                description: "What `key` said of the row that was picked up.",
            },
            Prop {
                name: "from",
                ty: "usize",
                default: "",
                description: "The picked row's index at the pick-up.",
            },
            Prop {
                name: "to",
                ty: "usize",
                default: "",
                description: "The index the first moved row ends up at, in the list without the moved ones. For one row, the index it ends up at.",
            },
            Prop {
                name: "keys",
                ty: "Vec<String>",
                default: "",
                description: "Every moved row's key, in list order. Just `key` when there was no selection.",
            },
            Prop {
                name: "indices",
                ty: "Vec<usize>",
                default: "",
                description: "Every moved row's index at the pick-up, ascending. apply() hands them to move_block, which is why a row that was never rendered moves correctly.",
            },
            Prop {
                name: "apply(&mut Vec<T>) / as_move() / count() / is_block()",
                ty: "methods",
                default: "",
                description: "apply puts the block where it landed; as_move is the same report as a SortableMove; SortableWindowMove::single(key, from, to) builds the one-row case.",
            },
        ],
    },
    ApiEntry {
        name: "SortableWindowHandle",
        description: "The part of a row that starts a drag and takes focus, inside the content of row. With a handle only the handle starts a drag, so a touch on the rest of the row still scrolls the list; without one the whole row is picked up and touch scrolling on it is off. Keys as SortableHandle; when nothing is held, the arrows, Home and End move focus through the list, scrolling rows into the page as they go.",
        props: &[
            Prop {
                name: "label",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The handle's accessible name. Without one: Reorder, followed by the row's item_label.",
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
            Prop {
                name: "icon",
                ty: "Option<ViewFn>",
                default: "Grip",
                description: "The glyph, in place of the grip (unused when children are given).",
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

    let crates = steps(&[
        "rust", "leptos", "wasm", "tailwind", "trunk", "serde", "tokio", "axum",
    ]);
    let crates_selected = RwSignal::new(HashSet::<String>::new());
    let crates_move = move |moved: SortableMove| {
        crates.update(|crates| apply_move(crates, &moved, |item| item.clone()));
    };
    let queue_selected = RwSignal::new(HashSet::<String>::new());

    let pinned = steps(&["Inbox", "Drafts", "Sent", "Archive"]);
    let pinned_move = move |moved: SortableMove| {
        pinned.update(|pinned| apply_move(pinned, &moved, |item| item.clone()));
    };

    let tracks = RwSignal::new((1..=5_000).collect::<Vec<u32>>());
    let selecting = RwSignal::new(false);
    let range = RangeSelection::new(Signal::derive(move || tracks.with(Vec::len)), move |i| {
        tracks.with(|tracks| tracks.get(i).map(|n| n.to_string()).unwrap_or_default())
    });
    let window_last = RwSignal::new(None::<SortableWindowMove>);
    let window_move = move |moved: SortableWindowMove| {
        tracks.update(|tracks| moved.apply(tracks));
        window_last.set(Some(moved));
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
                    description="Tick some items, then pick up any ticked one: they all go together, keeping their order, and land as one block. The held item becomes a pile with a count on it, the other ticked items leave their places, and the rest slide to close up around the single gap the block would land in — nothing in the page reflows, it is all transforms. The keyboard does the same: the pile moves a slot at a time."
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
                    title="Sideways block drag"
                    description="The same block on a horizontal list: tick some chips, then pick any ticked one up. It is the pile that follows the pointer, the other ticked chips leave their places and the rest close up around the one gap it would land in. Everything is a transform, so nothing in the page reflows while it is held."
                    code=SIDEWAYS
                >
                    <div class="w-full overflow-x-auto pt-3 pr-3 pb-3">
                        <Sortable
                            label="Crates"
                            orientation=Orientation::Horizontal
                            selected=crates_selected
                            on_move=crates_move
                        >
                            <For each=move || crates.get() key=|item| item.clone() let:item>
                                <SortableItem value=item.clone() label=item.clone()>
                                    <Checkbox on_checked_change={
                                        let item = item.clone();
                                        move |on| {
                                            crates_selected
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
                    title="Plain"
                    description="variant=SortableItemVariant::Plain drops the card: no border, shadow or background until the row is hovered, selected or held. It is for a list that lives among menu entries, like pinned items in a sidebar, where the card look has to be switched off by hand otherwise."
                    code=PLAIN
                >
                    <div class="w-56 rounded-lg border bg-sidebar p-2 text-sidebar-foreground">
                        <Sortable label="Pinned" class="gap-0.5" on_move=pinned_move>
                            <For each=move || pinned.get() key=|item| item.clone() let:item>
                                <SortableItem
                                    value=item.clone()
                                    label=item.clone()
                                    variant=SortableItemVariant::Plain
                                    class="group/row"
                                >
                                    <SortableHandle class="size-5 opacity-0 group-hover/row:opacity-100 focus-visible:opacity-100" />
                                    <span>{item.clone()}</span>
                                </SortableItem>
                            </For>
                        </Sortable>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Scrolling"
                    description="Hold an item near the top or bottom of the list and it scrolls. It is the list's own scroller that moves, never the page behind it, so the same list works inside a dialog whose body is locked. Give the list a height and overflow-y-auto; the nearest ancestor that actually scrolls is the one used. The items here can be ticked too, which is the way to try a block over a longer list: pick up any ticked item and the rest fold into the pile under it."
                    code=SCROLLING
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <Sortable
                            label="Queue"
                            class="max-h-56 overflow-y-auto pr-2"
                            selected=queue_selected
                            on_move=queue_move
                        >
                            <For each=move || queue.get() key=|item| item.clone() let:item>
                                <SortableItem value=item.clone() label=item.clone()>
                                    <SortableHandle />
                                    <Checkbox on_checked_change={
                                        let item = item.clone();
                                        move |on| {
                                            queue_selected
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
                    title="Windowed"
                    description="Five thousand rows, and about twenty of them in the page. SortableWindow keeps only the rows in view, because every row is the same height and a row's place is its index times that: the drop, the slide of every row it passes and the window itself are arithmetic, so a drag costs the rows on screen and not the list. It owns its scroller, so give it a height. The held row stays in the page wherever it is carried, the list scrolls at its edges while one is held, and the keyboard is the same as above — arrows, Home and End move focus through the list when nothing is held, scrolling rows in as they go. Turn on Select, tick rows (Shift-click, or Shift and Space, for a range) and drag any ticked row: the whole selection moves as one block, including the rows that are not in the page. On pick-up the block collapses into one stack: the row you hold wears a count and a pile, and follows the pointer; the other selected rows leave the list, which closes up around them, so the list is as many rows long as it has unselected ones plus one; a single marker shows the one slot the block will land in. On the drop the stack opens into the selected rows, contiguous, at that slot, and Escape puts everything back. The list is scrolled by as many rows as left it above the held one, so what you were looking at stays put. Where the stack lands is decided by the rows it has visibly passed. Every handle is a tab stop; the arrows are how you reach rows that are not in the page. Fixed row height, vertical."
                    code=WINDOW
                >
                    <div class="flex w-full max-w-sm flex-col gap-3">
                        <div class="flex items-center gap-3 text-sm">
                            <Switch checked=selecting attr:aria-label="Select" />
                            <span>"Select"</span>
                            <span
                                class="ml-auto text-xs text-muted-foreground"
                                data-testid="sortable-window-count"
                            >
                                {move || format!("{} selected", range.len())}
                            </span>
                        </div>
                        <SortableWindow
                            label="Tracks"
                            class="h-96"
                            count=move || tracks.with(Vec::len)
                            row_height=44.0
                            key=move |i: usize| {
                                tracks
                                    .with(|tracks| {
                                        tracks.get(i).map(|n| n.to_string()).unwrap_or_default()
                                    })
                            }
                            item_label=move |i: usize| {
                                tracks
                                    .with(|tracks| {
                                        tracks
                                            .get(i)
                                            .map(|n| format!("Track {n:04}"))
                                            .unwrap_or_default()
                                    })
                            }
                            selected=range.selected
                            row=move |i: usize| {
                                let n = tracks.with(|tracks| tracks.get(i).copied().unwrap_or(0));
                                view! {
                                    {move || {
                                        selecting
                                            .get()
                                            .then(|| {
                                                view! {
                                                    <Checkbox
                                                        class="size-5 rounded-none"
                                                        checked=range.checked(i)
                                                        on_click=range.on_click(i)
                                                        attr:aria-label=format!("Select row {}", i + 1)
                                                    />
                                                }
                                            })
                                    }}
                                    <SortableWindowHandle />
                                    <span class="tabular-nums">{format!("Track {n:04}")}</span>
                                    <span class="ml-auto text-xs tabular-nums text-muted-foreground">
                                        {format!("#{}", i + 1)}
                                    </span>
                                }
                                    .into_any()
                            }
                            on_move=window_move
                        />

                        <p class="text-xs text-muted-foreground" data-testid="sortable-window-last">
                            {move || match window_last.get() {
                                None => "No move yet.".to_string(),
                                Some(moved) if moved.is_block() => {
                                    format!("Moved {} items to {}", moved.count(), moved.to)
                                }
                                Some(moved) => {
                                    format!(
                                        "Moved {} from {} to {}",
                                        moved.key,
                                        moved.from,
                                        moved.to,
                                    )
                                }
                            }}
                        </p>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
