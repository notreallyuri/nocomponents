use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};
use leptos::{either::Either, prelude::*};
use nocomponents::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        canvas::{
            Canvas, CanvasMarquee, CanvasNodeLabel, CanvasShape, CanvasSticky, CanvasText,
            ShapeKind, StickyColor,
        },
        kbd::Kbd,
    },
    primitives::canvas::{CanvasDrag, CanvasGesture, CanvasPoint},
    utils::viewport::{Viewport, WorldBox},
};

const DEFAULT: &str = r#"// The canvas owns where you are looking. What is on it is yours.
// A press is a click: hold space and drag to pan, or use the middle button.
<Canvas class="h-96">
    <CanvasSticky x=40.0 y=40.0 color=StickyColor::Yellow>
        "Hold space and drag to pan."
    </CanvasSticky>
    <CanvasShape x=260.0 y=60.0 kind=ShapeKind::Ellipse>"An ellipse"</CanvasShape>
    <CanvasText x=40.0 y=240.0>"Text sits on the board too."</CanvasText>
</Canvas>"#;

const CLICK: &str = r#"// Where a click landed, in world coordinates — which is the part a caller
// cannot work out, since it needs the surface's box and the transform.
// It does not fire for a click on a node, or for the click that ends a pan.
let notes = RwSignal::new(Vec::<(f64, f64)>::new());

view! {
    <Canvas
        on_surface_click=Callback::new(move |at: CanvasPoint| {
            notes.update(|notes| notes.push((at.x, at.y)));
        })
        class="h-80"
    >
        <For each=move || notes.get() key=|at| … let:at>
            <CanvasSticky x=at.0 y=at.1>"New"</CanvasSticky>
        </For>
    </Canvas>
}"#;

/// The board the marquee demo selects over: world x, y and size.
const BAND_NOTES: &[(f64, f64, f64)] = &[
    (40.0, 40.0, 120.0),
    (210.0, 70.0, 100.0),
    (370.0, 40.0, 140.0),
    (60.0, 210.0, 110.0),
    (250.0, 230.0, 130.0),
    (440.0, 220.0, 100.0),
];

const MARQUEE: &str = r#"// The press, the live rectangle and the release — which is everything a
// click cannot give. Reported on every move and once more when the pointer
// comes up, and `done` is which one you are reading.
let band = RwSignal::new(None::<WorldBox>);
let selected = RwSignal::new(Vec::<usize>::new());

view! {
    <Canvas
        on_surface_drag=Callback::new(move |drag: CanvasDrag| {
            // `rect` is sorted, so a drag up and to the left is as ordinary
            // as one down and to the right. `intersects` selects what the
            // band touches rather than what it swallows whole — a band that
            // had to contain a node could never reach one larger than the
            // screen.
            selected.set(
                notes.iter().enumerate()
                    .filter(|(_, note)| drag.rect.intersects(note))
                    .map(|(i, _)| i)
                    .collect(),
            );
            // The band is the gesture; the selection is what it left behind.
            band.set((!drag.done).then_some(drag.rect));
        })
        // The two share one press and never both fire: a gesture that stayed
        // within a few pixels was a click, and one that did not was a drag.
        on_surface_click=Callback::new(move |_| selected.set(Vec::new()))
        class="h-96"
    >
        <For each=… key=… let:note>
            <CanvasSticky … selected=… />
        </For>
        // Inside the transformed layer, so it stays over the world it was
        // drawn around rather than over the screen it was drawn on.
        <CanvasMarquee rect=band />
    </Canvas>
}"#;

const NODES: &str = r#"// A node reports its own gestures and the caller decides what they mean —
// the same stance the kanban board takes with `on_move`. What the node owns
// is the arithmetic: a pointer travels in screen pixels and a node lives in
// world units, so every delta is divided by the zoom, and the zoom is in the
// canvas' context, which only something inside the canvas can reach.
let box_ = RwSignal::new(WorldBox::new(40.0, 40.0, 180.0, 120.0));
let label = RwSignal::new("Drag me, or my grips".to_string());

view! {
    <CanvasShape
        x=Signal::derive(move || box_.get().x)
        y=Signal::derive(move || box_.get().y)
        width=Signal::derive(move || box_.get().width)
        height=Signal::derive(move || box_.get().height)
        on_move=Callback::new(move |at: CanvasPoint| {
            box_.update(|box_| (box_.x, box_.y) = (at.x, at.y));
        })
        on_resize=Callback::new(move |next: WorldBox| box_.set(next))
        // Once, when the gesture is over. The two above are the fortieth
        // intermediate frame; this is the one worth telling a peer about,
        // and it carries where the drag began — which the caller cannot
        // keep for itself, having overwritten it on every move since.
        // Silent for a press that changed nothing, so selecting is not an
        // edit and a click does not put an operation on the wire.
        on_gesture_end=Callback::new(move |gesture: CanvasGesture| {
            log.update(|log| log.push(gesture));
        })
        // The grips are drawn when this is true; eight on every node is
        // unreadable, and `data-selected` is what the styled layer keys on.
        selected=true
    >
        // Double-click to rename. Not a click and not a press: a node is
        // dragged with the same button, so either would fire a rename on
        // every drag that began on the words. The input inherits the node's
        // own type — a browser gives one none of that by default, and an
        // editor that changes font halfway through is not one.
        <CanvasNodeLabel value=label on_change=Callback::new(move |next| label.set(next)) />
    </CanvasShape>
}"#;

const HAND: &str = r#"// A board that is only ever looked at wants the hand tool back.
<Canvas pan_on_drag=true class="h-72" />"#;

const CONTROLLED: &str = r#"// The viewport is a plain signal, as useful for reading where the reader
// has gone as for putting them somewhere.
let viewport = RwSignal::new(Viewport::default());

view! { <Canvas viewport=viewport class="h-80" /> }"#;

const GESTURES: &str = r#"// Wheel pans, ctrl+wheel zooms — which is also what a trackpad pinch
// arrives as, there being no pinch event on the desktop.
// Space+drag and the middle button pan; the primary button is a click,
// and `pan_on_drag=true` gives it back to the hand tool.
// ⌘0 resets, ⌘+ and ⌘- step the zoom."#;

const API: &[ApiEntry] = &[
    ApiEntry {
        name: "Canvas",
        description: "An infinite surface you can pan and zoom, with a dotted background, a transformed content layer and the zoom controls. The canvas owns where you are looking; what is on it belongs to the caller, the same stance as the kanban board.",
        props: &[
            Prop {
                name: "viewport",
                ty: "RwSignal<Viewport>",
                default: "Viewport::default()",
                description: "Where the surface is looking. A plain signal, as useful for reading where the reader has gone as for driving them somewhere.",
            },
            Prop {
                name: "limits",
                ty: "ZoomLimits",
                default: "0.1 – 4.0",
                description: "How far in and out the surface may go. A canvas that can reach zero is a canvas you cannot get back.",
            },
            Prop {
                name: "pan_on_drag",
                ty: "bool",
                default: "false",
                description: "Whether dragging the surface with the primary button pans it — the hand tool, always on. Off, because a press is the one gesture a caller cannot get back, and a board worth the name has something to click on. Space+drag, the middle button and two fingers pan either way.",
            },
            Prop {
                name: "on_surface_click",
                ty: "Option<Callback<CanvasPoint>>",
                default: "None",
                description: "Run for a click that landed on the board rather than on anything placed on it, with the world coordinate — which is what a caller cannot work out for itself. Silent for a click on a node, and for the click that ends a pan.",
            },
            Prop {
                name: "on_surface_drag",
                ty: "Option<Callback<CanvasDrag>>",
                default: "None",
                description: "Run while the primary button is dragged across the board, and once more when it is let go — the press, the live rectangle and the release, which is what a marquee, a lasso and a shape pulled open all need and a click cannot give. It shares one press with `on_surface_click`: a gesture that stayed within a few pixels was a click, one that did not was this, and the two never both fire. Silent while the primary button is the hand tool.",
            },
            Prop {
                name: "controls",
                ty: "bool",
                default: "true",
                description: "Whether to draw the zoom controls in the corner.",
            },
            Prop {
                name: "node_ref",
                ty: "Option<AnyNodeRef>",
                default: "None",
                description: "The surface itself, for something outside the canvas that has to measure it — a palette that drops a node where you let go cannot ask the context, not being inside the provider. Pass it here and hand it to `world_at`, rather than wrapping the canvas in a div and measuring that: the two are the same box only until somebody puts a margin or a second child on the wrapper.",
            },
            Prop {
                name: "gap",
                ty: "f64",
                default: "24.0",
                description: "World units between background dots.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the surface's classes. It has no height of its own — give it one.",
            },
            Prop {
                name: "children",
                ty: "Children",
                default: "",
                description: "What sits on the board, in world coordinates.",
            },
        ],
    },
    ApiEntry {
        name: "CanvasNode",
        description: "One thing on the board, placed in world coordinates. Position is a prop rather than state here, so a node moved by anything other than the reader still lands where the caller says it does — what the node owns is the gesture, and it reports where a drag would put it. The grips are sized in screen pixels and divided by the zoom before they are drawn: left to scale, a 10px square is 1px at 0.1× and covers its own node at 4×.",
        props: &[
            Prop {
                name: "x",
                ty: "Signal<f64>",
                default: "",
                description: "World x.",
            },
            Prop {
                name: "y",
                ty: "Signal<f64>",
                default: "",
                description: "World y.",
            },
            Prop {
                name: "width",
                ty: "Option<Signal<f64>>",
                default: "None",
                description: "World width. Unset, the node is sized by its content — which is what a label wants.",
            },
            Prop {
                name: "height",
                ty: "Option<Signal<f64>>",
                default: "None",
                description: "World height.",
            },
            Prop {
                name: "on_move",
                ty: "Option<Callback<CanvasPoint>>",
                default: "None",
                description: "Where a drag on the node itself would put it, in world coordinates. Unset, the node does not move. The node owns the arithmetic — a pointer travels in screen pixels and a node lives in world units, so the delta is divided by the zoom — and reports rather than applies, the same stance the kanban board takes.",
            },
            Prop {
                name: "on_resize",
                ty: "Option<Callback<WorldBox>>",
                default: "None",
                description: "What a drag on one of the eight grips would make of it. No grips are drawn without this, or without a `width` and `height` to write back to. Measured from the press rather than from the last move, so a drag past the minimum and back does not lag the pointer.",
            },
            Prop {
                name: "on_gesture_end",
                ty: "Option<Callback<CanvasGesture>>",
                default: "None",
                description: "Run once when a move or a resize is over, with the box it began from, the box it ended at, and the grip that did it — `None` for a move. One callback for both, because writing an operation a peer can replay is the same job either way. It carries the starting box because the caller cannot keep it: the live callbacks have been overwriting that position since the press, and an operation somebody can invert needs both ends. Silent for a press that changed nothing, so selecting a node is not an edit. Where it ended is read back out of the caller's own signals, so a caller that snapped the node to a grid reports what it did rather than what it was asked to do.",
            },
            Prop {
                name: "z",
                ty: "Signal<i32>",
                default: "0",
                description: "Where the node sits in the stack. Depth is DOM order without it, so \"bring to front\" is a move within the caller's list — which changes the keys and rebuilds the markup, fine at four nodes and wrong at four hundred. Written into `z-index` only when it is not zero, so an ordinary node stays out of a stacking order it has no opinion about.",
            },
            Prop {
                name: "aspect",
                ty: "Option<f64>",
                default: "None",
                description: "Width ÷ height, kept through a resize. With one, only the four corners are drawn: a side cannot honour a ratio without moving an edge the pointer is not on.",
            },
            Prop {
                name: "angle",
                ty: "Signal<f64>",
                default: "0.0",
                description: "Rotation about the node's own centre, in radians. Rendered, carried into every `CanvasGesture` and saved with the rest of the box — but no gesture turns one yet, so it comes from the caller's own data or from nothing. A node with an angle draws no grips: `WorldBox::resized` takes its delta in world axes, which stop being the box's axes once it is turned, so the grips would grow it the wrong way. Refusing to draw them beats drawing eight that lie.",
            },
            Prop {
                name: "min_size",
                ty: "f64",
                default: "24.0",
                description: "The smallest a resize may leave it, in world units. A drag past the far edge stops here rather than turning the box inside out.",
            },
            Prop {
                name: "selected",
                ty: "Signal<bool>",
                default: "false",
                description: "Whether this is the node the caller considers selected. Written as `data-selected`, which is what the outline and the grips are drawn by — eight grips on every node is unreadable.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the node's classes.",
            },
            Prop {
                name: "children",
                ty: "Option<Children>",
                default: "None",
                description: "Anything at all, or nothing: a frame drawn round a selection, a guide, a slot waiting to be filled is a box the caller wants placed and measured in world units and holding nothing.",
            },
        ],
    },
    ApiEntry {
        name: "CanvasSticky",
        description: "A square note in one of four colours — the shape a board like this is mostly made of.",
        props: &[
            Prop {
                name: "x / y",
                ty: "Signal<f64>",
                default: "",
                description: "World position.",
            },
            Prop {
                name: "size",
                ty: "Signal<f64>",
                default: "160.0",
                description: "World width and height; a sticky is square.",
            },
            Prop {
                name: "color",
                ty: "StickyColor",
                default: "Yellow",
                description: "`Yellow`, `Green`, `Blue` or `Pink`.",
            },
            Prop {
                name: "on_move / on_resize",
                ty: "Option<Callback<…>>",
                default: "None",
                description: "As on `CanvasNode`. The ratio is fixed at 1 and `min_size` is 60, since a sticky is square and a sticky you cannot read is not one.",
            },
            Prop {
                name: "selected",
                ty: "Signal<bool>",
                default: "false",
                description: "Draws the outline and the four corner grips.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the note's classes.",
            },
        ],
    },
    ApiEntry {
        name: "CanvasShape",
        description: "A rectangle or an ellipse, with optional content centred in it.",
        props: &[
            Prop {
                name: "x / y",
                ty: "Signal<f64>",
                default: "",
                description: "World position.",
            },
            Prop {
                name: "width / height",
                ty: "Signal<f64>",
                default: "160.0 / 100.0",
                description: "World size.",
            },
            Prop {
                name: "kind",
                ty: "ShapeKind",
                default: "Rectangle",
                description: "`Rectangle` or `Ellipse` — which is a border radius rather than a different element.",
            },
            Prop {
                name: "on_move / on_resize / aspect / min_size / selected",
                ty: "…",
                default: "None / 32.0 / false",
                description: "As on `CanvasNode`. Pass `aspect=1.0` for an ellipse that stays a circle.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the shape's classes.",
            },
        ],
    },
    ApiEntry {
        name: "CanvasText",
        description: "A label on the board with no chrome around it. Sized by its own content, since a caption has no width of its own.",
        props: &[
            Prop {
                name: "x / y",
                ty: "Signal<f64>",
                default: "",
                description: "World position.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the text's classes.",
            },
        ],
    },
    ApiEntry {
        name: "CanvasNodeLabel",
        description: "A node's label, which becomes a textarea when you double-click it. A textarea rather than an input because the text it stands in for wraps and an input does not — on a sticky note the words would reflow onto one line the moment you began editing and jump back when you stopped. It inherits the node's own font, colour and alignment, wears no background and no ring, and is sized by its content, so the only thing that changes when you start typing is that there is a caret. Editing text is not the canvas' business — a node's children are the caller's — but the seam between editing and the gestures a node already has is: the words let a press through so a note can still be dragged by them, and the textarea stops one, because that press belongs to the caret. The value stays the caller's. Enter breaks the line — the key that makes a new line in every other multi-line field should not be the one that closes this one — and everything else ends the edit keeping the text: Escape, ⌘/Ctrl+Enter, and clicking away. There is no cancel on purpose: the words under the caret are the note, so a key that silently threw them away would be a key nobody presses twice.",
        props: &[
            Prop {
                name: "value",
                ty: "Signal<String>",
                default: "",
                description: "What it shows, and what the input is seeded with.",
            },
            Prop {
                name: "on_change",
                ty: "Option<Callback<String>>",
                default: "None",
                description: "The new text, whenever the edit ends — Escape, ⌘/Ctrl+Enter, or clicking away. A plain Enter breaks the line instead. Silent when nothing changed. A key that ends the edit marks it settled, because a browser fires `blur` at an element it takes out of the document and the edit would otherwise be reported twice: harmless for a caller that applies what it is told, and not harmless at all for one that rejects it.",
            },
            Prop {
                name: "editing",
                ty: "RwSignal<bool>",
                default: "false",
                description: "Whether the input is showing. A signal rather than internal state, so a rename can be started from somewhere else — a menu item, a key on the board.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over both shapes; `data-editing` tells them apart. Anything typographic belongs here rather than on the node, since both wear it and that is what keeps them the same size.",
            },
        ],
    },
    ApiEntry {
        name: "CanvasMarquee",
        description: "The band a surface drag draws, placed in world coordinates. Nothing but a box — the arithmetic that produced it is `CanvasDrag::rect` and what it means is the caller's. It is a part rather than something the canvas draws for itself because a band is not always a selection: the same rectangle is a shape being pulled open, or a region being cropped. Goes inside the canvas, so it stays over the world it was drawn around rather than over the screen it was drawn on, and it never takes the pointer — a band under the cursor is what the release would land on, and the gesture would end on itself.",
        props: &[
            Prop {
                name: "rect",
                ty: "Signal<Option<WorldBox>>",
                default: "",
                description: "The band, in world units. `None` renders nothing, which is what a board with no gesture on it looks like.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the band's classes. Its border is `calc(1px / var(--canvas-zoom))`, so it stays one pixel however far in the reader is.",
            },
        ],
    },
    ApiEntry {
        name: "CanvasControls",
        description: "Zoom out, the current zoom as a percentage (press it to reset), zoom in, and fit. Fit reads the nodes out of the DOM rather than from a registry the caller keeps in step — nodes can come and go without telling anybody, and an empty board is left alone rather than fitted to nothing.",
        props: &[Prop {
            name: "class",
            ty: "Signal<String>",
            default: "\"\"",
            description: "Merged over the panel's classes.",
        }],
    },
    ApiEntry {
        name: "CanvasBackground",
        description: "The dotted paper. Outside the transformed layer on purpose: inside it the dots would scale with the zoom and blur out at 4×, so the spacing follows the zoom and the dots themselves do not.",
        props: &[
            Prop {
                name: "gap",
                ty: "f64",
                default: "24.0",
                description: "World units between dots.",
            },
            Prop {
                name: "class",
                ty: "Signal<String>",
                default: "\"\"",
                description: "Merged over the background's classes.",
            },
        ],
    },
];

#[component]
pub fn Page() -> impl IntoView {
    let viewport = RwSignal::new(Viewport::default());
    let placed = RwSignal::new(Vec::<(usize, CanvasPoint)>::new());
    let next = RwSignal::new(1usize);
    let shape = RwSignal::new(WorldBox::new(60.0, 60.0, 200.0, 130.0));
    let shape_label = RwSignal::new("Drag me, or my grips".to_string());
    let note = RwSignal::new(WorldBox::new(340.0, 90.0, 150.0, 150.0));
    let note_label = RwSignal::new("Double-click to rename".to_string());
    let picked = RwSignal::new(String::from("shape"));
    let tilted = RwSignal::new(WorldBox::new(120.0, 300.0, 0.0, 0.0).rotated(-0.28));
    let log = RwSignal::new(Vec::<CanvasGesture>::new());
    let band = RwSignal::new(None::<WorldBox>);
    let banded = RwSignal::new(Vec::<usize>::new());
    // One closure for both nodes and for both gestures: what a caller does with either is the
    // same, which is why there is one callback rather than an `on_move_end` and an `on_resize_end`
    // that would be handed the same body twice.
    let record = Callback::new(move |gesture: CanvasGesture| {
        log.update(|log| log.push(gesture));
    });

    view! {
        <DocLayout
            title="Canvas"
            description="An infinite surface you can pan and zoom, with things placed on it."
        >
            <div class="flex flex-col gap-8">
                <DemoSection
                    title="Default"
                    description="Hold space and drag to pan, ctrl+wheel or pinch to zoom. The primary button is left alone on purpose: it is the one gesture a caller cannot get back, and a board worth the name has something to click on. The canvas owns where you are looking and nothing else — the nodes take their world position as props, so the caller's list stays the one place a position is written down."
                    code=DEFAULT
                >
                    <Canvas class="h-96">
                        <CanvasSticky x=40.0 y=40.0 color=StickyColor::Yellow>
                            <span class="font-semibold">"Pan"</span>
                            <span>"Hold space and drag, or use the middle button."</span>
                        </CanvasSticky>
                        <CanvasSticky x=230.0 y=90.0 color=StickyColor::Green>
                            <span class="font-semibold">"Zoom"</span>
                            <span>"Ctrl and the wheel, or a pinch on a trackpad."</span>
                        </CanvasSticky>
                        <CanvasSticky x=420.0 y=40.0 color=StickyColor::Blue>
                            <span class="font-semibold">"Fit"</span>
                            <span>"Reads the nodes out of the DOM, so it needs no registry."</span>
                        </CanvasSticky>
                        <CanvasShape x=60.0 y=250.0 width=180.0 height=90.0>
                            "A rectangle"
                        </CanvasShape>
                        <CanvasShape
                            x=300.0
                            y=250.0
                            width=120.0
                            height=120.0
                            kind=ShapeKind::Ellipse
                        >
                            "An ellipse"
                        </CanvasShape>
                        <CanvasText x=470.0 y=290.0>
                            "…and text, sized by its own content."
                        </CanvasText>
                    </Canvas>
                </DemoSection>

                <DemoSection
                    title="A click on the board"
                    description="`on_surface_click` reports where a click landed in world coordinates — the part a caller cannot work out for itself, since it takes the surface's own box and the current transform. It stays quiet for a click on a node, which belongs to the node, and for the click that ends a pan, which was never a click on anywhere."
                    code=CLICK
                >
                    <div class="flex flex-col gap-3">
                        <Canvas
                            on_surface_click=Callback::new(move |at: CanvasPoint| {
                                placed.update(|placed| placed.push((next.get_untracked(), at)));
                                next.update(|next| *next += 1);
                            })
                            class="h-80"
                        >
                            {move || {
                                placed
                                    .get()
                                    .into_iter()
                                    .map(|(id, at)| {
                                        view! {
                                            <CanvasSticky
                                                x=at.x
                                                y=at.y
                                                size=110.0
                                                color=StickyColor::Green
                                                class="text-xs"
                                            >
                                                <span class="font-semibold">{format!("#{id}")}</span>
                                                <span class="font-mono tabular-nums">
                                                    {format!("{:.0}, {:.0}", at.x, at.y)}
                                                </span>
                                            </CanvasSticky>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </Canvas>
                        <div class="flex items-center gap-3 text-xs text-muted-foreground">
                            <span>
                                "Click the board to drop a note. " <Kbd>"Space"</Kbd>
                                " and drag to pan."
                            </span>
                            <Button
                                variant=ButtonVariant::Outline
                                size=ButtonSize::Sm
                                on:click=move |_| placed.set(Vec::new())
                            >
                                "Clear"
                            </Button>
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="A drag on the board"
                    description="`on_surface_drag` gives back the press, the live rectangle and the release, which is everything a click cannot: a marquee, a lasso and a shape pulled open all need the middle of the gesture and not only its end. It reports on every move and once more when the pointer comes up, and `done` is which one you are reading — here the band is drawn from the running report and the selection is what the last one leaves behind. It shares one press with `on_surface_click`, and the two never both fire: a gesture that stayed within a few pixels was a click, one that did not was a drag. Click the empty board to clear."
                    code=MARQUEE
                >
                    <div class="flex flex-col gap-3">
                        <Canvas
                            on_surface_drag=Callback::new(move |drag: CanvasDrag| {
                                banded
                                    .set(
                                        BAND_NOTES
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, (x, y, size))| {
                                                drag.rect.intersects(&WorldBox::new(*x, *y, *size, *size))
                                            })
                                            .map(|(index, _)| index)
                                            .collect(),
                                    );
                                band.set((!drag.done).then_some(drag.rect));
                            })
                            on_surface_click=Callback::new(move |_| banded.set(Vec::new()))
                            class="h-96"
                        >
                            {BAND_NOTES
                                .iter()
                                .enumerate()
                                .map(|(index, (x, y, size))| {
                                    view! {
                                        <CanvasSticky
                                            x=*x
                                            y=*y
                                            size=*size
                                            color=if index % 2 == 0 {
                                                StickyColor::Yellow
                                            } else {
                                                StickyColor::Blue
                                            }
                                            selected=Signal::derive(move || {
                                                banded.get().contains(&index)
                                            })
                                            class="text-xs"
                                        >
                                            <span class="font-semibold">
                                                {format!("Note {}", index + 1)}
                                            </span>
                                        </CanvasSticky>
                                    }
                                })
                                .collect_view()}
                            <CanvasMarquee rect=band />
                        </Canvas>
                        <p class="text-xs text-muted-foreground">
                            "Drag across the board to select. "
                            {move || {
                                let count = banded.get().len();
                                match count {
                                    0 => "Nothing selected.".to_string(),
                                    1 => "One note selected.".to_string(),
                                    _ => format!("{count} notes selected."),
                                }
                            }}
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="Moving, resizing, renaming"
                    description="A node reports its own gestures and the caller decides what they mean — the same stance the kanban board takes with `on_move`. What the node owns is the arithmetic nobody should write twice: a pointer travels in screen pixels and a node lives in world units, so every delta is divided by the zoom, and the zoom is in the canvas' context, which only something inside the canvas can reach. The grips are sized in screen pixels for the same reason the background dots are — everything else on a board is meant to scale, and a control is not."
                    code=NODES
                >
                    <div class="flex flex-col gap-3">
                        <Canvas class="h-96">
                            <CanvasShape
                                x=Signal::derive(move || shape.get().x)
                                y=Signal::derive(move || shape.get().y)
                                width=Signal::derive(move || shape.get().width)
                                height=Signal::derive(move || shape.get().height)
                                on_move=Callback::new(move |at: CanvasPoint| {
                                    shape.update(|shape| (shape.x, shape.y) = (at.x, at.y));
                                })
                                on_resize=Callback::new(move |next: WorldBox| shape.set(next))
                                on_gesture_end=record
                                selected=Signal::derive(move || picked.get() == "shape")
                                on:pointerdown=move |_| picked.set("shape".into())
                            >
                                <CanvasNodeLabel
                                    value=shape_label
                                    on_change=Callback::new(move |next| shape_label.set(next))
                                    class="text-center"
                                />
                            </CanvasShape>

                            <CanvasSticky
                                x=Signal::derive(move || note.get().x)
                                y=Signal::derive(move || note.get().y)
                                size=Signal::derive(move || note.get().width)
                                color=StickyColor::Yellow
                                on_move=Callback::new(move |at: CanvasPoint| {
                                    note.update(|note| (note.x, note.y) = (at.x, at.y));
                                })
                                on_resize=Callback::new(move |next: WorldBox| note.set(next))
                                on_gesture_end=record
                                selected=Signal::derive(move || picked.get() == "note")
                                on:pointerdown=move |_| picked.set("note".into())
                            >
                                <span class="text-[0.65rem] font-semibold tracking-wide uppercase opacity-60">
                                    "Note"
                                </span>
                                <CanvasNodeLabel
                                    value=note_label
                                    on_change=Callback::new(move |next| note_label.set(next))
                                    class="break-words"
                                />
                            </CanvasSticky>

                            <CanvasText
                                x=Signal::derive(move || tilted.get().x)
                                y=Signal::derive(move || tilted.get().y)
                                angle=Signal::derive(move || tilted.get().angle)
                                on_move=Callback::new(move |at: CanvasPoint| {
                                    tilted.update(|tilted| (tilted.x, tilted.y) = (at.x, at.y));
                                })
                                on_gesture_end=record
                                selected=Signal::derive(move || picked.get() == "tilted")
                                on:pointerdown=move |_| picked.set("tilted".into())
                                class="rounded-sm bg-background/70 px-1 py-0.5"
                            >
                                "…and one that has been turned"
                            </CanvasText>
                        </Canvas>
                        <p class="text-xs text-muted-foreground">
                            "Press one to select it, drag it to move it, drag a grip to resize it,
                            double-click its label to rename it. The note is square, so it fixes
                            the ratio at 1 and only its corners are drawn. The third one carries an
                            `angle`: it moves and it reports like the others, and it draws no grips,
                            because a resize takes its delta in world axes and a turned box's edges
                            do not lie along them."
                        </p>
                        <div class="rounded-lg border p-3">
                            <div class="mb-1.5 text-xs font-medium">
                                "What a peer would be sent"
                            </div>
                            {move || {
                                let log = log.get();
                                if log.is_empty() {
                                    return Either::Left(
                                        view! {
                                            <p class="text-xs text-muted-foreground">
                                                "Nothing yet. Selecting a node is not an edit, so a
                                                press that moved nothing says nothing."
                                            </p>
                                        },
                                    );
                                }
                                Either::Right(
                                    view! {
                                        <ul class="flex flex-col gap-0.5 font-mono text-[0.7rem] text-muted-foreground tabular-nums">
                                            {log
                                                .iter()
                                                .rev()
                                                .take(4)
                                                .map(|gesture| {
                                                    let what = match gesture.handle {
                                                        None => "moved  ".to_string(),
                                                        Some(handle) => format!("resize {}", handle.as_str()),
                                                    };
                                                    view! {
                                                        <li>
                                                            {format!(
                                                                "{what}  {:.0},{:.0} {:.0}×{:.0}  →  {:.0},{:.0} {:.0}×{:.0}",
                                                                gesture.from.x,
                                                                gesture.from.y,
                                                                gesture.from.width,
                                                                gesture.from.height,
                                                                gesture.to.x,
                                                                gesture.to.y,
                                                                gesture.to.width,
                                                                gesture.to.height,
                                                            )}
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </ul>
                                    },
                                )
                            }}
                        </div>
                    </div>
                </DemoSection>

                <DemoSection
                    title="The hand tool"
                    description="`pan_on_drag` puts the primary button back on the pan, for a board that is only ever looked at — a diagram, a map, a seating plan. The cursor says which one you are on: a pointer when a press means a click, a hand when it means a pan."
                    code=HAND
                >
                    <Canvas pan_on_drag=true class="h-72">
                        <CanvasShape x=60.0 y=60.0 width=150.0 height=80.0>
                            "Drag anywhere"
                        </CanvasShape>
                        <CanvasShape
                            x=280.0
                            y=100.0
                            width=110.0
                            height=110.0
                            kind=ShapeKind::Ellipse
                        >
                            "…the board follows"
                        </CanvasShape>
                    </Canvas>
                </DemoSection>

                <DemoSection
                    title="The viewport is a signal"
                    description="A plain `RwSignal<Viewport>`, which is as useful for reading where the reader has gone as for putting them somewhere. The arithmetic behind it lives in `utils::viewport` and is unit-tested off the DOM — zooming toward the pointer rather than the middle is the part worth having tests for."
                    code=CONTROLLED
                >
                    <div class="flex flex-col gap-3">
                        <Canvas viewport=viewport controls=false class="h-72">
                            <CanvasSticky x=60.0 y=60.0 color=StickyColor::Pink>
                                "Move me around and watch the numbers."
                            </CanvasSticky>
                            <CanvasShape x=280.0 y=110.0 width=140.0 height=80.0>
                                "Shape"
                            </CanvasShape>
                        </Canvas>
                        <p class="font-mono text-xs text-muted-foreground tabular-nums">
                            {move || {
                                let v = viewport.get();
                                format!("x {:>8.1}   y {:>8.1}   zoom {:>5.2}", v.x, v.y, v.zoom)
                            }}
                        </p>
                    </div>
                </DemoSection>

                <DemoSection
                    title="The gestures"
                    description="A browser reports a trackpad pinch as a wheel event with `ctrlKey` set — there is no pinch event on the desktop — so that flag, rather than a modifier anybody pressed, is what separates zooming from two-finger panning. A wheel notch means a ratio and not an amount, so two notches in cover the same ground however far in you already are."
                    code=GESTURES
                >
                    <div class="grid gap-2 text-sm sm:grid-cols-2">
                        <div class="rounded-lg border p-3">
                            <div class="font-medium">"Pan"</div>
                            <p class="mt-1 text-muted-foreground">
                                "Hold space and drag, or use the middle button. Two fingers on a trackpad. `pan_on_drag` adds the primary button."
                            </p>
                        </div>
                        <div class="rounded-lg border p-3">
                            <div class="font-medium">"Zoom"</div>
                            <p class="mt-1 text-muted-foreground">
                                "Ctrl or ⌘ with the wheel, a trackpad pinch, or the controls in the corner."
                            </p>
                        </div>
                        <div class="rounded-lg border p-3">
                            <div class="font-medium">"Keyboard"</div>
                            <p class="mt-1 text-muted-foreground">
                                "⌘0 resets the view, ⌘+ and ⌘- step the zoom. The surface is a tab stop so the keys can reach it."
                            </p>
                        </div>
                        <div class="rounded-lg border p-3">
                            <div class="font-medium">"Limits"</div>
                            <p class="mt-1 text-muted-foreground">
                                "0.1× to 4× by default. A zoom refused at the limit leaves the surface exactly where it was."
                            </p>
                        </div>
                    </div>
                </DemoSection>

                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}
