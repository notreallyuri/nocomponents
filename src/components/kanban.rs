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
    #[prop(default = None, into)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <KanbanCardRoot
            id=id
            disabled=disabled
            on_select=on_select
            class=move || {
                cn!(
                    "touch-none rounded-md border bg-card p-2.5 text-sm shadow-sm select-none",
                    // Never `transition-transform`: the card is following a pointer, and easing
                    // it would have it lag behind the cursor by the duration.
                    "transition-[box-shadow,scale] hover:shadow-md",
                    // Lifted, not faded — the card is the thing in hand, so it reads as picked
                    // up rather than as a hole in the list. `relative` is what lets `z-50` win.
                    "data-[dragging=true]:relative data-[dragging=true]:z-50 data-[dragging=true]:scale-[1.02] data-[dragging=true]:cursor-grabbing data-[dragging=true]:shadow-lg data-[dragging=true]:ring-2 data-[dragging=true]:ring-primary/40",
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
