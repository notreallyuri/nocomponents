//! Combobox blocks.

use super::Block;
use leptos::prelude::*;
use nocomponents::{
    components::{
        avatar::{Avatar, AvatarFallback},
        badge::{Badge, BadgeVariant},
        combobox::{Combobox, ComboboxContent, ComboboxItem, ComboboxTrigger},
        command::{CommandEmpty, CommandInput, CommandList},
        field::{Field, FieldDescription, FieldLabel},
    },
    icons::x::X,
};

#[derive(Clone, Copy, PartialEq)]
struct Person {
    id: &'static str,
    name: &'static str,
    team: &'static str,
    initials: &'static str,
}

#[rustfmt::skip]
const PEOPLE: &[Person] = &[
    Person { id: "ana",   name: "Ana Ferreira",  team: "Platform",  initials: "AF" },
    Person { id: "bo",    name: "Bo Lindqvist",  team: "Design",    initials: "BL" },
    Person { id: "chidi", name: "Chidi Okonkwo", team: "Platform",  initials: "CO" },
    Person { id: "dana",  name: "Dana Whitfield", team: "Docs",     initials: "DW" },
    Person { id: "emre",  name: "Emre Yilmaz",   team: "Design",    initials: "EY" },
    Person { id: "farah", name: "Farah Haddad",  team: "Security",  initials: "FH" },
];

const MULTI: &str = r##"// The combobox's own value is one `Option<String>`. A multi-select is a
// list, so the list is the caller's and the items are told two things:
// what makes them look chosen, and not to close the layer on the way out.
let chosen = RwSignal::new(vec!["ana".to_string()]);

let toggle = move |id: String| {
    chosen.update(|chosen| match chosen.iter().position(|held| *held == id) {
        Some(index) => { chosen.remove(index); }
        None => chosen.push(id),
    })
};

view! {
    <Field class="w-80">
        // The label names the trigger, not an id nothing wears: a combobox
        // trigger hands the field the id it already has.
        <FieldLabel>"Reviewers"</FieldLabel>

        <Combobox>
            // A verb, not a value: the badges below are what is chosen, and the
            // trigger draws its own chevron.
            <ComboboxTrigger class="w-full justify-between font-normal">
                "Add reviewers…"
            </ComboboxTrigger>

            <ComboboxContent class="w-80">
                <CommandInput placeholder="Search people…" />
                <CommandList>
                    <CommandEmpty>"Nobody by that name."</CommandEmpty>
                    <ComboboxItem
                        value=person.id
                        label=person.name
                        keywords=person.team
                        // Ticked from the caller's list rather than from the
                        // combobox's value, and the layer stays open: closing
                        // after the first pick is the bug in a multi-select.
                        selected=Signal::derive(move || holds(chosen, person.id))
                        close_on_select=false
                        on_select=Callback::new(move |id| toggle(id))
                    >
                        <Avatar class="size-6"><AvatarFallback>{person.initials}</AvatarFallback></Avatar>
                        <span>{person.name}</span>
                        <span class="ml-auto text-xs text-muted-foreground">{person.team}</span>
                    </ComboboxItem>
                </CommandList>
            </ComboboxContent>
        </Combobox>

        // The badges are the same list read a second time, so removing one
        // un-ticks the row. They sit in the field rather than in the trigger
        // because each carries a button, and a button inside a button is not
        // a thing the HTML parser will keep.
        <div class="flex flex-wrap gap-1">
            <Badge variant=BadgeVariant::Secondary class="gap-1 pr-1">
                {person.name}
                <button
                    type="button"
                    class="rounded-sm opacity-60 hover:opacity-100"
                    on:click=move |_| toggle(person.id.to_string())
                >
                    <X class="size-3" />
                    <span class="sr-only">{format!("Remove {}", person.name)}</span>
                </button>
            </Badge>
        </div>

        <FieldDescription>"Anyone here can approve the change."</FieldDescription>
    </Field>
}"##;

pub const BLOCKS: &[Block] = &[Block {
    title: "Picking several people at once",
    description: "A combobox comes back with *one* value — that is what its `value` is, an \
                  `Option<String>` — so a multi-select is not a mode it has, it is a list the \
                  caller keeps and two things the items are told: what makes them look chosen, \
                  and not to close the layer on the way out. Closing after the first pick is the \
                  bug this replaces; staying open also keeps the search text, so three matches of \
                  one query can be taken in three keystrokes. Everything on screen is that one \
                  list read again — the ticks beside the rows and the badges under the field — so \
                  removing a badge un-ticks a row without either being told about the other. The \
                  trigger stays a verb rather than a value: the badges are what is chosen, and a \
                  trigger repeating the one name in them says nothing twice. The badges are in the field rather than inside the trigger \
                  because each one carries a remove button, and a button inside a button is not \
                  something the HTML parser will keep; in the field they are ordinary buttons \
                  with ordinary labels, which is also what a screen reader needs. The label \
                  points at the trigger itself: a combobox trigger already has an id from its \
                  floating root, so the field is handed that one rather than minting a second \
                  that nothing wears.",
    code: MULTI,
    view: || view! { <ReviewerField /> }.into_any(),
}];

fn person(id: &str) -> Person {
    PEOPLE
        .iter()
        .copied()
        .find(|person| person.id == id)
        .unwrap_or(PEOPLE[0])
}

#[component]
fn ReviewerField() -> impl IntoView {
    // The selection lives out here, because a list is not what the combobox holds.
    let chosen = RwSignal::new(vec!["ana".to_string()]);

    let holds = move |id: &'static str| chosen.with(|chosen| chosen.iter().any(|held| held == id));

    let toggle = move |id: &'static str| {
        chosen.update(|chosen| match chosen.iter().position(|held| held == id) {
            Some(index) => {
                chosen.remove(index);
            }
            None => chosen.push(id.to_string()),
        })
    };

    view! {
        <Field class="w-80">
            <FieldLabel>"Reviewers"</FieldLabel>

            <Combobox>
                // A verb, not a value: the badges below are what is chosen, and a trigger
                // repeating the one name in them says nothing twice.
                <ComboboxTrigger class="w-full justify-between font-normal">
                    "Add reviewers…"
                </ComboboxTrigger>

                <ComboboxContent class="w-80">
                    <CommandInput placeholder="Search people…" />
                    <CommandList>
                        <CommandEmpty>"Nobody by that name."</CommandEmpty>
                        {PEOPLE
                            .iter()
                            .map(|person| {
                                let Person { id, name, team, initials } = *person;

                                view! {
                                    <ComboboxItem
                                        value=id
                                        label=name
                                        keywords=team
                                        selected=Signal::derive(move || holds(id))
                                        close_on_select=false
                                        on_select=Callback::new(move |_| toggle(id))
                                    >
                                        <Avatar class="size-6">
                                            <AvatarFallback class="text-[10px]">
                                                {initials}
                                            </AvatarFallback>
                                        </Avatar>
                                        <span>{name}</span>
                                        <span class="ml-auto text-xs text-muted-foreground">
                                            {team}
                                        </span>
                                    </ComboboxItem>
                                }
                            })
                            .collect_view()}
                    </CommandList>
                </ComboboxContent>
            </Combobox>

            // The same list again: remove a badge and its row un-ticks itself.
            <div class="flex flex-wrap gap-1">
                {move || {
                    chosen
                        .get()
                        .into_iter()
                        .map(|id| {
                            let Person { id, name, .. } = person(&id);

                            view! {
                                <Badge variant=BadgeVariant::Secondary class="gap-1 pr-1">
                                    {name}
                                    <button
                                        type="button"
                                        class="rounded-sm opacity-60 transition-opacity hover:opacity-100"
                                        on:click=move |_| toggle(id)
                                    >
                                        <X class="size-3" />
                                        <span class="sr-only">{format!("Remove {name}")}</span>
                                    </button>
                                </Badge>
                            }
                        })
                        .collect_view()
                }}
            </div>

            <FieldDescription>"Anyone here can approve the change."</FieldDescription>
        </Field>
    }
}
