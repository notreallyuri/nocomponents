use crate::utils::controllable::forward;
use crate::{
    cn,
    primitives::kanban::{KanbanBoardRoot, KanbanCardRoot, KanbanColumnRoot, KanbanSlotRoot},
};
use leptos::prelude::*;

#[allow(unused_imports)]
pub use crate::primitives::kanban::KanbanMove;

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
    #[prop(optional, into)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <KanbanCardRoot
            id=id
            disabled=disabled
            on_select=forward(on_select)
            class=move || {
                cn!(
                    "touch-none rounded-md border bg-card p-2.5 text-sm shadow-sm select-none",
                    "transition-[box-shadow,scale] hover:shadow-md",
                    "data-[dragging=true]:relative data-[dragging=true]:z-50 data-[dragging=true]:scale-[1.02] data-[dragging=true]:cursor-grabbing data-[dragging=true]:shadow-lg data-[dragging=true]:ring-2 data-[dragging=true]:ring-primary/40",
                    class.get(),
                )
            }
        >
            {children()}
        </KanbanCardRoot>
    }
}

#[component]
pub fn KanbanSlot(
    #[prop(into)] index: Signal<usize>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
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
