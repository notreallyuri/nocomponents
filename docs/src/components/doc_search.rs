//! The ⌘K palette, over the same `NAV` the sidebar is built from.

use crate::{app::href, layout::doc_layout::NAV};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use nocomponents::{
    components::{
        button::{Button, ButtonVariant},
        command::{
            CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
        },
        kbd::Kbd,
    },
    icons::search::Search,
};

#[component]
pub fn DocSearch() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        // A button that reads as a search field on a wide screen and as an icon on a narrow one,
        // where the header has no room for a field it does not really need.
        <Button
            variant=ButtonVariant::Outline
            class="w-8 justify-center gap-2 px-0 font-normal text-muted-foreground sm:w-56 sm:justify-between sm:px-2.5"
            on_click=Callback::new(move |_| open.set(true))
        >
            <span class="flex items-center gap-2">
                <Search class="size-4" />
                <span class="hidden sm:inline">"Search components…"</span>
                <span class="sr-only sm:hidden">"Search components"</span>
            </span>
            <Kbd class="hidden sm:inline-flex">"⌘K"</Kbd>
        </Button>

        <CommandDialog
            open=open
            title="Search the docs"
            description="Jump to any page in the documentation."
        >
            <CommandInput placeholder="Search components…" />
            <CommandList>
                <CommandEmpty>"No pages found."</CommandEmpty>
                {NAV
                    .iter()
                    .map(|group| {
                        let mut items = group.items.to_vec();
                        if group.label == "Components" {
                            items.sort_by_key(|item| item.label.to_lowercase());
                        }

                        view! {
                            <CommandGroup heading=group
                                .label>
                                {items
                                    .into_iter()
                                    .map(|item| {
                                        let navigate = use_navigate();
                                        let target = href(item.href);
                                        // Minted per item rather than once above: the palette's
                                        // children are a `ChildrenFn`, and a single navigator
                                        // cloned into them would move out of a `Fn` closure.

                                        view! {
                                            <CommandItem
                                                value=item.label
                                                on_select=Callback::new(move |_| {
                                                    open.set(false);
                                                    navigate(&target, Default::default());
                                                })
                                            >
                                                {item.label}
                                            </CommandItem>
                                        }
                                    })
                                    .collect_view()}
                            </CommandGroup>
                        }
                    })
                    .collect_view()}
            </CommandList>
        </CommandDialog>
    }
}
