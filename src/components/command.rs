use crate::{
    cn,
    components::dialog::{DialogDescription, DialogHeader, DialogPortal, DialogTitle},
    icons::search::Search,
    primitives::{
        command::{
            CommandEmptyRoot, CommandFilter, CommandGroupLabelRoot, CommandGroupRoot,
            CommandInputRoot, CommandItemRoot, CommandListRoot, CommandRoot, CommandSeparatorRoot,
            use_command_shortcut,
        },
        dialog::DialogRoot,
    },
};
use leptos::prelude::*;

#[component]
pub fn Command(
    #[prop(optional, into)] search: RwSignal<String>,
    #[prop(optional, into)] value: RwSignal<Option<String>>,
    #[prop(default = true)] should_filter: bool,
    #[prop(default = None, into)] filter: Option<CommandFilter>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <CommandRoot
            search=search
            value=value
            should_filter=should_filter
            filter=filter
            class=move || {
                cn!(
                    "flex h-full w-full flex-col overflow-hidden rounded-lg bg-popover text-popover-foreground",
                    class.get()
                )
            }
        >
            {children()}
        </CommandRoot>
    }
}

#[component]
pub fn CommandInput(
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] auto_focus: bool,
) -> impl IntoView {
    view! {
        <div class="flex h-11 items-center gap-2 border-b px-3">
            <Search class="size-4 shrink-0 text-muted-foreground" />
            <CommandInputRoot
                placeholder=placeholder
                auto_focus=auto_focus
                class=move || {
                    cn!(
                        "flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
                        class.get()
                    )
                }
            />
        </div>
    }
}

#[component]
pub fn CommandList(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        // No `overflow-x-hidden` beside it: `tw_merge` treats the two axes as one group and drops
        // whichever comes first, so nothing in here may be wider than the list — which is why the
        // separator does not pull itself out past the edges the way a menu's does.
        <CommandListRoot class=move || {
            cn!("max-h-80 scroll-py-1 overflow-y-auto", class.get())
        }>{children()}</CommandListRoot>
    }
}

#[component]
pub fn CommandEmpty(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <CommandEmptyRoot class=move || {
            cn!("py-6 text-center text-sm text-muted-foreground", class.get())
        }>{children()}</CommandEmptyRoot>
    }
}

#[component]
pub fn CommandGroup(
    #[prop(optional, into)] heading: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <CommandGroupRoot class=move || cn!("overflow-hidden p-1 text-foreground", class.get())>
            <Show when=move || !heading.get().is_empty()>
                <CommandGroupLabelRoot class="px-2 py-1.5 text-xs font-medium text-muted-foreground">
                    {move || heading.get()}
                </CommandGroupLabelRoot>
            </Show>
            {children()}
        </CommandGroupRoot>
    }
}

#[component]
pub fn CommandItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] keywords: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = None, into)] on_select: Option<Callback<String>>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <CommandItemRoot
            value=value
            keywords=keywords
            disabled=disabled
            on_select=on_select
            class=move || {
                cn!(
                    "relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none select-none data-[state=active]:bg-accent data-[state=active]:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:size-4 [&_svg]:shrink-0 [&_svg]:text-muted-foreground",
                    class.get()
                )
            }
        >
            {children()}
        </CommandItemRoot>
    }
}

#[component]
pub fn CommandSeparator(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    view! { <CommandSeparatorRoot class=move || cn!("h-px bg-border", class.get()) /> }
}

#[component]
pub fn CommandShortcut(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            data-slot="command-shortcut"
            class=move || {
                cn!("ml-auto text-xs tracking-widest text-muted-foreground", class.get())
            }
        >
            {children()}
        </span>
    }
}

#[component]
pub fn CommandDialog(
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(default = Some("k"))] shortcut: Option<&'static str>,
    #[prop(optional, into)] title: Signal<String>,
    #[prop(optional, into)] description: Signal<String>,
    #[prop(optional, into)] search: RwSignal<String>,
    #[prop(optional, into)] value: RwSignal<Option<String>>,
    #[prop(default = true)] should_filter: bool,
    #[prop(default = None, into)] filter: Option<CommandFilter>,
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let is_open = open.unwrap_or_else(|| RwSignal::new(false));

    if let Some(shortcut) = shortcut {
        use_command_shortcut(shortcut, is_open);
    }

    // Cleared on close rather than on open: a palette that reopens showing the last search, then
    // blanks it a frame later, reads as a glitch.
    Effect::new(move |_| {
        if !is_open.get() {
            search.set(String::new());
        }
    });

    let children = StoredValue::new(children);

    view! {
        <DialogRoot open=is_open>
            <DialogPortal
                show_close=false
                class=move || { cn!("overflow-hidden p-0 sm:max-w-lg", class.get()) }
            >
                <DialogHeader class="sr-only">
                    <DialogTitle>
                        {move || {
                            let title = title.get();
                            if title.is_empty() { "Command palette".to_string() } else { title }
                        }}
                    </DialogTitle>
                    <DialogDescription>
                        {move || {
                            let description = description.get();
                            if description.is_empty() {
                                "Search for a command to run.".to_string()
                            } else {
                                description
                            }
                        }}
                    </DialogDescription>
                </DialogHeader>

                <Command
                    search=search
                    value=value
                    should_filter=should_filter
                    filter=filter
                    class="rounded-none"
                >
                    {children.with_value(|children| children())}
                </Command>
            </DialogPortal>
        </DialogRoot>
    }
}
