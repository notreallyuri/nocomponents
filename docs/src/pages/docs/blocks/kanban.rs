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
        dialog::{Dialog, DialogDescription, DialogHeader, DialogPortal, DialogTitle},
        input::Input,
        kanban::{KanbanBoard, KanbanCard, KanbanColumn, KanbanColumnHeader, KanbanSlot},
        tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    },
    icons::{check::Check, message::MessageSquare, paperclip::Paperclip},
    primitives::kanban::KanbanMove,
    utils::types::AnchorRender,
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

const DETAIL: &str = r##"// Everything below the title is markup the caller arranges — a colour bar,
// a count, a label. The component owes it two things only: `children`, and
// not starting a drag when a control inside the card is pressed.
#[derive(Clone, Copy, PartialEq)]
enum Priority { Low, Medium, High }

// The face: a bar you can hover, a title, and the counts as icons rather
// than as three more lines of text.
<KanbanCard
    id=ticket.id
    // Not `on:click` — a card following the pointer is `pointer-events: none`,
    // so the click never reaches it. `on_select` fires only when the press
    // never travelled, which is the one thing the caller cannot work out.
    on_select=Callback::new(move |_| { open.set(Some(ticket)); visible.set(true); })
>
    <div class="flex flex-col gap-2">
        <Tooltip>
            <TooltipTrigger render=Callback::new(move |anchor: AnchorRender| view! {
                <div node_ref=anchor.node_ref class=priority.bar() />
            }.into_any()) />
            <TooltipContent>{priority.label()}</TooltipContent>
        </Tooltip>

        <span>{ticket.title}</span>

        <div class="flex items-center gap-3 text-muted-foreground">
            <MessageSquare /> {ticket.comments}
            <Paperclip />     {ticket.attachments}
            <Check />         {format!("{}/{}", ticket.done, ticket.steps)}
        </div>
    </div>
</KanbanCard>

// And the detail is a dialog over the board, opened by the same signal.
<Dialog open=visible>
    <DialogPortal>
        <DialogHeader>
            <DialogTitle>{…}</DialogTitle>
            <DialogDescription>{…}</DialogDescription>
        </DialogHeader>
        {…}
    </DialogPortal>
</Dialog>"##;

#[derive(Clone, Copy, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    /// A literal per arm: Tailwind reads the source for class names and cannot follow one that is
    /// assembled at runtime.
    fn bar(self) -> &'static str {
        match self {
            Priority::Low => "h-1.5 w-10 rounded-full bg-emerald-500",
            Priority::Medium => "h-1.5 w-10 rounded-full bg-amber-500",
            Priority::High => "h-1.5 w-10 rounded-full bg-rose-500",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Priority::Low => "Low priority",
            Priority::Medium => "Medium priority",
            Priority::High => "High priority",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Ticket {
    id: &'static str,
    column: &'static str,
    title: &'static str,
    priority: Priority,
    comments: usize,
    attachments: usize,
    done: usize,
    steps: usize,
    detail: &'static str,
}

#[rustfmt::skip]
const TICKETS: &[Ticket] = &[
    Ticket { id: "nc-41", column: "Backlog", title: "Split globals.css", priority: Priority::Medium,
             comments: 3, attachments: 1, done: 1, steps: 4,
             detail: "Four palettes and fourteen accents is most of a consumer's stylesheet, for tokens they mostly will not use. Separate the base tokens so a project takes one and opts into the rest." },
    Ticket { id: "nc-52", column: "Backlog", title: "Re-encode the avatar", priority: Priority::Low,
             comments: 0, attachments: 2, done: 0, steps: 2,
             detail: "docs/assets/avatar.jpg is a 6.9MB PNG with a .jpg name, and it is include_bytes!d — so all of it lands in the wasm binary." },
    Ticket { id: "nc-60", column: "In progress", title: "Board auto-scroll", priority: Priority::High,
             comments: 5, attachments: 0, done: 2, steps: 6,
             detail: "A board wider than the viewport can only be crossed by letting go and scrolling. Needs the board's rect and the live pointer, a rate curve, and a stop on release." },
    Ticket { id: "nc-33", column: "Done", title: "Tree", priority: Priority::Medium,
             comments: 2, attachments: 0, done: 5, steps: 5,
             detail: "role=tree, one tab stop, aria-level per depth, and aria-expanded only on the items that have children." },
];

fn detail_board() -> AnyView {
    let tickets = RwSignal::new(TICKETS.to_vec());
    // Two signals rather than one derived from the other: the dialog owns whether it is showing,
    // and this owns what it is showing. Deriving `visible` from `open` and clearing `open` when it
    // goes false is the obvious shape and it hangs the page — `set` notifies whether or not the
    // value changed, so the two effects write to each other for ever. Nothing needs to read the
    // ticket while the dialog is closed, so nothing has to clear it.
    let open = RwSignal::new(None::<Ticket>);
    let visible = RwSignal::new(false);

    let apply = move |moved: KanbanMove| {
        tickets.update(|tickets| {
            if let Some(ticket) = tickets.iter_mut().find(|t| t.id == moved.card) {
                // The columns are a fixed set here, so a move is one field.
                ticket.column = match moved.to.as_str() {
                    "Backlog" => "Backlog",
                    "In progress" => "In progress",
                    _ => "Done",
                };
            }
        });
    };

    view! {
        <div class="flex w-full flex-col gap-3">
            <KanbanBoard on_move=Callback::new(apply)>
                {move || {
                    ["Backlog", "In progress", "Done"]
                        .into_iter()
                        .map(|column| {
                            let here: Vec<Ticket> = tickets
                                .get()
                                .into_iter()
                                .filter(|t| t.column == column)
                                .collect();
                            let count = here.len();

                            view! {
                                <KanbanColumn id=column.to_string()>
                                    <KanbanColumnHeader>
                                        <span>{column}</span>
                                        <Badge variant=BadgeVariant::Secondary>
                                            {count.to_string()}
                                        </Badge>
                                    </KanbanColumnHeader>

                                    <KanbanSlot index=0 />
                                    {here
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, ticket)| {
                                            view! {
                                                <KanbanCard
                                                    id=ticket.id.to_string()
                                                    on_select=Callback::new(move |_| {
                                                        open.set(Some(ticket));
                                                        visible.set(true);
                                                    })
                                                >
                                                    <div class="flex flex-col gap-2">
                                                        <Tooltip>
                                                            <TooltipTrigger render=Callback::new(move |
                                                                anchor: AnchorRender|
                                                            {
                                                                view! {
                                                                    <div
                                                                        node_ref=anchor.node_ref
                                                                        class=ticket.priority.bar()
                                                                    />
                                                                }
                                                                    .into_any()
                                                            }) />
                                                            <TooltipContent>
                                                                {ticket.priority.label()}
                                                            </TooltipContent>
                                                        </Tooltip>

                                                        <span class="leading-snug">{ticket.title}</span>

                                                        <div class="flex items-center gap-3 text-xs text-muted-foreground">
                                                            <span class="flex items-center gap-1">
                                                                <MessageSquare class="size-3.5" />
                                                                {ticket.comments.to_string()}
                                                            </span>
                                                            <span class="flex items-center gap-1">
                                                                <Paperclip class="size-3.5" />
                                                                {ticket.attachments.to_string()}
                                                            </span>
                                                            <span class="flex items-center gap-1">
                                                                <Check class="size-3.5" />
                                                                {format!("{}/{}", ticket.done, ticket.steps)}
                                                            </span>
                                                        </div>
                                                    </div>
                                                </KanbanCard>
                                                <KanbanSlot index=i + 1 />
                                            }
                                        })
                                        .collect_view()}
                                </KanbanColumn>
                            }
                        })
                        .collect_view()
                }}
            </KanbanBoard>

            <p class="text-xs text-muted-foreground">
                "Click a card to open it. Drag one to move it."
            </p>

            <Dialog open=visible>
                <DialogPortal class="sm:max-w-lg">
                    <DialogHeader>
                        <DialogTitle>
                            {move || open.get().map(|t| t.title).unwrap_or_default()}
                        </DialogTitle>
                        <DialogDescription>
                            {move || {
                                open.get()
                                    .map(|t| format!("{} · {}", t.id, t.column))
                                    .unwrap_or_default()
                            }}
                        </DialogDescription>
                    </DialogHeader>

                    {move || {
                        open.get()
                            .map(|ticket| {
                                view! {
                                    <div class="flex flex-col gap-4">
                                        <div class="flex items-center gap-2">
                                            <div class=ticket.priority.bar() />
                                            <span class="text-xs text-muted-foreground">
                                                {ticket.priority.label()}
                                            </span>
                                        </div>

                                        <p class="text-sm leading-relaxed text-muted-foreground">
                                            {ticket.detail}
                                        </p>

                                        <div class="flex items-center gap-4 border-t pt-3 text-xs text-muted-foreground">
                                            <span class="flex items-center gap-1.5">
                                                <MessageSquare class="size-3.5" />
                                                {format!("{} comments", ticket.comments)}
                                            </span>
                                            <span class="flex items-center gap-1.5">
                                                <Paperclip class="size-3.5" />
                                                {format!("{} attachments", ticket.attachments)}
                                            </span>
                                            <span class="flex items-center gap-1.5">
                                                <Check class="size-3.5" />
                                                {format!("{} of {} done", ticket.done, ticket.steps)}
                                            </span>
                                        </div>
                                    </div>
                                }
                            })
                    }}
                </DialogPortal>
            </Dialog>
        </div>
    }
    .into_any()
}

pub const BLOCKS: &[Block] = &[
    Block {
        slug: "a-board-you-can-build",
        title: "A board you can build",
        uses: &[
            "badge",
            "button",
            "context-menu",
            "dialog",
            "input",
            "kanban",
            "tooltip",
        ],
        description: "Columns you add, cards you add, and two ways to move one: drag it, or right-click \
                  it and pick a column. Both end in the same closure — `on_move` hands back a \
                  `KanbanMove`, and the context menu builds one by hand — so the two cannot come to \
                  disagree about what moving means. The card is the context menu's trigger, which \
                  works because the drag guards on the primary button: a right-click was never \
                  going to start one. Cards are keyed by id rather than by title, since two cards \
                  called the same thing would be one card that teleports.",
        code: BOARD,
        view: board,
    },
    Block {
        slug: "a-card-with-more-on-it-than-fits",
        title: "A card with more on it than fits",
        uses: &[
            "badge",
            "button",
            "context-menu",
            "dialog",
            "input",
            "kanban",
            "tooltip",
        ],
        description: "Trello's answer to a card that has a description, a checklist, five comments and \
                  two attachments: put a bar, a title and three counts on the face, and everything \
                  else behind a dialog. Every part of that is the caller's markup — the component \
                  owes it `children` and one guarantee, which is that pressing a control inside a \
                  card does not start a drag. What the component *does* owe is `on_select`: a card \
                  that is following the pointer is `pointer-events: none`, so a click never reaches \
                  it, and only the gesture knows whether the pointer travelled. The priority bar is \
                  a `Tooltip` trigger rendered `as_child`, so the tooltip hangs off the bar rather \
                  than off a wrapper around it.",
        code: DETAIL,
        view: detail_board,
    },
];
