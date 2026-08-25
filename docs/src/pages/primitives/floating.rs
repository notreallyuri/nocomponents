//! `floating`.

use super::{Primitive, Signature, Signatures};
use leptos::prelude::*;

pub const PRIMITIVE: Primitive = Primitive {
    slug: "floating",
    name: "floating",
    blurb: "The shared machinery under every layer that hangs off a trigger: the open state, the \
            ARIA written onto whatever shape the trigger is, and the delay that lets it animate out.",
    used_by: &[
        "combobox",
        "context-menu",
        "date-picker",
        "dropdown-menu",
        "floating-menu",
        "hover-card",
        "menubar",
        "popover",
        "select",
        "tooltip",
    ],
    view: page,
};

const USE: &str = r#"// A component aliases the shared context rather than defining its own —
// `pub type PopoverContext = FloatingContext;` — and reuses the parts.
use nocomponents::primitives::floating::{FloatingRoot, FloatingTrigger, TriggerAria};

<FloatingRoot trigger_aria=TriggerAria::Popup("dialog")>
    <FloatingTrigger
        context=ctx
        data_slot="popover-trigger"
        render=as_child
    >
        "Open"
    </FloatingTrigger>
    {children()}
</FloatingRoot>"#;

const CONTEXT: &[Signature] = &[
    Signature {
        name: "is_open",
        shape: "RwSignal<bool>",
        what: "Whether the layer is open.",
    },
    Signature {
        name: "is_mounted",
        shape: "RwSignal<bool>",
        what: "Stays true for unmount_delay (150ms) after closing, so the exit animation can run. Never key content on is_open alone or it vanishes mid-animation.",
    },
    Signature {
        name: "trigger_ref / content_ref",
        shape: "AnyNodeRef",
        what: "The two elements. The trigger comes in three shapes — a bare button, a styled Button, anything behind render — and the node ref is all they share, which is why the ARIA is written onto it rather than into markup.",
    },
    Signature {
        name: "trigger_id / content_id",
        shape: "RwSignal<String>",
        what: "Minted here so aria-controls and aria-labelledby have something to point at. An id the caller wrote on the trigger is adopted instead, so a <label for> still names it.",
    },
    Signature {
        name: "open / close / toggle",
        shape: "fn(&self)",
        what: "The transitions. Closing sets is_open at once and is_mounted after the delay.",
    },
];

const PARTS: &[Signature] = &[
    Signature {
        name: "FloatingRoot",
        shape: "component",
        what: "Provides the context and writes the trigger's ARIA from an effect. Takes trigger_aria, and the unmount delay.",
    },
    Signature {
        name: "FloatingTrigger",
        shape: "component",
        what: "The press that opens it. Renders its own <button>, or hands a TriggerRender to a caller's render — the node ref, the click, and the disabled already resolved against any Field around it.",
    },
    Signature {
        name: "FloatingProvider",
        shape: "component",
        what: "The context alone, for a component that builds its own root — a submenu whose panel is positioned differently from its parent's.",
    },
];

const ARIA: &[Signature] = &[
    Signature {
        name: "TriggerAria::Popup(role)",
        shape: "\"menu\" | \"listbox\" | \"dialog\"",
        what: "aria-haspopup with that value, aria-expanded always, and aria-controls only while open — it may only point at an element that exists, and the content is not in the DOM until then.",
    },
    Signature {
        name: "TriggerAria::Describes",
        shape: "—",
        what: "For a layer that describes its trigger rather than being opened by it: aria-describedby while open, and no aria-expanded, which would be a lie. What a tooltip wants.",
    },
    Signature {
        name: "TriggerAria::None",
        shape: "—",
        what: "Nothing announced, for content that is decorative or announced some other way.",
    },
];

fn page() -> AnyView {
    view! {
        <div class="flex flex-col gap-8">
            <p class="text-sm leading-relaxed text-muted-foreground">
                "Ten components hang a layer off a trigger, and they differ only in what the layer
                is. This is everything they share: the state, the ids, the ARIA, and the delay
                between closing and unmounting. Positioning goes through "
                <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                    "floating-ui-leptos"
                </code> " on top of it, with "
                <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                    "utils::get_placement"
                </code> " mapping a "
                <code class="rounded bg-muted px-1 py-0.5 text-xs">"Side"</code> " and an "
                <code class="rounded bg-muted px-1 py-0.5 text-xs">"Align"</code>
                " onto one of the twelve placements."
            </p>

            <div class="rounded-lg border bg-muted/40 p-4">
                <p class="text-xs leading-relaxed text-muted-foreground">
                    <span class="font-medium text-foreground">
                        "This one is not used directly."
                    </span>
                    " A component aliases the context — "
                    <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                        "pub type PopoverContext = FloatingContext;"
                    </code>
                    " — and reuses the parts, so what a reader of the popover sees is a popover. It
                    is here because it is where the ARIA and the timing actually live, and because
                    a bug in any of the ten is usually a bug in this."
                </p>
            </div>

            <div class="flex flex-col gap-4">
                <h2 class="text-lg font-semibold tracking-tight text-foreground">"Using it"</h2>
                <nocomponents::components::code::CodeBlock code=USE />
            </div>

            <div class="flex flex-col gap-6 border-t border-border/60 pt-8">
                <h2 class="text-lg font-semibold tracking-tight text-foreground">"API"</h2>
                <Signatures
                    title="FloatingContext"
                    note="What use_popover(), use_select() and the rest all resolve to."
                    rows=CONTEXT
                />
                <Signatures title="The parts" rows=PARTS />
                <Signatures
                    title="TriggerAria"
                    note="What the trigger announces. Written onto the node from an effect, since the trigger is a different element in each of the three shapes."
                    rows=ARIA
                />
            </div>
        </div>
    }
    .into_any()
}
