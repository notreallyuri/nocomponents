//! Canvas blocks.

use super::Block;
use leptos::{ev, prelude::*};
use leptos_node_ref::AnyNodeRef;
use nocomponents::{
    components::{
        badge::{Badge, BadgeVariant},
        button::{Button, ButtonSize, ButtonVariant},
        canvas::{
            Canvas, CanvasMarquee, CanvasNode, CanvasNodeLabel, CanvasShape, CanvasSticky,
            CanvasText, ShapeKind, StickyColor,
        },
        kbd::Kbd,
        label::Label,
        separator::Separator,
        slider::Slider,
        toggle::{ToggleSize, ToggleVariant},
        toggle_group::{ToggleGroup, ToggleGroupItem},
    },
    icons::IconRoot,
    primitives::{
        canvas::{CanvasDrag, CanvasPoint, surface_holds, world_at},
        drag::use_drag,
        toggle_group::ToggleGroupType,
    },
    utils::viewport::{Viewport, WorldBox},
};

/// The selection frame's depth. Above everything a reader can put on the board, because a frame
/// under one of its own members is a frame whose grips cannot be grabbed.
const FRAME_Z: i32 = i32::MAX;

const EDITOR: &str = r##"// You make a thing by dragging it off the palette and letting go over the
// board. One gesture, and where you drop it is where it lands — which is the
// only reason the canvas leaves the primary button alone: a board whose press
// pans has nothing left to select a node with, to drag one by, or to drop one
// on.

// A set rather than one id. `selected` on a node has always been a
// `Signal<bool>` each, so the state was already the right shape; the marquee
// is what made it a set.
let items = RwSignal::new(Vec::<Item>::new());
let selected = RwSignal::new(Vec::<usize>::new());
let band = RwSignal::new(None::<WorldBox>);

// Where everything selected was when the gesture began. A node reports where
// *it* would go and nothing else — which is the right thing for it to know —
// so moving five means turning that one position into a delta and applying it
// to the other four. The caller cannot read the delta off its own list, having
// overwritten those positions on every move since the press.
let held = StoredValue::new(Vec::<(usize, WorldBox)>::new());

// The palette lives outside the canvas, so a drop cannot come through
// `on_surface_click`. `world_at` is the canvas' own conversion, taking the
// surface's node ref and the viewport rather than making a caller measure and
// divide again; whether a release outside the board counts is the caller's
// question, so it is a second call.
let board = AnyNodeRef::new();
let drop_at = move |x: f64, y: f64| {
    surface_holds(board, x, y)
        .then(|| world_at(board, viewport.get_untracked(), x, y))
        .flatten()
};

<Canvas
    viewport=viewport
    node_ref=board
    // A press on the board is a click, and a click on nothing is a
    // deselection. A press that travelled is a band instead — the two share
    // one press and never both fire.
    on_surface_click=Callback::new(move |_| selected.set(Vec::new()))
    on_surface_drag=Callback::new(move |drag: CanvasDrag| {
        // The first report of a gesture is the one with no band yet, which is
        // where the selection to add to gets remembered. Empty unless shift is
        // held, so a plain band replaces and a shift one extends.
        if band.get_untracked().is_none() {
            base.set_value(if drag.shift { selected.get_untracked() } else { Vec::new() });
        }
        let mut ids = base.get_value();
        items.with_untracked(|items| for item in items {
            // What the band *touches*, not what it swallows whole: a band that
            // had to contain a node could never reach one bigger than the screen.
            if drag.rect.intersects(&item.box_()) && !ids.contains(&item.id) {
                ids.push(item.id);
            }
        });
        selected.set(ids);
        band.set((!drag.done).then_some(drag.rect));
    })
>
    <Nodes items=items selected=selected held=held />
    <CanvasMarquee rect=band />

    // The frame round a selection of more than one. Mounted on whether there
    // *is* one rather than rebuilt as it changes: the grips being dragged live
    // on this element, and replacing it mid-gesture takes the gesture with it.
    <Show when=move || frame.get().is_some()>
        <CanvasNode
            x=… y=… width=… height=…
            on_resize=Callback::new(move |to: WorldBox| {
                // Every member scaled inside the frame as though painted on
                // it, measured from where they were at the press — scaling
                // what is already scaled compounds, and a drag back to where
                // it started would not undo itself.
                let members = scaling.get_value();
                let from = WorldBox::around(members.iter().map(|(_, b)| *b))?;
                items.update(|items| for item in items.iter_mut() {
                    if let Some((_, was)) = members.iter().find(|(id, _)| *id == item.id) {
                        item.set_box(was.scaled_within(from, to));
                    }
                });
            })
            z=i32::MAX
            selected=true
            // The frame takes no pointer: a press inside it belongs to
            // whatever is under it — a member, or the board and a new band.
            // Only the grips are grabbable.
            class="pointer-events-none [&>[data-slot=canvas-node-handle]]:pointer-events-auto"
        />
    </Show>
</Canvas>

// A node reports its own gestures, so the block writes no pointer code at all
// for the things on the board.
#[component]
fn Nodes(/* … */) -> impl IntoView {
    // The ids, not the items: a drag rewrites the list forty times a second,
    // and keying on the ids means all that changes is the numbers in a style.
    let ids = Memo::new(move |_| items.with(|items| items.iter().map(|i| i.id).collect::<Vec<_>>()));

    view! {
        {move || ids.get().into_iter().map(|id| {
            view! {
                <CanvasSticky
                    x=… y=… size=…
                    // Where a drag on this node would put it. The zoom a screen
                    // delta is divided by lives in the canvas' context, so the
                    // node does the arithmetic — and this turns it into the
                    // distance everything else selected has to go too.
                    on_move=Callback::new(move |at: CanvasPoint| {
                        let held = held.get_value();
                        let (_, anchor) = held.iter().find(|(other, _)| *other == id)?;
                        let (dx, dy) = (at.x - anchor.x, at.y - anchor.y);
                        items.update(|items| for item in items.iter_mut() {
                            if let Some((_, was)) = held.iter().find(|(o, _)| *o == item.id) {
                                item.set_box(was.moved_by(dx, dy));
                            }
                        });
                    })
                    // A resize is one node's, always: the grips that scale a
                    // whole selection are on the frame drawn around it.
                    on_resize=Callback::new(move |box_: WorldBox| write(id, |i| i.set_box(box_)))
                    // Depth as a number rather than a position in the list.
                    // "To front" used to be a move to the end, which changes
                    // the keys and rebuilds every node after it.
                    z=…
                    selected=Signal::derive(move || selected.get().contains(&id))
                    // Three rules, and the middle one is what makes a group
                    // draggable at all: a press outside the selection replaces
                    // it, one on something already in it leaves it alone, and
                    // shift adds or removes — so a band can be corrected
                    // without being drawn again. Then everything selected is
                    // remembered, which is what the move above measures from.
                    on:pointerdown=move |e: ev::PointerEvent| { select(id, &e); }
                >
                    // Double-click to rename. The press goes through to the
                    // node, so a sticky can still be dragged by its words;
                    // it is the input that stops one, the caret's being the
                    // only gesture that outranks the node's.
                    <CanvasNodeLabel
                        value=label
                        on_change=Callback::new(move |next| write(id, |i| i.label = next))
                    />
                </CanvasSticky>
            }
        }).collect_view()}
    }
}"##;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Note,
    Rectangle,
    Ellipse,
    Text,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Kind::Note => "Note",
            Kind::Rectangle => "Rectangle",
            Kind::Ellipse => "Ellipse",
            Kind::Text => "Text",
        }
    }

    /// World width and height for one of these, given the size the sidebar is set to. Text has
    /// none of its own — a caption is as wide as its words, which is what `CanvasText` is for.
    fn size(self, picked: f64) -> (f64, f64) {
        match self {
            Kind::Rectangle => (picked * 1.4, picked * 0.75),
            Kind::Text => (0.0, 0.0),
            _ => (picked, picked),
        }
    }

    /// The path data is Lucide's, the same as the library's own icons; a block needs a glyph the
    /// styled layer never asked for, and `IconRoot` is what makes it look like the rest.
    fn icon(self) -> AnyView {
        match self {
            Kind::Note => view! {
                <IconRoot class="size-4">
                    <path d="M16 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h11l5-5V5a2 2 0 0 0-2-2z" />
                    <path d="M15 21v-4a2 2 0 0 1 2-2h4" />
                </IconRoot>
            }
            .into_any(),
            Kind::Rectangle => view! {
                <IconRoot class="size-4">
                    <rect width="18" height="14" x="3" y="5" rx="1" />
                </IconRoot>
            }
            .into_any(),
            Kind::Ellipse => view! {
                <IconRoot class="size-4">
                    <circle cx="12" cy="12" r="9" />
                </IconRoot>
            }
            .into_any(),
            Kind::Text => view! {
                <IconRoot class="size-4">
                    <path d="M4 7V4h16v3" />
                    <path d="M12 4v16" />
                    <path d="M9 20h6" />
                </IconRoot>
            }
            .into_any(),
        }
    }
}

const KINDS: &[Kind] = &[Kind::Note, Kind::Rectangle, Kind::Ellipse, Kind::Text];

/// Slug, the colour a note is given, and the swatch that stands for it. A literal per arm, since
/// Tailwind reads the source for class names and cannot follow one assembled at runtime.
const COLORS: &[(&str, StickyColor, &str)] = &[
    ("yellow", StickyColor::Yellow, "bg-amber-300"),
    ("green", StickyColor::Green, "bg-emerald-300"),
    ("blue", StickyColor::Blue, "bg-sky-300"),
    ("pink", StickyColor::Pink, "bg-pink-300"),
];

#[derive(Clone, PartialEq)]
struct Item {
    id: usize,
    kind: Kind,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    /// Depth, as a number rather than as a position in this list. "To front" used to be a move to
    /// the end of the `Vec`, which changes the keys and rebuilds every node after it; a `z` prop
    /// makes it one number nothing else has to agree with.
    z: i32,
    color: StickyColor,
    label: String,
}

impl Item {
    /// The box it actually occupies, which is not always the four numbers it stores.
    ///
    /// A note is square — `CanvasSticky` takes one `size` — so its height is its width whatever
    /// `h` says. A text node has no size of its own at all: it is as wide as its words, and only
    /// the DOM knows how wide that is. Reported as a point, so a marquee still catches it (a
    /// zero-sized box intersects whatever contains it) and a frame around it is not stretched to
    /// a guess.
    fn box_(&self) -> WorldBox {
        match self.kind {
            Kind::Note => WorldBox::new(self.x, self.y, self.w, self.w),
            Kind::Text => WorldBox::new(self.x, self.y, 0.0, 0.0),
            _ => WorldBox::new(self.x, self.y, self.w, self.h),
        }
    }

    /// The inverse, and the one place a move or a scale is written back.
    ///
    /// A note stays square through a group scale that stretched one axis more than the other:
    /// the width wins, because that is the number `CanvasSticky` is drawn from and a note whose
    /// stored height disagreed with its drawn one would put the next frame in the wrong place.
    fn set_box(&mut self, box_: WorldBox) {
        (self.x, self.y) = (box_.x, box_.y);
        match self.kind {
            Kind::Note => (self.w, self.h) = (box_.width, box_.width),
            Kind::Text => {}
            _ => (self.w, self.h) = (box_.width, box_.height),
        }
    }
}

/// What is on the board.
///
/// A component of its own rather than markup inside the block, because a node reports its own
/// gestures through the canvas' context — which only something *inside* the canvas can reach.
/// The block used to run its own `use_drag` here and divide by the zoom by hand; `on_move` and
/// `on_resize` are that arithmetic moved to where it belongs, and every consumer gets it.
///
/// `held` is where every selected node was when the current gesture began. A node reports where
/// *it* would go and nothing else — which is the right thing for it to know — so moving a
/// selection of five means turning that one position into a delta and applying it to the other
/// four. The caller cannot read the delta off its own list, having overwritten those positions on
/// every move since the press, which is the same reason `CanvasGesture` carries `from`.
#[component]
fn Nodes(
    items: RwSignal<Vec<Item>>,
    selected: RwSignal<Vec<usize>>,
    held: StoredValue<Vec<(usize, WorldBox)>>,
) -> impl IntoView {
    // The ids, not the items: a drag rewrites the list forty times a second, and keying the markup
    // on the ids means all that changes in the DOM is the numbers in one `style`.
    let ids = Memo::new(move |_| {
        items.with(|items| items.iter().map(|item| item.id).collect::<Vec<_>>())
    });

    // A member of a selection shows no grips of its own. The frame drawn round the set has them,
    // and they land on the same corners: two grips over one point is a coin toss about which one
    // the reader grabbed, and the answers differ — one scales the set, the other scales a node
    // out of it.
    let grips = Signal::derive(move || {
        if selected.get().len() < 2 {
            String::new()
        } else {
            "[&>[data-slot=canvas-node-handle]]:hidden".to_string()
        }
    });

    view! {
        {move || {
            ids.get()
                .into_iter()
                .map(|id| {
                    let seed = items
                        .with_untracked(|items| items.iter().find(|item| item.id == id).cloned());
                    let Some(seed) = seed else {
                        return ().into_any();
                    };
                    let field = move |pick: fn(&Item) -> f64| {
                        Signal::derive(move || {
                            items
                                .with(|items| {
                                    items.iter().find(|item| item.id == id).map_or(0.0, pick)
                                })
                        })
                    };
                    let (x, y) = (field(|item| item.x), field(|item| item.y));
                    let (w, h) = (field(|item| item.w), field(|item| item.h));
                    let label = Signal::derive(move || {
                        items
                            .with(|items| {
                                items
                                    .iter()
                                    .find(|item| item.id == id)
                                    .map_or(String::new(), |item| item.label.clone())
                            })
                    });
                    let z = Signal::derive(move || {
                        items
                            .with(|items| {
                                items.iter().find(|item| item.id == id).map_or(0, |item| item.z)
                            })
                    });
                    let is_selected = Signal::derive(move || selected.get().contains(&id));
                    let editing = RwSignal::new(false);
                    let moved = Callback::new(move |at: CanvasPoint| {
                        let held = held.get_value();
                        let Some((_, anchor)) = held.iter().find(|(other, _)| *other == id) else {
                            return;
                        };
                        let (dx, dy) = (at.x - anchor.x, at.y - anchor.y);
                        items
                            .update(|items| {
                                for item in items.iter_mut() {
                                    if let Some((_, was)) = held
                                        .iter()
                                        .find(|(other, _)| *other == item.id)
                                    {
                                        item.set_box(was.moved_by(dx, dy));
                                    }
                                }
                            });
                    });
                    let resized = Callback::new(move |box_: WorldBox| {
                        items
                            .update(|items| {
                                if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                                    item.set_box(box_);
                                }
                            });
                    });
                    let renamed = Callback::new(move |next: String| {
                        items
                            .update(|items| {
                                if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                                    item.label = next;
                                }
                            });
                    });
                    let select = move |e: ev::PointerEvent| {
                        if e.button() != 0 {
                            return;
                        }
                        selected
                            .update(|ids| {
                                if e.shift_key() {
                                    match ids.iter().position(|other| *other == id) {
                                        Some(at) => {
                                            ids.remove(at);
                                        }
                                        None => ids.push(id),
                                    }
                                } else if !ids.contains(&id) {
                                    ids.clear();
                                    ids.push(id);
                                }
                            });
                        let ids = selected.get_untracked();
                        held.set_value(
                            items
                                .with_untracked(|items| {
                                    items
                                        .iter()
                                        .filter(|item| item.id == id || ids.contains(&item.id))
                                        .map(|item| (item.id, item.box_()))
                                        .collect()
                                }),
                        );
                    };
                    match seed.kind {
                        Kind::Note => {

                            // One derived signal per number the node draws itself from. The list is what
                            // changes; these are what the DOM listens to.

                            // Three gestures, one closure each, and every one of them a write to the same
                            // list. Nothing here knows about the zoom or about pointers.
                            //
                            // The move is the only one that is not about this node alone: where the node
                            // under the pointer went, measured from where it started, is the distance
                            // everything else selected has to go too.
                            // A resize is one node's, always: the grips on a node belong to it, and the
                            // ones that scale a whole selection are on the frame drawn around it.
                            // A press selects; the node's own gesture does the moving. Not `on:click`,
                            // which would not fire until the drag was over.
                            //
                            // Three rules, and the middle one is what makes a group draggable at all: a
                            // press on something *outside* the selection replaces it, a press on
                            // something already in it leaves it alone, and shift adds or removes — so a
                            // marquee can be corrected without being drawn again.
                            // Where everything goes back to being measured from. The node under the
                            // pointer is in here whether or not it is selected — it is the one being
                            // dragged, and a shift-press that just deselected it is still a press on
                            // the thing the reader has hold of.

                            view! {
                                <CanvasSticky
                                    x=x
                                    y=y
                                    size=w
                                    color=seed.color
                                    on_move=moved
                                    on_resize=resized
                                    z=z
                                    selected=is_selected
                                    class=grips
                                    on:pointerdown=select
                                >
                                    <span class="text-[0.65rem] font-semibold tracking-wide uppercase opacity-60">
                                        "Note"
                                    </span>
                                    <CanvasNodeLabel
                                        value=label
                                        on_change=renamed
                                        editing=editing
                                        class="break-words"
                                    />
                                </CanvasSticky>
                            }
                                .into_any()
                        }
                        Kind::Rectangle | Kind::Ellipse => {
                            let kind = if seed.kind == Kind::Ellipse {
                                ShapeKind::Ellipse
                            } else {
                                ShapeKind::Rectangle
                            };
                            let aspect = (seed.kind == Kind::Ellipse).then_some(1.0);
                            // An ellipse keeps its ratio, because a squashed circle drawn with
                            // `rounded-full` is an ellipse the reader cannot tell they made.
                            view! {
                                <CanvasShape
                                    x=x
                                    y=y
                                    width=w
                                    height=h
                                    kind=kind
                                    aspect=aspect
                                    on_move=moved
                                    on_resize=resized
                                    z=z
                                    selected=is_selected
                                    class=grips
                                    on:pointerdown=select
                                >
                                    <CanvasNodeLabel
                                        value=label
                                        on_change=renamed
                                        editing=editing
                                        class="text-center"
                                    />
                                </CanvasShape>
                            }
                                .into_any()
                        }
                        Kind::Text => {
                            view! {
                                <CanvasText
                                    x=x
                                    y=y
                                    on_move=moved
                                    z=z
                                    selected=is_selected
                                    class="rounded-sm bg-background/70 px-1 py-0.5"
                                    on:pointerdown=select
                                >
                                    <CanvasNodeLabel
                                        value=label
                                        on_change=renamed
                                        editing=editing
                                    />
                                </CanvasText>
                            }
                                .into_any()
                        }
                    }
                })
                .collect_view()
        }}
    }
}

fn editor() -> AnyView {
    let items = RwSignal::new(Vec::<Item>::new());
    // A set rather than one id. `selected` on a node is a `Signal<bool>` each, so the state was
    // always the right shape for this; what made it a set is the marquee.
    let selected = RwSignal::new(Vec::<usize>::new());
    let next = RwSignal::new(1usize);
    let color = RwSignal::new(vec!["yellow".to_string()]);
    let size = RwSignal::new(vec![140.0]);

    // The band being dragged over the board, and the selection it is adding to — empty unless
    // shift is held, which is what makes a second marquee extend the first rather than replace
    // it. The gesture's first report is the one where there is no band yet, which is where that
    // gets remembered.
    let band = RwSignal::new(None::<WorldBox>);
    let base = StoredValue::new(Vec::<usize>::new());

    // Where everything selected was when the current gesture began. One for the move, taken at
    // the press; one for the frame's scale, taken at its first report — a grip's press stops
    // propagating before it reaches anything of ours, so there is nowhere earlier to take it.
    let held = StoredValue::new(Vec::<(usize, WorldBox)>::new());
    let scaling = StoredValue::new(Vec::<(usize, WorldBox)>::new());

    // The viewport is a plain signal, and holding it here is what lets the palette — which lives
    // outside the canvas and so cannot `use_canvas()` — work out where a release landed.
    let viewport = RwSignal::new(Viewport::default());
    let board = AnyNodeRef::new();

    // What is being carried off the palette, and where the pointer has it. `None` the rest of the
    // time, which is also what says whether to draw the thing following the pointer.
    let carried = RwSignal::new(None::<Kind>);
    let ghost = RwSignal::new((0.0f64, 0.0f64));

    let current_color = Signal::derive(move || {
        color
            .with(|color| {
                color
                    .first()
                    .and_then(|slug| COLORS.iter().find(|entry| entry.0 == slug.as_str()))
                    .map(|entry| entry.1)
            })
            .unwrap_or_default()
    });
    let picked = Signal::derive(move || size.with(|size| size.first().copied().unwrap_or(140.0)));

    // Where a release lands on the board, in world units. A drop that began in the sidebar is not
    // a click the canvas ever sees, so it cannot come through `on_surface_click` — but the
    // conversion is the canvas', and `world_at` is it, taking the surface's node ref and the
    // viewport rather than making every caller measure and divide again. A release outside the
    // board is a release onto nothing, which is the caller's question and so a second call.
    let drop_at = move |client_x: f64, client_y: f64| -> Option<CanvasPoint> {
        surface_holds(board, client_x, client_y)
            .then(|| world_at(board, viewport.get_untracked(), client_x, client_y))
            .flatten()
    };

    let place = move |kind: Kind, at: CanvasPoint| {
        let (w, h) = kind.size(picked.get_untracked());
        let id = next.get_untracked();

        items.update(|items| {
            items.push(Item {
                id,
                kind,
                // Dropped from the middle, because that is where the thing was under the pointer
                // on the way over.
                x: at.x - w / 2.0,
                y: at.y - h / 2.0,
                w,
                h,
                z: 0,
                color: current_color.get_untracked(),
                // Named for what it is and numbered. There is no field to type one into: a label
                // is edited where it is, by double-clicking it on the board.
                label: format!("{} {id}", kind.label()),
            })
        });
        next.update(|next| *next += 1);
        selected.set(vec![id]);
    };

    // One gesture from the sidebar to the board. `use_drag` listens on the window, so it does not
    // care that it began on one side of a border and ended on the other — which is the whole
    // reason a drop can be a drop rather than two clicks that have to be explained to the reader.
    let palette = use_drag(move |point| ghost.set((point.client_x, point.client_y)))
        .on_start(move |point| ghost.set((point.client_x, point.client_y)))
        .on_end(move |end| {
            let Some(kind) = carried.get_untracked() else {
                return;
            };
            carried.set(None);
            if let Some(at) = drop_at(end.point.client_x, end.point.client_y) {
                place(kind, at);
            }
        });

    // A band over the board selects what it touches — `intersects` rather than containment,
    // because a band that had to swallow a node whole could never reach one bigger than the
    // screen. It reports on every move and once more on release; the band is drawn from the
    // running report and the selection is what the last one leaves behind.
    let marquee = Callback::new(move |drag: CanvasDrag| {
        if band.get_untracked().is_none() {
            base.set_value(if drag.shift {
                selected.get_untracked()
            } else {
                Vec::new()
            });
        }
        let mut ids = base.get_value();
        items.with_untracked(|items| {
            for item in items {
                if drag.rect.intersects(&item.box_()) && !ids.contains(&item.id) {
                    ids.push(item.id);
                }
            }
        });
        selected.set(ids);
        band.set((!drag.done).then_some(drag.rect));
    });

    // The frame drawn round a selection of more than one, and what its grips do. Nothing is drawn
    // for a single node — it has grips of its own, and a second set over them would be two ways
    // to do the same thing that disagree at the edges.
    let frame = Memo::new(move |_| {
        let ids = selected.get();
        if ids.len() < 2 {
            return None;
        }
        items.with(|items| {
            WorldBox::around(
                items
                    .iter()
                    .filter(|item| ids.contains(&item.id))
                    .map(Item::box_),
            )
        })
    });

    // Every member scaled inside the frame as though it were painted on it, measured from where
    // they were at the press — scaling what is already scaled compounds, and a drag back to
    // where it started would not undo itself.
    let scale_group = Callback::new(move |to: WorldBox| {
        if scaling.with_value(Vec::is_empty) {
            let ids = selected.get_untracked();
            scaling.set_value(items.with_untracked(|items| {
                items
                    .iter()
                    .filter(|item| ids.contains(&item.id))
                    .map(|item| (item.id, item.box_()))
                    .collect()
            }));
        }
        let members = scaling.get_value();
        let Some(from) = WorldBox::around(members.iter().map(|(_, box_)| *box_)) else {
            return;
        };
        items.update(|items| {
            for item in items.iter_mut() {
                if let Some((_, was)) = members.iter().find(|(other, _)| *other == item.id) {
                    item.set_box(was.scaled_within(from, to));
                }
            }
        });
    });

    let delete_selected = move || {
        let ids = selected.get_untracked();
        if ids.is_empty() {
            return;
        }
        items.update(|items| items.retain(|item| !ids.contains(&item.id)));
        selected.set(Vec::new());
    };

    let duplicate_selected = move || {
        let ids = selected.get_untracked();
        let copies = items.with_untracked(|items| {
            items
                .iter()
                .filter(|item| ids.contains(&item.id))
                .cloned()
                .collect::<Vec<_>>()
        });
        if copies.is_empty() {
            return;
        }
        let start = next.get_untracked();
        let fresh = (start..start + copies.len()).collect::<Vec<_>>();
        next.set(start + copies.len());
        items.update(|items| {
            for (mut copy, id) in copies.into_iter().zip(fresh.iter().copied()) {
                copy.id = id;
                copy.x += 24.0;
                copy.y += 24.0;
                items.push(copy);
            }
        });
        selected.set(fresh);
    };

    // Depth is a number now rather than a position in this list, so "to front" writes one field
    // instead of moving items about and changing the keys every node after them is drawn from.
    let raise_selected = move || {
        let ids = selected.get_untracked();
        if ids.is_empty() {
            return;
        }
        items.update(|items| {
            let mut top = items.iter().map(|item| item.z).max().unwrap_or(0);
            for item in items.iter_mut().filter(|item| ids.contains(&item.id)) {
                top = top.saturating_add(1);
                item.z = top;
            }
        });
    };

    let chosen = Signal::derive(move || {
        let ids = selected.get();
        items.with(|items| {
            items
                .iter()
                .filter(|item| ids.contains(&item.id))
                .cloned()
                .collect::<Vec<_>>()
        })
    });

    view! {
        <div class="flex w-full flex-col gap-2">
            <div class="flex h-[30rem] w-full overflow-hidden rounded-lg border">
                <aside class="flex w-52 shrink-0 flex-col border-r bg-muted/30 select-none">
                    // The palette scrolls and the footer does not: a count you have to scroll to
                    // is a count nobody reads.
                    <div class="flex flex-1 flex-col gap-3 overflow-y-auto p-3">
                        <div class="flex flex-col gap-1.5">
                            <Label class="text-xs text-muted-foreground">
                                "Drag onto the board"
                            </Label>
                            <div class="grid grid-cols-2 gap-1.5">
                                {KINDS
                                    .iter()
                                    .map(|kind| {
                                        let kind = *kind;
                                        view! {
                                            <button
                                                type="button"
                                                class="flex cursor-grab touch-none flex-col items-center gap-1 rounded-lg border bg-background p-2 text-[0.7rem] text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground active:cursor-grabbing data-[carried=true]:border-ring data-[carried=true]:text-foreground"
                                                data-carried=move || {
                                                    (carried.get() == Some(kind)).then_some("true")
                                                }
                                                on:pointerdown=move |e: ev::PointerEvent| {
                                                    if e.button() != 0 {
                                                        return;
                                                    }
                                                    // Otherwise the press begins a text selection
                                                    // that runs along behind the gesture.
                                                    e.prevent_default();
                                                    carried.set(Some(kind));
                                                    palette.start(&e);
                                                }
                                            >
                                                {kind.icon()}
                                                <span>{kind.label()}</span>
                                            </button>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </div>

                        <Separator />

                        <div class="flex flex-col gap-1.5">
                            <Label class="text-xs text-muted-foreground">"Colour"</Label>
                            <ToggleGroup
                                kind=ToggleGroupType::Single
                                variant=ToggleVariant::Outline
                                size=ToggleSize::Sm
                                value=color
                                class="w-full"
                            >
                                {COLORS
                                    .iter()
                                    .map(|(slug, _, swatch)| {
                                        view! {
                                            <ToggleGroupItem value=*slug class="flex-1 px-0">
                                                <span class=format!("size-3.5 rounded-full {swatch}") />
                                            </ToggleGroupItem>
                                        }
                                    })
                                    .collect_view()}
                            </ToggleGroup>
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <Label class="flex items-center justify-between text-xs text-muted-foreground">
                                <span>"Size"</span>
                                <span class="font-mono tabular-nums">
                                    {move || format!("{:.0}", picked.get())}
                                </span>
                            </Label>
                            <Slider value=size min=60.0 max=320.0 step=10.0 />
                        </div>
                    </div>

                    // The inspector is pinned rather than scrolled with the palette: what is
                    // selected, and what you can do to it, is the one thing that must never be
                    // below the fold.
                    <div class="flex flex-col gap-2 border-t p-3">
                        {move || {
                            let picked = chosen.get();
                            if picked.is_empty() {
                                return view! {
                                    <p class="text-xs leading-relaxed text-muted-foreground">
                                        "Nothing selected. Drag something off the palette above, then press it — or drag a band across the board."
                                    </p>
                                }
                                    .into_any();
                            }
                            let bounds = WorldBox::around(picked.iter().map(Item::box_))
                                .unwrap_or_default();
                            let (title, kind) = match picked.as_slice() {
                                [item] => (item.label.clone(), item.kind.label().to_string()),
                                many => {
                                    (format!("{} selected", many.len()), "Selection".to_string())
                                }
                            };
                            // One box around whatever is selected, which for a single node is
                            // the node. Everything below reads the same numbers either way.
                            view! {
                                <div class="flex items-center justify-between gap-2">
                                    <span class="truncate text-sm font-medium">{title}</span>
                                    <Badge variant=BadgeVariant::Secondary>{kind}</Badge>
                                </div>
                                <span class="font-mono text-[0.7rem] text-muted-foreground tabular-nums">
                                    {format!(
                                        "x {:.0}  y {:.0}  {:.0}×{:.0}",
                                        bounds.x,
                                        bounds.y,
                                        bounds.width,
                                        bounds.height,
                                    )}
                                </span>
                                <div class="flex flex-wrap gap-1.5">
                                    <Button
                                        variant=ButtonVariant::Outline
                                        size=ButtonSize::Sm
                                        on:click=move |_| duplicate_selected()
                                    >
                                        "Duplicate"
                                    </Button>
                                    <Button
                                        variant=ButtonVariant::Outline
                                        size=ButtonSize::Sm
                                        on:click=move |_| raise_selected()
                                    >
                                        "To front"
                                    </Button>
                                    <Button
                                        variant=ButtonVariant::Destructive
                                        size=ButtonSize::Sm
                                        on:click=move |_| delete_selected()
                                    >
                                        "Delete"
                                    </Button>
                                </div>
                            }
                                .into_any()
                        }}
                    </div>

                    <div class="flex items-center justify-between gap-1 border-t p-2">
                        <Badge variant=BadgeVariant::Outline>
                            {move || format!("{} on the board", items.with(Vec::len))}
                        </Badge>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Sm
                            on:click=move |_| {
                                items.set(Vec::new());
                                selected.set(Vec::new());
                            }
                        >
                            "Clear"
                        </Button>
                    </div>
                </aside>

                <div class="relative flex-1">
                    <Canvas
                        viewport=viewport
                        // The surface's own box, which the palette needs to work out where a
                        // release landed. Taken from the canvas rather than measured off this
                        // wrapper: the two are the same box only until somebody puts a margin
                        // or a second child on it.
                        node_ref=board
                        // A press on the board is a click, and a click on nothing is a
                        // deselection — a gesture a surface that panned on press has nowhere
                        // to put.
                        on_surface_click=Callback::new(move |_: CanvasPoint| {
                            selected.set(Vec::new())
                        })
                        // And a press that travelled is a band. The two share one press and
                        // never both fire, so a click still clears and a drag still selects.
                        on_surface_drag=marquee
                        class="h-full rounded-none border-0"
                        on:keydown=move |e: ev::KeyboardEvent| {
                            if matches!(e.key().as_str(), "Delete" | "Backspace") {
                                e.prevent_default();
                                delete_selected();
                            }
                        }
                    >
                        <Nodes items=items selected=selected held=held />
                        <CanvasMarquee rect=band />
                        // Mounted on whether there *is* a frame rather than rebuilt as it
                        // changes: the grips being dragged live on this element, and replacing
                        // it mid-gesture takes the gesture with it.
                        <Show when=move || frame.get().is_some()>
                            <CanvasNode
                                x=Signal::derive(move || frame.get().map_or(0.0, |box_| box_.x))
                                y=Signal::derive(move || frame.get().map_or(0.0, |box_| box_.y))
                                width=Signal::derive(move || {
                                    frame.get().map_or(0.0, |box_| box_.width)
                                })
                                height=Signal::derive(move || {
                                    frame.get().map_or(0.0, |box_| box_.height)
                                })
                                on_resize=scale_group
                                on_gesture_end=Callback::new(move |_| scaling.set_value(Vec::new()))
                                z=FRAME_Z
                                selected=true
                                min_size=8.0
                                // The frame itself takes no pointer: a press inside it belongs
                                // to whatever is under it — a member, or the board and a new
                                // band. Only the grips are grabbable.
                                class="pointer-events-none border-[length:calc(1px/var(--canvas-zoom))] border-dashed border-primary/70 [&>[data-slot=canvas-node-handle]]:pointer-events-auto"
                            />
                        </Show>
                    </Canvas>
                </div>
            </div>

            <p class="text-xs text-muted-foreground">
                "Drag a shape off the palette and let go over the board. Press one to select it,
                drag a band across the board to select several, " <Kbd>"Shift"</Kbd>
                " to add or remove. Drag any of them to move them all, drag the frame's grips to
                scale them, double-click a label to rename. " <Kbd>"Space"</Kbd>
                " and drag to pan, " <Kbd>"⌫"</Kbd> " to delete."
            </p>

            // What is being carried, drawn at the size and zoom it will land at, so the drop is
            // not a surprise. `fixed` and `pointer-events-none`: it follows the pointer across
            // the border between the two panes, and must never be what the release lands on.
            {move || {
                carried
                    .get()
                    .map(|kind| {
                        let (x, y) = ghost.get();
                        let zoom = viewport.get().zoom;
                        let (w, h) = kind.size(picked.get());
                        view! {
                            <div
                                class="pointer-events-none fixed z-50 flex items-center justify-center rounded-md border-2 border-dashed border-ring bg-accent/60 text-xs font-medium text-foreground"
                                style=format!(
                                    "left: {x}px; top: {y}px; width: {}px; height: {}px; transform: translate(-50%, -50%);",
                                    (w * zoom).max(72.0),
                                    (h * zoom).max(28.0),
                                )
                            >
                                {kind.label()}
                            </div>
                        }
                    })
            }}
        </div>
    }
    .into_any()
}

pub const BLOCKS: &[Block] = &[Block {
    slug: "a-board-you-drag-things-onto",
    title: "A board you drag things onto",
    uses: &[
        "badge",
        "button",
        "canvas",
        "kbd",
        "label",
        "separator",
        "slider",
        "toggle-group",
    ],
    description: "A palette down the side, and you make a thing by dragging it off and letting go over \
                  the board. Then move it, drag its grips to resize it, and double-click its \
                  label to rename it — there is no field for a name in the sidebar, because the \
                  name is edited where it is. This is the block the canvas' default gesture \
                  exists for: a surface that pans on the primary button has no press left to \
                  select a node with, none to drag one by, none to drag a band with, and nowhere \
                  to put a click on empty space that means \"deselect\". The drop crosses a \
                  border no context reaches — the palette sits outside the canvas — so it goes \
                  through `world_at`, which is the canvas' own conversion taking the surface's \
                  node ref and the viewport rather than making a caller measure and divide again. \
                  `use_drag` listens on the window, so one gesture spans both panes, and the thing \
                  following the pointer is drawn at the size and zoom it will land at. \
                  Selection is a set: a band over the board takes what it touches, shift adds and \
                  removes, and a press on something already selected leaves the set alone, which \
                  is what makes a group draggable by any of its members. Moving several is the \
                  one piece of arithmetic the canvas cannot do for you — a node reports where \
                  *it* would go, which is the right thing for it to know — so the block turns \
                  that into a delta and applies it to the rest, measured from what everything was \
                  at the press rather than from a list it has been overwriting since. Scaling \
                  several is a frame drawn around them whose grips are a `CanvasNode` of its own, \
                  with `WorldBox::scaled_within` keeping each member's place in it; the frame \
                  takes no pointer, so a press inside it still reaches the member underneath. \
                  Everything after the drop is `on_move`, `on_resize`, `on_surface_drag` and \
                  `CanvasNodeLabel`: the block writes no pointer code for the things on the \
                  board. Depth is a `z` prop rather than a position in the list, so \"to front\" \
                  writes one number instead of moving items about and rebuilding every node after \
                  them. The nodes are keyed on their ids rather than on their values, so a drag \
                  rewriting the list forty times a second changes the numbers in one `style` \
                  attribute instead of the board.",
    code: EDITOR,
    view: editor,
}];
