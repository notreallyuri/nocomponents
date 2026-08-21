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
        button::{Button, ButtonVariant},
        command::{
            Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem,
            CommandList, CommandSeparator, CommandShortcut,
        },
        kbd::Kbd,
    },
    icons::{copy::Copy, panel::PanelLeft, search::Search},
};

const DEFAULT: &str = r#"<Command class="rounded-lg border">
    <CommandInput placeholder="Type a command or search…" />
    <CommandList>
        <CommandEmpty>"No results found."</CommandEmpty>
        <CommandGroup heading="Suggestions">
            <CommandItem value="Search docs" on_select=record>
                <Search />
                "Search docs"
            </CommandItem>
            <CommandItem value="Copy snippet" on_select=record>
                <Copy />
                "Copy snippet"
            </CommandItem>
            <CommandItem value="Toggle sidebar" on_select=record>
                <PanelLeft />
                "Toggle sidebar"
                <CommandShortcut>"⌘B"</CommandShortcut>
            </CommandItem>
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Settings">
            <CommandItem value="Profile" on_select=record>
                "Profile"
                <CommandShortcut>"⌘P"</CommandShortcut>
            </CommandItem>
            <CommandItem value="Sign out" keywords="logout exit session" on_select=record>
                "Sign out"
            </CommandItem>
            <CommandItem value="Billing" disabled=true>"Billing"</CommandItem>
        </CommandGroup>
    </CommandList>
</Command>"#;

const DIALOG: &str = r#"{
    let open = RwSignal::new(false);
    view! {
        <Button variant=ButtonVariant::Outline on:click=move |_| open.set(true)>
            "Open palette"
            <Kbd>"⌘K"</Kbd>
        </Button>

        // `shortcut` defaults to "k"; the listener is on the document, so the palette
        // opens from anywhere on the page.
        <CommandDialog open=open>
            <CommandInput placeholder="Type a command or search…" />
            <CommandList>
                <CommandEmpty>"No results found."</CommandEmpty>
                <CommandGroup heading="Pages">
                    <CommandItem value="Dialog" on_select=go>"Dialog"</CommandItem>
                    <CommandItem value="Popover" on_select=go>"Popover"</CommandItem>
                </CommandGroup>
            </CommandList>
        </CommandDialog>
    }
}"#;

const KEYWORDS: &str = r#"<Command class="rounded-lg border">
    <CommandInput placeholder="Try 'logout', or 'appearance'…" />
    <CommandList>
        <CommandEmpty>"No results found."</CommandEmpty>
        <CommandItem value="Sign out" keywords="logout exit session">"Sign out"</CommandItem>
        <CommandItem value="Toggle theme" keywords="dark light appearance">
            "Toggle theme"
        </CommandItem>
        <CommandItem value="Invite a teammate" keywords="member seat">
            "Invite a teammate"
        </CommandItem>
    </CommandList>
</Command>"#;

const FILTERING: &str = r#"<Command filter=Callback::new(|(search, haystack): (String, String)| {
    // Every letter of the search, in order, somewhere in the item.
    let mut haystack = haystack.to_lowercase().chars().collect::<Vec<_>>().into_iter();
    search.to_lowercase().chars().all(|needle| haystack.any(|c| c == needle))
})>
    <CommandInput placeholder="Fuzzy: try 'sgn'…" />
    <CommandList>
        <CommandEmpty>"Nothing matches."</CommandEmpty>
        <CommandItem value="Sign out">"Sign out"</CommandItem>
        <CommandItem value="Save and quit">"Save and quit"</CommandItem>
        <CommandItem value="Settings">"Settings"</CommandItem>
    </CommandList>
</Command>"#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Command",
        description: "The palette. Owns the search, the highlight and the count of what still matches.",
        props: &[
            Prop {
                name: "search",
                ty: "RwSignal<String>",
                default: "\"\"",
                description: "What is typed into the input. Pass one to read it, preset it or clear it from outside.",
            },
            Prop {
                name: "value",
                ty: "RwSignal<Option<String>>",
                default: "None",
                description: "The value of the last item activated, for a caller who would rather read a signal than pass `on_select` to every item.",
            },
            Prop {
                name: "should_filter",
                ty: "bool",
                default: "true",
                description: "Off when the list arrives already narrowed — a search that went to a server has no business being filtered again on the way in.",
            },
            Prop {
                name: "filter",
                ty: "Option<Callback<(String, String), bool>>",
                default: "None",
                description: "Replaces the default match. Takes the search and the item's value plus its keywords.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the palette's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "An input and a list.",
            },
        ],
    },
    ApiEntry {
        name: "CommandInput",
        description: "The search box. A `role=combobox` that keeps focus while the arrows move the highlight through `aria-activedescendant`.",
        props: &[
            Prop {
                name: "placeholder",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The input's placeholder.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the input's classes, not the row around it.",
            },
            Prop {
                name: "auto_focus",
                ty: "bool",
                default: "false",
                description: "Takes focus on mount. A palette in a dialog does not need it — the dialog focuses the first thing it contains, which is this.",
            },
        ],
    },
    ApiEntry {
        name: "CommandList",
        description: "The scroll port. The highlight is kept inside it, and only this element scrolls — not the page behind it.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the list's classes; where to change `max-h-80`.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "Groups, items, separators and an empty state.",
            },
        ],
    },
    ApiEntry {
        name: "CommandEmpty",
        description: "Shown only while nothing matches, and not rendered otherwise.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the empty state's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "What to say when there is nothing. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
    ApiEntry {
        name: "CommandGroup",
        description: "A labelled run of items. Takes itself out of the flow when every item in it has been filtered away.",
        props: &[
            Prop {
                name: "heading",
                ty: "Signal<String>",
                default: "\"\"",
                description: "The group's label, which is what `aria-labelledby` points at. Leave it empty for a group that only exists to be ruled off.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the group's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The group's items.",
            },
        ],
    },
    ApiEntry {
        name: "CommandItem",
        description: "One command. Matched on its value and keywords; activated by click or by Enter, which clicks it.",
        props: &[
            Prop {
                name: "value",
                ty: "String",
                default: "",
                description: "What the item is matched and reported by. Unique within the palette — it is the handle the highlight is carried on.",
            },
            Prop {
                name: "keywords",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Extra words the item can be found by but which are not shown: \"logout\" on a \"Sign out\" item.",
            },
            Prop {
                name: "disabled",
                ty: "Signal<bool>",
                default: "false",
                description: "Skipped by the arrows and unclickable; still counted as a match, so it does not turn the empty state on.",
            },
            Prop {
                name: "on_select",
                ty: "Option<Callback<String>>",
                default: "None",
                description: "Run on activation, with the item's value.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the item's classes.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The item's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
    ApiEntry {
        name: "CommandSeparator",
        description: "A rule between groups.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the rule's classes.",
        }],
    },
    ApiEntry {
        name: "CommandShortcut",
        description: "The keyboard hint on the right of an item. Presentational — it binds nothing.",
        props: &[
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the hint's classes.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "The keys, written out.",
            },
        ],
    },
    ApiEntry {
        name: "CommandDialog",
        description: "The palette as a dialog, with the Ctrl/Cmd shortcut wired up and a title that is read out but not shown.",
        props: &[
            Prop {
                name: "open",
                ty: "Option<RwSignal<bool>>",
                default: "None",
                description: "Pass one to drive it from outside; omit it and the dialog owns the signal.",
            },
            Prop {
                name: "shortcut",
                ty: "Option<&'static str>",
                default: "Some(\"k\")",
                description: "A single unshifted key that toggles the palette with Ctrl or Cmd held. `None` leaves the shortcut to the caller.",
            },
            Prop {
                name: "title",
                ty: "Signal<String>",
                default: "\"Command palette\"",
                description: "Announced, not shown. A dialog has to be named.",
            },
            Prop {
                name: "description",
                ty: "Signal<String>",
                default: "\"Search for a command to run.\"",
                description: "Announced, not shown.",
            },
            Prop {
                name: "search",
                ty: "RwSignal<String>",
                default: "\"\"",
                description: "As on `Command`. Cleared when the dialog closes.",
            },
            Prop {
                name: "value",
                ty: "RwSignal<Option<String>>",
                default: "None",
                description: "As on `Command`.",
            },
            Prop {
                name: "should_filter",
                ty: "bool",
                default: "true",
                description: "As on `Command`.",
            },
            Prop {
                name: "filter",
                ty: "Option<Callback<(String, String), bool>>",
                default: "None",
                description: "As on `Command`.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged onto the dialog panel.",
            },
            Prop {
                name: "children",
                ty: "ChildrenFn",
                default: "",
                description: "The palette's contents. Re-run on each mount, hence `ChildrenFn`.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let open = RwSignal::new(false);
    let picked = RwSignal::new(None::<String>);
    let ran = RwSignal::new(None::<String>);

    let record = Callback::new(move |value: String| ran.set(Some(value)));
    let close = Callback::new(move |value: String| {
        picked.set(Some(value));
        open.set(false);
    });

    view! {
        <DocLayout
            title="Command"
            description="A list of commands, filtered by what you type — the ⌘K palette."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Focus stays in the input the whole time: the arrows move the highlight, not the focus ring, and Enter clicks whatever is highlighted."
                    code=DEFAULT
                >
                    <div class="flex flex-col gap-3">
                        <Command class="rounded-lg border">
                            <CommandInput placeholder="Type a command or search…" />
                            <CommandList>
                                <CommandEmpty>"No results found."</CommandEmpty>
                                <CommandGroup heading="Suggestions">
                                    <CommandItem value="Search docs" on_select=record>
                                        <Search />
                                        "Search docs"
                                    </CommandItem>
                                    <CommandItem value="Copy snippet" on_select=record>
                                        <Copy />
                                        "Copy snippet"
                                    </CommandItem>
                                    <CommandItem value="Toggle sidebar" on_select=record>
                                        <PanelLeft />
                                        "Toggle sidebar"
                                        <CommandShortcut>"⌘B"</CommandShortcut>
                                    </CommandItem>
                                </CommandGroup>
                                <CommandSeparator />
                                <CommandGroup heading="Settings">
                                    <CommandItem value="Profile" on_select=record>
                                        "Profile"
                                        <CommandShortcut>"⌘P"</CommandShortcut>
                                    </CommandItem>
                                    <CommandItem
                                        value="Sign out"
                                        keywords="logout exit session"
                                        on_select=record
                                    >
                                        "Sign out"
                                    </CommandItem>
                                    <CommandItem value="Billing" disabled=true>
                                        "Billing"
                                    </CommandItem>
                                </CommandGroup>
                            </CommandList>
                        </Command>
                        <p class="text-sm text-muted-foreground">
                            {move || match ran.get() {
                                Some(value) => format!("Ran: {value}"),
                                None => "Nothing run yet.".to_string(),
                            }}
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Dialog"
                    description="The same palette in a dialog, opened by Ctrl+K — or ⌘K — from anywhere on the page. The dialog is titled for screen readers without showing a heading."
                    code=DIALOG
                >
                    <div class="flex flex-wrap items-center gap-3">
                        <Button variant=ButtonVariant::Outline on:click=move |_| open.set(true)>
                            "Open palette"
                            <Kbd>"⌘K"</Kbd>
                        </Button>
                        <span class="text-sm text-muted-foreground">
                            {move || match picked.get() {
                                Some(value) => format!("Went to: {value}"),
                                None => "Try ⌘K.".to_string(),
                            }}
                        </span>

                        <CommandDialog open=open>
                            <CommandInput placeholder="Type a command or search…" />
                            <CommandList>
                                <CommandEmpty>"No results found."</CommandEmpty>
                                <CommandGroup heading="Pages">
                                    <CommandItem value="Dialog" on_select=close>
                                        "Dialog"
                                    </CommandItem>
                                    <CommandItem value="Popover" on_select=close>
                                        "Popover"
                                    </CommandItem>
                                    <CommandItem value="Dropdown Menu" on_select=close>
                                        "Dropdown Menu"
                                    </CommandItem>
                                </CommandGroup>
                                <CommandSeparator />
                                <CommandGroup heading="Actions">
                                    <CommandItem
                                        value="Toggle theme"
                                        keywords="dark light appearance"
                                        on_select=close
                                    >
                                        "Toggle theme"
                                        <CommandShortcut>"⌘T"</CommandShortcut>
                                    </CommandItem>
                                </CommandGroup>
                            </CommandList>
                        </CommandDialog>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Keywords"
                    description="An item is matched on its value and on keywords that are never shown, so 'logout' finds 'Sign out'. The default match wants every whitespace-separated term somewhere in one of the two, in any order."
                    code=KEYWORDS
                >
                    <Command class="rounded-lg border">
                        <CommandInput placeholder="Try 'logout', or 'appearance'…" />
                        <CommandList>
                            <CommandEmpty>"No results found."</CommandEmpty>
                            <CommandItem value="Sign out" keywords="logout exit session">
                                "Sign out"
                            </CommandItem>
                            <CommandItem value="Toggle theme" keywords="dark light appearance">
                                "Toggle theme"
                            </CommandItem>
                            <CommandItem value="Invite a teammate" keywords="member seat">
                                "Invite a teammate"
                            </CommandItem>
                        </CommandList>
                    </Command>
                </DemoSection>

                <DemoSection
                    title="Filtering"
                    description="`filter` replaces the substring match — here with a subsequence one, so 'sgn' finds 'Sign out'. `should_filter=false` turns filtering off altogether, for a list that was narrowed somewhere else."
                    code=FILTERING
                >
                    <Command
                        class="rounded-lg border"
                        filter=Callback::new(|(search, haystack): (String, String)| {
                            let mut haystack = haystack
                                .to_lowercase()
                                .chars()
                                .collect::<Vec<_>>()
                                .into_iter();
                            search
                                .to_lowercase()
                                .chars()
                                .all(|needle| haystack.any(|c| c == needle))
                        })
                    >
                        <CommandInput placeholder="Fuzzy: try 'sgn'…" />
                        <CommandList>
                            <CommandEmpty>"Nothing matches."</CommandEmpty>
                            <CommandItem value="Sign out">"Sign out"</CommandItem>
                            <CommandItem value="Save and quit">"Save and quit"</CommandItem>
                            <CommandItem value="Settings">"Settings"</CommandItem>
                        </CommandList>
                    </Command>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
