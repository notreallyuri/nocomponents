//! `use_roving_focus`.

use super::{Primitive, Signature, Signatures};
use crate::components::demo_section::DemoSection;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use nocomponents::{primitives::roving_focus::use_roving_focus, utils::types::Orientation};

pub const PRIMITIVE: Primitive = Primitive {
    slug: "roving-focus",
    name: "use_roving_focus",
    blurb: "A group of things that is one tab stop, with the arrows moving inside it. Items are \
            found in the DOM by a marker, so it does not care how they are rendered.",
    used_by: &[
        "accordion",
        "context-menu",
        "dropdown-menu",
        "menubar",
        "navigation-menu",
        "radio-group",
        "select",
        "tabs",
        "toggle-group",
        "tree",
    ],
    view: page,
};

const USE: &str = r#"use nocomponents::primitives::roving_focus::{use_roving_focus, ROVING_ITEM};

let list = AnyNodeRef::new();
let roving = use_roving_focus(list, Orientation::Vertical);

view! {
    <div
        node_ref=list
        on:keydown=move |e| { if roving.on_keydown(&e.key()) { e.prevent_default(); } }
        on:focusin=move |_| roving.on_focus_in()
    >
        // Every item marked, and every item `tabindex="-1"`. The group hands
        // the tab stop to whichever one focus last landed on.
        <button data-roving-item="" tabindex="-1">"One"</button>
        <button data-roving-item="" tabindex="-1">"Two"</button>
    </div>
}"#;

const API: &[Signature] = &[
    Signature {
        name: "use_roving_focus",
        shape: "fn(AnyNodeRef, impl Into<Signal<Orientation>>) -> RovingFocus",
        what: "The container to search inside, and which arrows move through it. Orientation is a signal, so a group that turns takes its keys with it.",
    },
    Signature {
        name: "on_keydown",
        shape: "fn(&str) -> bool",
        what: "Handles one key, returning whether it moved focus. Up/Down or Left/Right by orientation, plus Home and End. The caller decides what to do with the keys it leaves alone, and should prevent_default when it returns true.",
    },
    Signature {
        name: "on_focus_in",
        shape: "fn()",
        what: "Puts the tab stop on the item focus landed on, so tabbing away and back returns to it rather than to the first.",
    },
    Signature {
        name: "sync_tab_stop",
        shape: "fn()",
        what: "Gives the group its initial tab stop — the selected item if there is one, else the first. Without it every item is tabindex=-1 and the group cannot be tabbed into at all.",
    },
    Signature {
        name: "focus_first / focus_last",
        shape: "fn()",
        what: "Jump to either end. What Home and End do, exposed for a caller who wants them on another key.",
    },
    Signature {
        name: "focus_offset",
        shape: "fn(isize)",
        what: "Move by n items, wrapping unless the group was built without_wrap. A tree uses it for Right and Left, which are steps through the DOM once a closed branch renders nothing.",
    },
    Signature {
        name: "ROVING_ITEM",
        shape: "&str = \"data-roving-item\"",
        what: "The marker each item wears. Items are found by querying for it rather than kept in a registry, so they can appear and vanish without telling the group.",
    },
];

const SKIPS: &[Signature] = &[
    Signature {
        name: "disabled",
        shape: "attribute",
        what: "A disabled item is skipped by the arrows.",
    },
    Signature {
        name: "data-disabled",
        shape: "attribute",
        what: "The same, for anything that is not a real control — a div with role=\"menuitem\" cannot be disabled, so it says so this way.",
    },
    Signature {
        name: "aria-disabled=\"true\"",
        shape: "attribute",
        what: "And the same again, for a part that announces it rather than wearing it.",
    },
];

fn page() -> AnyView {
    let list = AnyNodeRef::new();
    let roving = use_roving_focus(list, Orientation::Vertical);

    Effect::new(move |_| {
        if list.get().is_some() {
            roving.sync_tab_stop();
        }
    });

    view! {
        <div class="flex flex-col gap-8">
            <p class="text-sm leading-relaxed text-muted-foreground">
                "A list of buttons is a list of tab stops, which is the wrong shape: a reader
                tabbing past a menu should pass it, not walk it. This makes the group one stop and
                gives the arrows the walking — the pattern ARIA asks for on menus, tabs, radios and
                trees alike, which is why ten components share this one implementation."
            </p>

            <DemoSection
                title="One tab stop"
                description="Tab into the list, then use the arrows. Home and End jump to the ends, the third item is disabled and is skipped, and tabbing out and back returns to wherever you left off rather than to the top."
                code=USE
            >
                <div class="flex w-full flex-col gap-3">
                    <button class="rounded-md border px-2.5 py-1.5 text-sm">"Before"</button>
                    <div
                        node_ref=list
                        on:keydown=move |e| {
                            if roving.on_keydown(&e.key()) {
                                e.prevent_default();
                            }
                        }
                        on:focusin=move |_| roving.on_focus_in()
                        class="flex w-56 flex-col gap-1 rounded-lg border p-1"
                    >
                        {["Overview", "Documents", "Archived", "Members"]
                            .into_iter()
                            .enumerate()
                            .map(|(i, label)| {
                                let disabled = i == 2;
                                view! {
                                    <button
                                        tabindex="-1"
                                        data-roving-item=""
                                        disabled=disabled
                                        class="rounded-md px-2.5 py-1.5 text-left text-sm outline-none hover:bg-accent focus-visible:bg-accent focus-visible:ring-2 focus-visible:ring-ring disabled:opacity-40"
                                    >
                                        {label}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                    <button class="rounded-md border px-2.5 py-1.5 text-sm">"After"</button>
                </div>
            </DemoSection>

            <div class="flex flex-col gap-6 border-t border-border/60 pt-8">
                <h2 class="text-lg font-semibold tracking-tight text-foreground">"API"</h2>
                <Signatures title="RovingFocus" rows=API />
                <Signatures
                    title="What it skips"
                    note="Any of these marks an item the arrows step over. Typeahead reads the same three."
                    rows=SKIPS
                />
            </div>
        </div>
    }
    .into_any()
}
