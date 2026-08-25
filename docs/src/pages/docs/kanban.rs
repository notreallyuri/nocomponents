//! The kanban page: a board that reports moves, and a caller that applies them.

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
        badge::{Badge, BadgeVariant},
        kanban::{KanbanBoard, KanbanCard, KanbanColumn, KanbanColumnHeader, KanbanSlot},
    },
    primitives::kanban::KanbanMove,
};

const DEFAULT: &str = r#"// The board never touches the list — it says where a card was let go and
// the caller does the rest. One `apply` is the whole integration.
let columns = RwSignal::new(vec![
    ("todo".to_string(), vec!["Write the docs".to_string()]),
    ("doing".to_string(), vec!["Ship the tree".to_string()]),
]);

let apply = move |moved: KanbanMove| {
    columns.update(|columns| {
        for (_, cards) in columns.iter_mut() {
            cards.retain(|card| *card != moved.card);
        }
        if let Some((_, cards)) = columns.iter_mut().find(|(id, _)| *id == moved.to) {
            cards.insert(moved.index.min(cards.len()), moved.card);
        }
    });
};

<KanbanBoard on_move=Callback::new(apply)>
    <For each=move || columns.get() key=|(id, _)| id.clone() let:column>
        <KanbanColumn id=column.0.clone()>
            <KanbanColumnHeader>{column.0.clone()}</KanbanColumnHeader>
            <KanbanSlot index=0 />
            {column.1.iter().enumerate().map(|(i, card)| view! {
                <KanbanCard id=card.clone()>{card.clone()}</KanbanCard>
                <KanbanSlot index=i + 1 />
            }).collect_view()}
        </KanbanColumn>
    </For>
</KanbanBoard>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "KanbanBoard",
        description: "The board. It owns the gesture and nothing else — not the columns, not the cards, not their order.",
        props: &[
            Prop {
                name: "on_move",
                ty: "Callback<KanbanMove>",
                default: "",
                description: "A card was let go somewhere new: which card, the column it left, the column and index it landed at. Never fired for a card put back where it was.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the board's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The columns.",
            },
        ],
    },
    ApiEntry {
        name: "KanbanColumn",
        description: "A drop target. Wears data-over while a card would land in it.",
        props: &[
            Prop {
                name: "id",
                ty: "String",
                default: "",
                description: "What a move reports as from and to. Unique across the board.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the column's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "A header, the cards, and a KanbanSlot in each gap.",
            },
        ],
    },
    ApiEntry {
        name: "KanbanCard",
        description: "A drag source. Follows the pointer while it is in hand, without leaving the list — so the gap it came from stays open and no column reflows underneath.",
        props: &[
            Prop {
                name: "id",
                ty: "String",
                default: "",
                description: "What a move reports as card. Unique across the board.",
            },
            Prop {
                name: "on_select",
                ty: "Option<Callback<()>>",
                default: "None",
                description: "The card was pressed and let go without travelling. Use this rather than your own on:click — a card that is following the pointer has pointer-events: none, so a click never reaches it. Only the gesture knows whether the pointer moved.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Refuses to be picked up. A press on a button, link or input inside a card is never a drag either.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the card's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The card's content.",
            },
        ],
    },
    ApiEntry {
        name: "KanbanSlot",
        description: "The line a card would land on. Rendered by the caller between its cards, because only the caller knows where between is.",
        props: &[
            Prop {
                name: "index",
                ty: "usize",
                default: "",
                description: "How many cards down this gap sits. 0 is above the first card.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the line's classes.",
            },
        ],
    },
];

type Columns = Vec<(String, Vec<String>)>;

#[component]
pub fn Page() -> impl IntoView {
    let columns: RwSignal<Columns> = RwSignal::new(vec![
        (
            "Todo".to_string(),
            vec![
                "Write the kanban docs".to_string(),
                "Re-encode the avatar".to_string(),
            ],
        ),
        (
            "Doing".to_string(),
            vec!["Ship the tree".to_string(), "Fix the dropzone".to_string()],
        ),
        ("Done".to_string(), vec!["Apache-2.0".to_string()]),
    ]);
    let last = RwSignal::new(None::<KanbanMove>);
    let clicked = RwSignal::new(None::<String>);

    let apply = move |moved: KanbanMove| {
        columns.update(|columns| {
            for (_, cards) in columns.iter_mut() {
                cards.retain(|card| *card != moved.card);
            }
            if let Some((_, cards)) = columns.iter_mut().find(|(id, _)| *id == moved.to) {
                cards.insert(moved.index.min(cards.len()), moved.card.clone());
            }
        });
        last.set(Some(moved));
    };

    view! {
        <DocLayout
            title="Kanban"
            description="Columns of cards, and a card that can be dragged between them. The board reports the move; the list stays yours."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Press a card and move it: it lifts and follows the pointer, and the line shows where it would land. It follows by transform rather than by leaving the list, so the gap it came from stays open and no column reflows under the gesture. Letting go where it started reports nothing, and a press that never travels is a click rather than a drag — which is what on_select is for."
                    code=DEFAULT
                >
                    <div class="flex w-full flex-col gap-3">
                        <KanbanBoard on_move=Callback::new(
                            apply,
                        )>
                            {move || {
                                columns
                                    .get()
                                    .into_iter()
                                    .map(|(name, cards)| {
                                        let count = cards.len();
                                        view! {
                                            <KanbanColumn id=name.clone()>
                                                <KanbanColumnHeader>
                                                    <span>{name.clone()}</span>
                                                    <Badge variant=BadgeVariant::Secondary>
                                                        {count.to_string()}
                                                    </Badge>
                                                </KanbanColumnHeader>
                                                <KanbanSlot index=0 />
                                                {cards
                                                    .into_iter()
                                                    .enumerate()
                                                    .map(|(i, card)| {
                                                        let name = card.clone();
                                                        view! {
                                                            <KanbanCard
                                                                id=card.clone()
                                                                on_select=Callback::new(move |_| {
                                                                    clicked.set(Some(name.clone()))
                                                                })
                                                            >
                                                                {card.clone()}
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
                            {move || match clicked.get() {
                                Some(card) => format!("Clicked {card} · "),
                                None => String::new(),
                            }}
                            {move || match last.get() {
                                None => "No move yet.".to_string(),
                                Some(moved) => {
                                    format!(
                                        "{} — {} → {} at {}",
                                        moved.card,
                                        moved.from,
                                        moved.to,
                                        moved.index,
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
