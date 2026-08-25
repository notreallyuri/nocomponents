use crate::{
    cn,
    primitives::kanban::{
        KanbanBoardRoot, KanbanCardRoot, KanbanColumnRoot, KanbanMove, KanbanSlotRoot,
    },
};
use leptos::prelude::*;

#[component]
pub fn KanbanBoard(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(into)] on_move: Callback<KanbanMove>,
    children: Children,
) -> impl IntoView {
    view! {
        <KanbanBoardRoot
            on_move=on_move
            class=move || { cn!("flex w-full items-start gap-3 overflow-x-auto pb-2", class.get()) }
        >
            {children()}
        </KanbanBoardRoot>
    }
}

#[component]
pub fn KanbanColumn(
    #[prop(into)] id: String,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <KanbanColumnRoot
            id=id
            class=move || {
                cn!(
                    "flex w-64 shrink-0 flex-col gap-2 rounded-lg border bg-muted/40 p-2 transition-colors",
                    "data-[over=true]:border-primary/60 data-[over=true]:bg-primary/5",
                    class.get(),
                )
            }
        >
            {children()}
        </KanbanColumnRoot>
    }
}

#[component]
pub fn KanbanColumnHeader(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="kanban-column-header"
            class=move || {
                cn!(
                    "flex items-center justify-between px-1 text-xs font-medium text-muted-foreground",
                    class.get(),
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn KanbanCard(
    #[prop(into)] id: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <KanbanCardRoot
            id=id
            disabled=disabled
            class=move || {
                cn!(
                    "touch-none rounded-md border bg-card p-2.5 text-sm shadow-sm transition-[opacity,box-shadow] select-none",
                    "hover:shadow-md",
                    // The card stays where it is and fades: a board that pulled it out of the
                    // list would reflow every column under the pointer mid-gesture.
                    "data-[dragging=true]:opacity-40 data-[dragging=true]:shadow-none",
                    class.get(),
                )
            }
        >
            {children()}
        </KanbanCardRoot>
    }
}

/// The line a card would land on. Render one between each pair of cards, and one after the last.
#[component]
pub fn KanbanSlot(index: usize, #[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! {
        <KanbanSlotRoot
            index=index
            class=move || {
                cn!(
                    "h-0.5 rounded-full bg-transparent transition-colors",
                    "data-[active=true]:bg-primary",
                    class.get(),
                )
            }
        />
    }
}
