//! Toast blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        empty::{Empty, EmptyDescription, EmptyHeader, EmptyTitle},
        item::{
            Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemTitle, ItemVariant,
        },
        toast::{ToastPosition, use_toast},
    },
    icons::x::X,
};

#[derive(Clone, PartialEq)]
struct Doc {
    name: &'static str,
    detail: &'static str,
}

#[rustfmt::skip]
const DOCS: &[Doc] = &[
    Doc { name: "Q3 forecast.numbers", detail: "Edited 2 days ago · 1.2 MB" },
    Doc { name: "Offsite agenda.md",   detail: "Edited last week · 4 KB" },
    Doc { name: "Logo marks.zip",      detail: "Edited in March · 18 MB" },
];

const UNDO: &str = r##"// The row goes as soon as it is pressed — an undo is not a confirmation,
// and a list that waits to see whether you meant it is a dialog wearing a
// toast's clothes. What makes it safe is that the toast carries the way back.
let rows = RwSignal::new(DOCS.to_vec());

let delete = move |index: usize| {
    let removed = rows.with_untracked(|rows| rows[index].clone());

    // This handler is running inside the row it is about to remove, so the
    // toast is raised from a view that is already gone. A toast's signals
    // belong to the provider, which is what makes that legal.
    rows.update(|rows| { rows.remove(index); });

    toast
        .toast(format!("Deleted {}", removed.name))
        .description("Undo puts it back where it was.")
        .action("Undo", move || {
            // Back at its own index, not on the end. A list that reappears
            // in a different order has not undone anything — and the index
            // is clamped, because two deletions can be undone out of order.
            rows.update(|rows| {
                let at = index.min(rows.len());
                rows.insert(at, removed.clone());
            })
        })
        .position(ToastPosition::BottomRight)
        .show();
};

view! {
    <ItemGroup>
        <Item variant=ItemVariant::Outline>
            <ItemContent>
                <ItemTitle>{doc.name}</ItemTitle>
                <ItemDescription>{doc.detail}</ItemDescription>
            </ItemContent>
            <ItemActions>
                <Button
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::Icon
                    on:click=move |_| delete(index)
                >
                    <X />
                    <span class="sr-only">{format!("Delete {}", doc.name)}</span>
                </Button>
            </ItemActions>
        </Item>
    </ItemGroup>
}"##;

pub const BLOCKS: &[Block] = &[Block {
    slug: "a-delete-you-can-take-back",
    title: "A delete you can take back",
    uses: &["button", "empty", "item", "toast"],
    description: "The row goes the moment it is pressed. That is the whole point of the pattern: \
                  an undo is not a confirmation, and a list that stops to ask whether you meant \
                  it is a dialog wearing a toast's clothes — slower for the ninety-nine deletions \
                  that were meant and no safer for the one that was not. What makes it safe is \
                  that the toast carries the way back, which is what `.action(label, on_press)` \
                  is: a label and a closure, and the closure has the deleted row and the index it \
                  came from in hand. It puts the row back *there* rather than on the end, because \
                  a list that reappears in a different order has not undone anything — and the \
                  index is clamped, since deleting two rows gives two toasts and they can be \
                  undone in either order. The toast dismisses itself once the action runs: one \
                  that stayed up still offering to undo would undo twice. Let it time out instead \
                  and the deletion is simply final, which is the other half of the bargain — the \
                  five seconds are the confirmation dialog, served after the fact and only to the \
                  person who wanted one. Note what the handler does first: it removes the row it \
                  is running inside, so the toast is raised from a view that is already gone by \
                  the time it appears. That is legal because a toast's signals are minted under \
                  the provider rather than under whoever called `show()` — before that they were \
                  disposed in the same breath they were made, which was a panic rather than a \
                  missing toast.",
    code: UNDO,
    view: || view! { <DocList /> }.into_any(),
}];

#[component]
fn DocList() -> impl IntoView {
    let toast = use_toast();
    let rows = RwSignal::new(DOCS.to_vec());

    let delete = move |index: usize| {
        let removed = rows.with_untracked(|rows| rows[index].clone());

        // The row goes first, and this handler is running inside it — the toast is raised from a
        // view that no longer exists by the time it appears. Its signals belong to the provider,
        // which is what makes that legal.
        rows.update(|rows| {
            rows.remove(index);
        });

        toast
            .toast(format!("Deleted {}", removed.name))
            .description("Undo puts it back where it was.")
            .action("Undo", move || {
                rows.update(|rows| {
                    let at = index.min(rows.len());
                    rows.insert(at, removed.clone());
                })
            })
            .position(ToastPosition::BottomRight)
            .show();
    };

    view! {
        <div class="flex w-full max-w-lg flex-col gap-3">
            <ItemGroup class="gap-2">
                {move || {
                    rows.get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, doc)| {
                            let name = doc.name;

                            view! {
                                <Item variant=ItemVariant::Outline>
                                    <ItemContent>
                                        <ItemTitle>{doc.name}</ItemTitle>
                                        <ItemDescription>{doc.detail}</ItemDescription>
                                    </ItemContent>
                                    <ItemActions>
                                        <Button
                                            variant=ButtonVariant::Ghost
                                            size=ButtonSize::Icon
                                            on:click=move |_| delete(index)
                                        >
                                            <X />
                                            <span class="sr-only">{format!("Delete {name}")}</span>
                                        </Button>
                                    </ItemActions>
                                </Item>
                            }
                        })
                        .collect_view()
                }}
            </ItemGroup>

            // Only here because a demo has to be repeatable: three deletions and a page with
            // nothing left to press is not much of an example.
            <Show when=move || rows.with(|rows| rows.is_empty())>
                <Empty class="py-8">
                    <EmptyHeader>
                        <EmptyTitle>"Nothing left"</EmptyTitle>
                        <EmptyDescription>
                            "The undo only lasts as long as the toast does."
                        </EmptyDescription>
                    </EmptyHeader>
                    <Button
                        variant=ButtonVariant::Outline
                        size=ButtonSize::Sm
                        on:click=move |_| rows.set(DOCS.to_vec())
                    >
                        "Put them back"
                    </Button>
                </Empty>
            </Show>
        </div>
    }
}
