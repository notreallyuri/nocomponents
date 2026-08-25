//! Kanban blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        badge::{Badge, BadgeVariant},
        button::{Button, ButtonSize, ButtonVariant},
        context_menu::{
            ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuPortal,
            ContextMenuSeparator, ContextMenuSub, ContextMenuSubContent, ContextMenuSubTrigger,
            ContextMenuTrigger,
        },
        input::Input,
        kanban::{KanbanBoard, KanbanCard, KanbanColumn, KanbanColumnHeader, KanbanSlot},
    },
    primitives::kanban::KanbanMove,
};

const BOARD: &str = r##"// One list, and every gesture is a function over it. The board reports a
// move and never touches the list; the context menu calls the same function
// the drag does, so "drop it on Doing" and "Move to → Doing" cannot drift.
#[derive(Clone, PartialEq)]
struct Column {
    name: String,
    cards: Vec<String>,
}

let board = RwSignal::new(vec![ /* … */ ]);

// The one mutation. `on_move` is a drag; the context menu builds the same
// value and hands it to the same closure.
let apply = move |moved: KanbanMove| {
    board.update(|board| {
        for column in board.iter_mut() {
            column.cards.retain(|card| *card != moved.card);
        }
        if let Some(column) = board.iter_mut().find(|c| c.name == moved.to) {
            column.cards.insert(moved.index.min(column.cards.len()), moved.card);
        }
    });
};

<KanbanBoard on_move=Callback::new(apply)>
    <For each=move || board.get() key=|c| c.name.clone() let:column>
        <KanbanColumn id=column.name.clone()>
            <KanbanColumnHeader>
                <span>{column.name.clone()}</span>
                <Badge variant=BadgeVariant::Secondary>{column.cards.len()}</Badge>
            </KanbanColumnHeader>

            <KanbanSlot index=0 />
            {column.cards.iter().enumerate().map(|(i, card)| view! {
                // The card is the trigger. A right-click never starts a drag —
                // the gesture guards on the primary button — so the two live on
                // the same element without fighting.
                <ContextMenu>
                    <ContextMenuTrigger>
                        <KanbanCard id=card.clone()>{card.clone()}</KanbanCard>
                    </ContextMenuTrigger>
                    <ContextMenuPortal>
                        <ContextMenuContent>
                            <ContextMenuSub>
                                <ContextMenuSubTrigger>"Move to"</ContextMenuSubTrigger>
                                <ContextMenuSubContent>
                                    // Every column but the one it is in.
                                    {…}
                                </ContextMenuSubContent>
                            </ContextMenuSub>
                            <ContextMenuSeparator />
                            <ContextMenuItem on_click=…>"Delete"</ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenuPortal>
                </ContextMenu>
                <KanbanSlot index=i + 1 />
            }).collect_view()}

            // Adding a card is the same list, one push.
            <Input placeholder="Add a card…" on:keydown=… />
        </KanbanColumn>
    </For>

    // And a column is an empty Vec on the end.
    <Button variant=ButtonVariant::Outline on:click=add_column>"Add column"</Button>
</KanbanBoard>"##;

#[derive(Clone, PartialEq)]
struct Column {
    name: String,
    cards: Vec<String>,
}

fn seed() -> Vec<Column> {
    vec![
        Column {
            name: "Backlog".into(),
            cards: vec![
                "Split globals.css".into(),
                "Re-encode the avatar".into(),
                "Publish the CLI".into(),
            ],
        },
        Column {
            name: "In progress".into(),
            cards: vec!["Kanban block".into()],
        },
        Column {
            name: "Done".into(),
            cards: vec!["Tree".into(), "Dropzone".into()],
        },
    ]
}

fn board() -> AnyView {
    let board = RwSignal::new(seed());
    let next = RwSignal::new(0usize);

    // The one mutation every gesture goes through. A drag calls it with what the board reported;
    // the context menu builds the same value by hand. Two ways to ask, one thing that happens.
    let apply = move |moved: KanbanMove| {
        board.update(|board| {
            for column in board.iter_mut() {
                column.cards.retain(|card| *card != moved.card);
            }
            if let Some(column) = board.iter_mut().find(|column| column.name == moved.to) {
                let at = moved.index.min(column.cards.len());
                column.cards.insert(at, moved.card.clone());
            }
        });
    };

    let delete = move |card: String| {
        board.update(|board| {
            for column in board.iter_mut() {
                column.cards.retain(|existing| *existing != card);
            }
        });
    };

    let add_card = move |column_name: String, title: String| {
        let title = title.trim().to_string();
        if title.is_empty() {
            return;
        }
        // Ids are what the board moves cards by, so two cards called "Fix it" would be one card
        // that teleports. The counter is the whole fix.
        next.update(|n| *n += 1);
        let id = format!("{title} #{}", next.get_untracked());
        board.update(|board| {
            if let Some(column) = board.iter_mut().find(|column| column.name == column_name) {
                column.cards.push(id);
            }
        });
    };

    let add_column = move |name: String| {
        let name = name.trim().to_string();
        if name.is_empty() || board.with_untracked(|b| b.iter().any(|c| c.name == name)) {
            return;
        }
        board.update(|board| {
            board.push(Column {
                name,
                cards: Vec::new(),
            })
        });
    };

    let new_column = RwSignal::new(String::new());

    view! {
        <div class="flex w-full flex-col gap-3">
            <KanbanBoard on_move=Callback::new(apply)>
                {move || {
                    let columns = board.get();
                    let names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
                    columns
                        .into_iter()
                        .map(|column| {
                            let names = names.clone();
                            let here = column.name.clone();
                            let heading = here.clone();
                            let count = column.cards.len();
                            let draft = RwSignal::new(String::new());
                            let column_for_add = here.clone();

                            view! {
                                <KanbanColumn id=here.clone()>
                                    <KanbanColumnHeader>
                                        <span>{heading}</span>
                                        <Badge variant=BadgeVariant::Secondary>
                                            {count.to_string()}
                                        </Badge>
                                    </KanbanColumnHeader>

                                    <KanbanSlot index=0 />
                                    {column
                                        .cards
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, card)| {
                                            // `ContextMenu` re-renders its children, so everything
                                            // inside has to be `Fn` — a moved `String` would make
                                            // the closure `FnOnce`. Stored values are `Copy`.
                                            let others = StoredValue::new(
                                                names
                                                    .iter()
                                                    .filter(|name| **name != here)
                                                    .cloned()
                                                    .collect::<Vec<String>>(),
                                            );
                                            let card = StoredValue::new(card);
                                            let column = StoredValue::new(here.clone());

                                            view! {
                                                <ContextMenu>
                                                    <ContextMenuTrigger>
                                                        <KanbanCard id=card.get_value()>
                                                            {move || card.get_value()}
                                                        </KanbanCard>
                                                    </ContextMenuTrigger>
                                                    <ContextMenuPortal>
                                                        <ContextMenuContent class="w-44">
                                                            <ContextMenuSub>
                                                                <ContextMenuSubTrigger>"Move to"</ContextMenuSubTrigger>
                                                                <ContextMenuSubContent>
                                                                    {move || {
                                                                        others
                                                                            .get_value()
                                                                            .into_iter()
                                                                            .map(|target| {
                                                                                let to = target.clone();
                                                                                view! {
                                                                                    <ContextMenuItem on_click=Callback::new(move |_| {
                                                                                        apply(KanbanMove {
                                                                                            card: card.get_value(),
                                                                                            from: column.get_value(),
                                                                                            to: to.clone(),
                                                                                            // Past the end: `apply` clamps, and
                                                                                            // "the bottom" is what a menu move means.
                                                                                            index: usize::MAX,
                                                                                        })
                                                                                    })>{target}</ContextMenuItem>
                                                                                }
                                                                            })
                                                                            .collect_view()
                                                                    }}
                                                                </ContextMenuSubContent>
                                                            </ContextMenuSub>
                                                            <ContextMenuSeparator />
                                                            <ContextMenuItem
                                                                class="text-destructive"
                                                                on_click=Callback::new(move |_| {
                                                                    delete(card.get_value())
                                                                })
                                                            >
                                                                "Delete"
                                                            </ContextMenuItem>
                                                        </ContextMenuContent>
                                                    </ContextMenuPortal>
                                                </ContextMenu>
                                                <KanbanSlot index=i + 1 />
                                            }
                                        })
                                        .collect_view()}

                                    <Input
                                        model=draft
                                        attr:placeholder="Add a card…"
                                        class="h-8 bg-background text-sm"
                                        on:keydown=move |e| {
                                            if e.key() == "Enter" {
                                                add_card(column_for_add.clone(), draft.get_untracked());
                                                draft.set(String::new());
                                            }
                                        }
                                    />
                                </KanbanColumn>
                            }
                        })
                        .collect_view()
                }}

                <div class="flex w-56 shrink-0 flex-col gap-2 rounded-lg border border-dashed p-2">
                    <Input
                        model=new_column
                        attr:placeholder="New column…"
                        class="h-8 text-sm"
                        on:keydown=move |e| {
                            if e.key() == "Enter" {
                                add_column(new_column.get_untracked());
                                new_column.set(String::new());
                            }
                        }
                    />
                    <Button
                        variant=ButtonVariant::Outline
                        size=ButtonSize::Sm
                        on:click=move |_| {
                            add_column(new_column.get_untracked());
                            new_column.set(String::new());
                        }
                    >
                        "Add column"
                    </Button>
                </div>
            </KanbanBoard>

            <p class="text-xs text-muted-foreground">
                "Drag a card, or right-click one. Enter adds."
            </p>
        </div>
    }
    .into_any()
}

pub const BLOCKS: &[Block] = &[Block {
    title: "A board you can build",
    description: "Columns you add, cards you add, and two ways to move one: drag it, or right-click \
                  it and pick a column. Both end in the same closure — `on_move` hands back a \
                  `KanbanMove`, and the context menu builds one by hand — so the two cannot come to \
                  disagree about what moving means. The card is the context menu's trigger, which \
                  works because the drag guards on the primary button: a right-click was never \
                  going to start one. Cards are keyed by id rather than by title, since two cards \
                  called the same thing would be one card that teleports.",
    code: BOARD,
    view: board,
}];
