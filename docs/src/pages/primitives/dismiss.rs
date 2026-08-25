//! `use_dismissable_layer`.

use super::{Primitive, Signature, Signatures};
use leptos::prelude::*;

pub const PRIMITIVE: Primitive = Primitive {
    slug: "dismiss",
    name: "use_dismissable_layer",
    blurb: "One stack of layers, ordered by when they opened. Escape closes the topmost, and an \
            outside press walks down until it finds a layer that contains it.",
    used_by: &["dialog", "floating", "navigation-menu"],
    view: page,
};

const USE: &str = r#"use nocomponents::primitives::dismiss::{use_dismissable_layer, LayerBounds};

// Called once, in the layer's own root.
use_dismissable_layer(
    is_open,
    // What counts as "inside": a press landing in either is not an outside
    // press. A floating layer names its trigger as well as its content, or
    // pressing the trigger to close would close it and reopen it at once.
    LayerBounds::new(content_ref).and(trigger_ref),
    move || is_open.set(false),
);"#;

const API: &[Signature] = &[
    Signature {
        name: "use_dismissable_layer",
        shape: "fn(RwSignal<bool>, LayerBounds, impl Fn())",
        what: "Registers a layer for as long as its signal is true, and runs the closure when it should close. Called in the layer's root, once.",
    },
    Signature {
        name: "LayerBounds::new",
        shape: "fn(AnyNodeRef) -> LayerBounds",
        what: "The element a press must land inside to count as inside.",
    },
    Signature {
        name: "LayerBounds::modal",
        shape: "fn(self) -> LayerBounds",
        what: "A modal layer ignores outside presses entirely — it is dismissed by Escape, or by something it owns. Nothing beneath it is reachable to be pressed.",
    },
    Signature {
        name: "dismiss_topmost",
        shape: "fn() -> bool",
        what: "Closes the layer on top and says whether there was one. What Escape is wired to, and what a caller with its own Escape handling should call rather than reimplement.",
    },
];

const RULES: &[Signature] = &[
    Signature {
        name: "Escape",
        shape: "topmost only",
        what: "A dialog with a select open inside it: Escape closes the select, then the dialog. One key, one layer at a time, in the order they were opened.",
    },
    Signature {
        name: "outside pointerdown",
        shape: "walks down",
        what: "From the top, until a layer contains the press or a modal one stops the walk. So pressing inside a dialog does not close the dialog, and pressing its overlay does.",
    },
    Signature {
        name: "pointerdown, not click",
        shape: "the event",
        what: "A layer that closed on click would still be open while the press was held, and would hand the release to whatever it was covering.",
    },
];

fn page() -> AnyView {
    view! {
        <div class="flex flex-col gap-8">
            <p class="text-sm leading-relaxed text-muted-foreground">
                "Every dialog, popover, menu and drawer closes on Escape and on a press outside it.
                Written per component that is four listeners and a guess about which one should
                win; written once it is a stack, and the answer to \"which closes first\" falls out
                of the order they opened in."
            </p>

            <div class="rounded-lg border bg-muted/40 p-4">
                <p class="text-xs leading-relaxed text-muted-foreground">
                    <span class="font-medium text-foreground">"No demo on this page."</span>
                    " Every floating component on this site is one — open a dialog, open a select
                    inside it, and press Escape twice. That is this primitive, and a fake stack
                    built to be looked at would only be a worse version of the real one."
                </p>
            </div>

            <div class="flex flex-col gap-6">
                <h2 class="text-lg font-semibold tracking-tight text-foreground">"Using it"</h2>
                <p class="text-sm leading-relaxed text-muted-foreground">
                    "A layer registers itself and says what counts as inside it. Nothing else is
                    required — the listeners are installed once for the page, not once per layer."
                </p>
                <nocomponents::components::code::CodeBlock code=USE />
            </div>

            <div class="flex flex-col gap-6 border-t border-border/60 pt-8">
                <h2 class="text-lg font-semibold tracking-tight text-foreground">"API"</h2>
                <Signatures title="dismiss" rows=API />
                <Signatures
                    title="The rules"
                    note="What the stack decides, and why each is the way it is."
                    rows=RULES
                />
            </div>
        </div>
    }
    .into_any()
}
