//! `Typeahead`.

use super::{Primitive, Signature, Signatures};
use crate::components::demo_section::DemoSection;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use nocomponents::{
    primitives::{roving_focus::use_roving_focus, typeahead::Typeahead},
    utils::types::Orientation,
};

pub const PRIMITIVE: Primitive = Primitive {
    slug: "typeahead",
    name: "Typeahead",
    blurb: "Type to jump. Keystrokes accumulate for a second so \"ba\" finds Banana rather than \
            stopping at Apple, and a repeated letter cycles instead, as native lists do.",
    used_by: &["context-menu", "dropdown-menu", "menubar", "select"],
    view: page,
};

const USE: &str = r#"use nocomponents::primitives::typeahead::Typeahead;

let typeahead = Typeahead::new();

// In the container's keydown, beside the arrows. It returns whether the key
// was typing — anything that is not a single printable character is not, so
// the arrows and Escape fall through to whatever handles them.
on:keydown=move |e| {
    if roving.on_keydown(&e.key()) || typeahead.push(&e.key(), list) {
        e.prevent_default();
    }
}"#;

const API: &[Signature] = &[
    Signature {
        name: "Typeahead::new",
        shape: "fn() -> Typeahead",
        what: "A search that resets itself after a second of no typing. Copy, so it can be handed to as many handlers as needed.",
    },
    Signature {
        name: "push",
        shape: "fn(&str, AnyNodeRef) -> bool",
        what: "Takes one key and the container to search inside, and says whether the key was typing. False for anything that is not a single printable character, so arrows and Escape pass straight through.",
    },
];

const RULES: &[Signature] = &[
    Signature {
        name: "accumulating",
        shape: "1s",
        what: "\"b\" then \"a\" searches for \"ba\" and finds Banana — a search that reset on every key would stop at Apple and never reach it.",
    },
    Signature {
        name: "a repeated letter",
        shape: "cycles",
        what: "\"a\", \"a\", \"a\" steps through everything beginning with a rather than searching for \"aaa\", which is what every native list does.",
    },
    Signature {
        name: "the items",
        shape: "data-roving-item",
        what: "The same marker roving focus uses, and disabled ones are skipped the same way — so the two agree about what the list is without being told twice.",
    },
];

fn page() -> AnyView {
    let list = AnyNodeRef::new();
    let roving = use_roving_focus(list, Orientation::Vertical);
    let typeahead = Typeahead::new();

    Effect::new(move |_| {
        if list.get().is_some() {
            roving.sync_tab_stop();
        }
    });

    view! {
        <div class="flex flex-col gap-8">
            <p class="text-sm leading-relaxed text-muted-foreground">
                "The thing every native select does and no div ever does by itself. It is a
                separate primitive from the arrows because the two answer different questions about
                the same list, and a menu wants both — which is why "
                <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">"push"</code>
                " returns whether it took the key, so the arrows can have it if not."
            </p>

            <DemoSection
                title="Type to jump"
                description="Tab in, then type. \"ba\" reaches Banana rather than stopping at Apple; pressing \"a\" repeatedly cycles through the three that begin with it; and the arrows still work, because typeahead hands back the keys that were not typing."
                code=USE
            >
                <div
                    node_ref=list
                    on:keydown=move |e| {
                        if roving.on_keydown(&e.key()) || typeahead.push(&e.key(), list) {
                            e.prevent_default();
                        }
                    }
                    on:focusin=move |_| roving.on_focus_in()
                    class="flex w-56 flex-col gap-1 rounded-lg border p-1"
                >
                    {["Apple", "Apricot", "Avocado", "Banana", "Blackberry", "Cherry"]
                        .into_iter()
                        .map(|label| {
                            view! {
                                <button
                                    tabindex="-1"
                                    data-roving-item=""
                                    class="rounded-md px-2.5 py-1.5 text-left text-sm outline-none hover:bg-accent focus-visible:bg-accent focus-visible:ring-2 focus-visible:ring-ring"
                                >
                                    {label}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </DemoSection>

            <div class="flex flex-col gap-6 border-t border-border/60 pt-8">
                <h2 class="text-lg font-semibold tracking-tight text-foreground">"API"</h2>
                <Signatures title="Typeahead" rows=API />
                <Signatures
                    title="What it decides"
                    note="The three behaviours that make it feel like a native list."
                    rows=RULES
                />
            </div>
        </div>
    }
    .into_any()
}
