//! Command palette blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        alert_dialog::{
            AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogDescription,
            AlertDialogFooter, AlertDialogHeader, AlertDialogPortal, AlertDialogTitle,
        },
        button::{Button, ButtonVariant},
        command::{
            CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
            CommandSeparator,
        },
    },
    utils::theme::{Theme, use_theme},
};

const PALETTE: &str = r#"let open = RwSignal::new(false);
let confirming = RwSignal::new(false);
let theme = use_theme();

view! {
    <Button variant=ButtonVariant::Outline on_click=Callback::new(move |_| open.set(true))>
        "Open palette"
    </Button>

    // `shortcut=None`: this page already has a ⌘K, and two palettes answering one
    // chord is worse than either. A palette that owns the page keeps the default.
    <CommandDialog open=open shortcut=None title="Commands" description="Run a command.">
        <CommandInput placeholder="Type a command…" />
        <CommandList>
            <CommandEmpty>"Nothing matches."</CommandEmpty>
            <CommandGroup heading="Appearance">
                <CommandItem
                    value="Light theme"
                    on_select=Callback::new(move |_| {
                        theme.set_theme(Theme::Light);
                        open.set(false);
                    })
                >
                    "Light theme"
                </CommandItem>
            </CommandGroup>
            <CommandSeparator />
            <CommandGroup heading="Danger zone">
                // Closes one layer and opens another in the same click.
                <CommandItem
                    value="Delete project"
                    on_select=Callback::new(move |_| {
                        open.set(false);
                        confirming.set(true);
                    })
                >
                    "Delete project…"
                </CommandItem>
            </CommandGroup>
        </CommandList>
    </CommandDialog>

    <AlertDialog open=confirming>
        <AlertDialogPortal>
            <AlertDialogHeader>
                <AlertDialogTitle>"Delete this project?"</AlertDialogTitle>
            </AlertDialogHeader>
            <AlertDialogFooter>
                <AlertDialogCancel>"Keep it"</AlertDialogCancel>
                <AlertDialogAction on_click=Callback::new(move |_| { /* … */ })>
                    "Delete"
                </AlertDialogAction>
            </AlertDialogFooter>
        </AlertDialogPortal>
    </AlertDialog>
}"#;

pub const BLOCKS: &[Block] = &[Block {
    title: "A palette that acts",
    description: "Not a jump list — every item here does something, which is the other half of \
                  what a palette is for. Two seams. `shortcut=None`, because this page already \
                  binds ⌘K for its own search and a chord that opens two palettes is a bug the \
                  reader gets to see rather than read about. And \"Delete project…\" closes the \
                  palette and opens an alert in the same click: two layers changing places, with \
                  the escape key and the focus return going to the one that arrived last.",
    code: PALETTE,
    view: || view! { <ActionPalette /> }.into_any(),
}];

#[component]
fn ActionPalette() -> impl IntoView {
    let open = RwSignal::new(false);
    let confirming = RwSignal::new(false);
    let last = RwSignal::new(String::new());
    let theme = use_theme();

    let appearance = move |label: &'static str, choice: Theme| {
        view! {
            <CommandItem
                value=label
                on_select=Callback::new(move |_| {
                    theme.set_theme(choice);
                    last.set(format!("Switched to the {} theme", choice.as_str()));
                    open.set(false);
                })
            >
                {label}
            </CommandItem>
        }
    };

    view! {
        <div class="flex flex-col items-start gap-3">
            <Button variant=ButtonVariant::Outline on_click=Callback::new(move |_| open.set(true))>
                "Open palette"
            </Button>

            <p class="h-5 text-sm text-muted-foreground">{move || last.get()}</p>

            <CommandDialog
                open=open
                shortcut=None
                title="Commands"
                description="Run a command from anywhere in the project."
            >
                <CommandInput placeholder="Type a command…" />
                <CommandList>
                    <CommandEmpty>"Nothing matches."</CommandEmpty>
                    <CommandGroup heading="Appearance">
                        {appearance("Light theme", Theme::Light)}
                        {appearance("Dark theme", Theme::Dark)}
                        {appearance("System theme", Theme::System)}
                    </CommandGroup>
                    <CommandSeparator />
                    <CommandGroup heading="Danger zone">
                        <CommandItem
                            value="Delete project"
                            keywords="remove destroy"
                            on_select=Callback::new(move |_| {
                                open.set(false);
                                confirming.set(true);
                            })
                        >
                            "Delete project…"
                        </CommandItem>
                    </CommandGroup>
                </CommandList>
            </CommandDialog>

            <AlertDialog open=confirming>
                <AlertDialogPortal>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"Delete this project?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "Nothing is actually deleted — this is a demonstration of one layer
                            handing over to another."
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>"Keep it"</AlertDialogCancel>
                        <AlertDialogAction on_click=Callback::new(move |_| {
                            last.set("Deleted the project".to_string())
                        })>"Delete"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogPortal>
            </AlertDialog>
        </div>
    }
}
