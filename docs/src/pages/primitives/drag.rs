//! `use_drag`.

use super::{Primitive, Signature, Signatures};
use crate::components::demo_section::DemoSection;
use leptos::prelude::*;
use nocomponents::primitives::drag::use_drag;

pub const PRIMITIVE: Primitive = Primitive {
    slug: "drag",
    name: "use_drag",
    blurb: "One pointer gesture, owned in one place: the listeners, their lifetime, and the \
            velocity and stillness on release that tell a flick from a placement.",
    used_by: &[
        "color-picker",
        "floating-menu",
        "image-cropper",
        "kanban",
        "resizable",
        "scroll-area",
        "slider",
    ],
    view: page,
};

const USE: &str = r#"use nocomponents::primitives::drag::use_drag;

let drag = use_drag(move |point| {
    // Every move until the pointer is released.
    position.set((point.dx, point.dy));
})
// The box the fractions are measured against, when the press lands on a
// wrapper rather than on the surface itself.
.against(track_ref)
.on_start(move |point| { /* a click and a drag begin the same way */ })
.on_end(move |end| {
    // `velocity` and `idle` are what separate a flick from a placement.
    if end.velocity_x.abs() > 0.5 && end.idle < 80.0 { dismiss(); }
});

// Attach it yourself, and guard your own press.
view! {
    <div on:pointerdown=move |e| {
        if disabled.get_untracked() { return; }
        drag.start(&e);
    } />
}"#;

const HANDLE: &[Signature] = &[
    Signature {
        name: "use_drag",
        shape: "fn(impl Fn(DragPoint)) -> Drag",
        what: "Takes the move handler and gives back the gesture. Nothing is listening yet.",
    },
    Signature {
        name: "start",
        shape: "fn(&PointerEvent)",
        what: "Begins a gesture from your own on:pointerdown. Guard the press yourself — a slider ignores one while disabled, a drawer ignores one that landed in a scroller, and a guard belongs with the thing it is guarding.",
    },
    Signature {
        name: "stop",
        shape: "fn()",
        what: "Ends it early. A fresh start does this first, so a gesture cannot stack a second set of listeners on the first.",
    },
    Signature {
        name: "against",
        shape: "fn(AnyNodeRef) -> Drag",
        what: "Names the box fraction_x and fraction_y are measured in. Without it that is whatever element the press landed on.",
    },
    Signature {
        name: "on_start",
        shape: "fn(impl Fn(DragPoint)) -> Drag",
        what: "Runs on the press. Where a click and a drag become one gesture: the press does the thing, and the moves adjust it.",
    },
    Signature {
        name: "on_end",
        shape: "fn(impl Fn(DragEnd)) -> Drag",
        what: "Runs on release — and on pointercancel, which is the one every hand-rolled version misses.",
    },
    Signature {
        name: "active",
        shape: "RwSignal<bool>",
        what: "Whether a gesture is running. Mark the element with it rather than keeping a second flag beside it.",
    },
];

const POINT: &[Signature] = &[
    Signature {
        name: "client_x / client_y",
        shape: "f64",
        what: "Where the pointer is, in viewport coordinates.",
    },
    Signature {
        name: "dx / dy",
        shape: "f64",
        what: "How far it has come since the press. What a gesture that moves something by an amount wants.",
    },
    Signature {
        name: "fraction_x / fraction_y",
        shape: "f64",
        what: "Where it sits inside the reference box, 0 at one edge and 1 at the other. Unclamped, so an overshoot can be told from an edge, and measured live, so a page that scrolls mid-drag does not shift the frame under the gesture.",
    },
    Signature {
        name: "shift / alt",
        shape: "bool",
        what: "The modifiers held for this move, not for the press — so taking or releasing a key partway through changes what the rest of the drag means without you listening for keys.",
    },
];

const END: &[Signature] = &[
    Signature {
        name: "point",
        shape: "DragPoint",
        what: "Where it ended.",
    },
    Signature {
        name: "velocity_x / velocity_y",
        shape: "f64",
        what: "Pixels per millisecond over the last movement, signed. Zero if the pointer never moved.",
    },
    Signature {
        name: "idle",
        shape: "f64",
        what: "Milliseconds since the pointer last moved. A pause before release is a placement rather than a throw, which is the difference between dropping a panel and flicking it away.",
    },
];

fn page() -> AnyView {
    let offset = RwSignal::new((0.0f64, 0.0f64));
    let report = RwSignal::new(String::from("Press the square and move it."));

    let drag = use_drag(move |point| offset.set((point.dx, point.dy))).on_end(move |end| {
        offset.set((0.0, 0.0));
        report.set(format!(
            "released at {:.0}, {:.0} · velocity {:.2} px/ms · still for {:.0}ms — {}",
            end.point.dx,
            end.point.dy,
            end.velocity_x,
            end.idle,
            if end.velocity_x.abs() > 0.4 && end.idle < 100.0 {
                "a flick"
            } else {
                "a placement"
            },
        ));
    });

    view! {
        <div class="flex flex-col gap-8">
            <p class="text-sm leading-relaxed text-muted-foreground">
                "Every pointer gesture in the library goes through this one, and none of them own a
                window listener. It owns the lifetime — including "
                <code class="rounded bg-muted px-1 py-0.5 text-xs text-foreground">
                    "pointercancel"
                </code>
                ", which every hand-rolled copy missed, and which is what leaves a card stuck to
                the cursor when the browser takes the pointer away."
            </p>

            <DemoSection
                title="A gesture"
                description="The square follows the pointer and springs back on release. What is printed underneath is what on_end is handed — the velocity and the stillness, which is how a flick is told from a placement."
                code=USE
            >
                <div class="flex w-full flex-col gap-3">
                    <div class="flex h-40 items-center justify-center rounded-lg border border-dashed">
                        <div
                            on:pointerdown=move |e| drag.start(&e)
                            style=move || {
                                let (x, y) = offset.get();
                                format!("transform: translate3d({x}px, {y}px, 0)")
                            }
                            class="size-16 cursor-grab touch-none rounded-md bg-primary/80 select-none data-[dragging=true]:cursor-grabbing"
                            data-dragging=move || drag.active.get().then_some("true")
                        />
                    </div>
                    <p class="font-mono text-xs text-muted-foreground">{report}</p>
                </div>
            </DemoSection>

            <div class="flex flex-col gap-6 border-t border-border/60 pt-8">
                <h2 class="text-lg font-semibold tracking-tight text-foreground">"API"</h2>
                <Signatures title="Drag" rows=HANDLE />
                <Signatures title="DragPoint" note="What every move is handed." rows=POINT />
                <Signatures
                    title="DragEnd"
                    note="What the release is handed, on top of the point."
                    rows=END
                />
            </div>
        </div>
    }
    .into_any()
}
