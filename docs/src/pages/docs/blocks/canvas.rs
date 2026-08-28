//! Canvas blocks.

use super::Block;
use leptos::{ev, prelude::*};
use leptos_node_ref::AnyNodeRef;
use nocomponents::{
    components::{
        badge::{Badge, BadgeVariant},
        button::{Button, ButtonSize, ButtonVariant},
        canvas::{
            Canvas, CanvasNodeLabel, CanvasShape, CanvasSticky, CanvasText, ShapeKind, StickyColor,
        },
        input::Input,
        kbd::Kbd,
        label::Label,
        separator::Separator,
        slider::Slider,
        toggle::{ToggleSize, ToggleVariant},
        toggle_group::{ToggleGroup, ToggleGroupItem},
    },
    icons::IconRoot,
    primitives::{canvas::CanvasPoint, drag::use_drag, toggle_group::ToggleGroupType},
    utils::viewport::{Viewport, WorldBox},
};

const EDITOR: &str = r##"// You make a thing by dragging it off the palette and letting go over the
// board. One gesture, and where you drop it is where it lands — which is the
// only reason the canvas leaves the primary button alone: a board whose press
// pans has nothing left to select a node with, or to drop one on.

let items = RwSignal::new(Vec::<Item>::new());
let selected = RwSignal::new(None::<usize>);
// The viewport is a plain signal, and holding it is what lets the palette —
// which lives outside the canvas and cannot ask it anything — work out where
// a release landed in world coordinates.
let viewport = RwSignal::new(Viewport::default());
let board = AnyNodeRef::new();

let world_at = move |x: f64, y: f64| {
    let rect = board.get_untracked()?.get_bounding_client_rect();
    // A release outside the board is a release onto nothing.
    if x < rect.left() || x > rect.right() || y < rect.top() || y > rect.bottom() {
        return None;
    }
    let (x, y) = viewport.get_untracked().screen_to_world(x - rect.left(), y - rect.top());
    Some(CanvasPoint { x, y })
};

// The palette's gesture. `use_drag` listens on the window, so it does not
// care that it began on one side of a border and ended on the other.
let carried = RwSignal::new(None::<Kind>);
let ghost = RwSignal::new((0.0, 0.0));

let palette = use_drag(move |point| ghost.set((point.client_x, point.client_y)))
    .on_start(move |point| ghost.set((point.client_x, point.client_y)))
    .on_end(move |end| {
        let Some(kind) = carried.get_untracked() else { return };
        carried.set(None);
        if let Some(at) = world_at(end.point.client_x, end.point.client_y) {
            place(kind, at);
        }
    });

<div node_ref=board>
    <Canvas
        viewport=viewport
        // A press on the board is a click, and a click on nothing is a
        // deselection. There is no such gesture on a surface that pans.
        on_surface_click=Callback::new(move |_| selected.set(None))
    >
        <Nodes items=items selected=selected />
    </Canvas>
</div>

// A node reports its own gestures, so the block writes no pointer code at all
// for the things on the board: three callbacks, each a write to the same list.
#[component]
fn Nodes(/* … */) -> impl IntoView {
    // The ids, not the items: a drag rewrites the list forty times a second,
    // and keying on the ids means all that changes is the numbers in a style.
    let ids = Memo::new(move |_| items.with(|items| items.iter().map(|i| i.id).collect::<Vec<_>>()));

    view! {
        {move || ids.get().into_iter().map(|id| {
            let field = move |pick: fn(&Item) -> f64| {
                Signal::derive(move || items.with(|items|
                    items.iter().find(|item| item.id == id).map_or(0.0, pick)))
            };

            view! {
                <CanvasSticky
                    x=field(|item| item.x) y=field(|item| item.y) size=field(|item| item.w)
                    // Where a drag on the node would put it, in world units.
                    // The zoom a screen delta is divided by lives in the
                    // canvas' context, so the node does the arithmetic.
                    on_move=Callback::new(move |at: CanvasPoint| write(id, |item| {
                        (item.x, item.y) = (at.x, at.y);
                    }))
                    // And what a drag on one of the grips would make of it.
                    // A sticky is square, so `CanvasSticky` fixes the ratio
                    // at 1 and only the four corners are drawn.
                    on_resize=Callback::new(move |box_: WorldBox| write(id, |item| {
                        (item.x, item.y, item.w, item.h) =
                            (box_.x, box_.y, box_.width, box_.height);
                    }))
                    selected=Signal::derive(move || selected.get() == Some(id))
                    on:pointerdown=move |_| selected.set(Some(id))
                >
                    // Double-click to rename. The press goes through to the
                    // node, so a sticky can still be dragged by its words;
                    // it is the input that stops one, the caret's being the
                    // only gesture that outranks the node's.
                    <CanvasNodeLabel
                        value=label
                        on_change=Callback::new(move |next| write(id, |item| item.label = next))
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
    color: StickyColor,
    label: String,
}

/// What is on the board.
///
/// A component of its own rather than markup inside the block, because a node reports its own
/// gestures through the canvas' context — which only something *inside* the canvas can reach.
/// The block used to run its own `use_drag` here and divide by the zoom by hand; `on_move` and
/// `on_resize` are that arithmetic moved to where it belongs, and every consumer gets it.
#[component]
fn Nodes(items: RwSignal<Vec<Item>>, selected: RwSignal<Option<usize>>) -> impl IntoView {
    // The ids, not the items: a drag rewrites the list forty times a second, and keying the markup
    // on the ids means all that changes in the DOM is the numbers in one `style`.
    let ids = Memo::new(move |_| {
        items.with(|items| items.iter().map(|item| item.id).collect::<Vec<_>>())
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

                    // One derived signal per number the node draws itself from. The list is what
                    // changes; these are what the DOM listens to.
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
                    let is_selected = Signal::derive(move || selected.get() == Some(id));
                    let editing = RwSignal::new(false);

                    // Three gestures, one closure each, and every one of them a write to the same
                    // list. Nothing here knows about the zoom or about pointers.
                    let moved = Callback::new(move |at: CanvasPoint| {
                        items
                            .update(|items| {
                                if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                                    (item.x, item.y) = (at.x, at.y);
                                }
                            });
                    });
                    let resized = Callback::new(move |box_: WorldBox| {
                        items
                            .update(|items| {
                                if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                                    (item.x, item.y) = (box_.x, box_.y);
                                    (item.w, item.h) = (box_.width, box_.height);
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
                    // A press selects; the node's own gesture does the moving. Not `on:click`,
                    // which would not fire until the drag was over.
                    let select = move |_: ev::PointerEvent| selected.set(Some(id));

                    match seed.kind {
                        Kind::Note => {
                            view! {
                                <CanvasSticky
                                    x=x
                                    y=y
                                    size=w
                                    color=seed.color
                                    on_move=moved
                                    on_resize=resized
                                    selected=is_selected
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
                            // An ellipse keeps its ratio, because a squashed circle drawn with
                            // `rounded-full` is an ellipse the reader cannot tell they made.
                            let aspect = (seed.kind == Kind::Ellipse).then_some(1.0);
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
                                    selected=is_selected
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
                                    selected=is_selected
                                    class="rounded-sm bg-background/70 px-1 py-0.5"
                                    on:pointerdown=select
                                >
                                    <CanvasNodeLabel value=label on_change=renamed editing=editing />
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
    let selected = RwSignal::new(None::<usize>);
    let next = RwSignal::new(1usize);
    let color = RwSignal::new(vec!["yellow".to_string()]);
    let size = RwSignal::new(vec![140.0]);
    let draft = RwSignal::new(String::new());

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

    // Where a client position falls on the board, in world units. The canvas works this out for a
    // click of its own; a release that began in the sidebar is not one, so the block does the same
    // arithmetic against the box it wrapped the canvas in.
    let world_at = move |client_x: f64, client_y: f64| -> Option<CanvasPoint> {
        let rect = board.get_untracked()?.get_bounding_client_rect();
        if client_x < rect.left()
            || client_x > rect.right()
            || client_y < rect.top()
            || client_y > rect.bottom()
        {
            return None;
        }
        let (x, y) = viewport
            .get_untracked()
            .screen_to_world(client_x - rect.left(), client_y - rect.top());
        Some(CanvasPoint { x, y })
    };

    let place = move |kind: Kind, at: CanvasPoint| {
        let (w, h) = kind.size(picked.get_untracked());
        let id = next.get_untracked();
        let typed = draft.with_untracked(|draft| draft.trim().to_string());
        let label = if typed.is_empty() {
            format!("{} {id}", kind.label())
        } else {
            typed
        };

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
                color: current_color.get_untracked(),
                label,
            })
        });
        next.update(|next| *next += 1);
        selected.set(Some(id));
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
            if let Some(at) = world_at(end.point.client_x, end.point.client_y) {
                place(kind, at);
            }
        });

    let delete_selected = move || {
        if let Some(id) = selected.get_untracked() {
            items.update(|items| items.retain(|item| item.id != id));
            selected.set(None);
        }
    };

    let duplicate_selected = move || {
        let Some(id) = selected.get_untracked() else {
            return;
        };
        let copy = items.with_untracked(|items| items.iter().find(|item| item.id == id).cloned());
        let Some(mut copy) = copy else {
            return;
        };
        let fresh = next.get_untracked();
        copy.id = fresh;
        copy.x += 24.0;
        copy.y += 24.0;
        items.update(|items| items.push(copy));
        next.update(|next| *next += 1);
        selected.set(Some(fresh));
    };

    // Z-order is DOM order, so "bring to front" is a move to the end of the list and nothing else.
    let raise_selected = move || {
        let Some(id) = selected.get_untracked() else {
            return;
        };
        items.update(|items| {
            if let Some(at) = items.iter().position(|item| item.id == id) {
                let item = items.remove(at);
                items.push(item);
            }
        });
    };

    let chosen = Signal::derive(move || {
        selected
            .get()
            .and_then(|id| items.with(|items| items.iter().find(|item| item.id == id).cloned()))
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

                        <div class="flex flex-col gap-1.5">
                            <Label class="text-xs text-muted-foreground">"Label"</Label>
                            <Input
                                model=draft
                                attr:placeholder="Numbered if empty"
                                class="h-8 bg-background text-sm"
                            />
                        </div>
                    </div>

                    // The inspector is pinned rather than scrolled with the palette: what is
                    // selected, and what you can do to it, is the one thing that must never be
                    // below the fold.
                    <div class="flex flex-col gap-2 border-t p-3">
                    {move || match chosen.get() {
                            None => {
                                view! {
                                    <p class="text-xs leading-relaxed text-muted-foreground">
                                        "Nothing selected. Drag something off the palette above, then press it."
                                    </p>
                                }
                                    .into_any()
                            }
                            Some(item) => {
                                view! {
                                    <div class="flex items-center justify-between gap-2">
                                        <span class="truncate text-sm font-medium">
                                            {item.label}
                                        </span>
                                        <Badge variant=BadgeVariant::Secondary>
                                            {item.kind.label()}
                                        </Badge>
                                    </div>
                                    <span class="font-mono text-[0.7rem] text-muted-foreground tabular-nums">
                                        {format!(
                                            "x {:.0}  y {:.0}  {:.0}×{:.0}",
                                            item.x,
                                            item.y,
                                            item.w,
                                            item.h,
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
                            }
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
                                selected.set(None);
                            }
                        >
                            "Clear"
                        </Button>
                    </div>
                </aside>

                <div node_ref=board class="relative flex-1">
                    <Canvas
                        viewport=viewport
                        // A press on the board is a click, and a click on nothing is a
                        // deselection — a gesture a surface that panned on press has nowhere
                        // to put.
                        on_surface_click=Callback::new(move |_: CanvasPoint| selected.set(None))
                        class="h-full rounded-none border-0"
                        on:keydown=move |e: ev::KeyboardEvent| {
                            if matches!(e.key().as_str(), "Delete" | "Backspace") {
                                e.prevent_default();
                                delete_selected();
                            }
                        }
                    >
                        <Nodes items=items selected=selected />
                    </Canvas>
                </div>
            </div>

            <p class="text-xs text-muted-foreground">
                "Drag a shape off the palette and let go over the board. Press one to select it,
                drag its grips to resize, double-click its label to rename. "
                <Kbd>"Space"</Kbd> " and drag to pan, " <Kbd>"⌫"</Kbd> " to delete."
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
        "input",
        "kbd",
        "label",
        "separator",
        "slider",
        "toggle-group",
    ],
    description: "A palette down the side, and you make a thing by dragging it off and letting go over \
                  the board. Then move it, drag its grips to resize it, and double-click its \
                  label to rename it. This is the block the canvas' default gesture exists for: \
                  a surface \
                  that pans on the primary button has no press left to select a node with, none to \
                  drag one by, and nowhere to put a click on empty space that means \"deselect\". \
                  The drop crosses a border no context reaches — the palette sits outside the \
                  canvas — so it is done with the viewport signal and the box the canvas was \
                  wrapped in, which is the same arithmetic `on_surface_click` does on the inside. \
                  `use_drag` listens on the window, so one gesture spans both panes, and the thing \
                  following the pointer is drawn at the size and zoom it will land at. Everything \
                  after the drop is `on_move`, `on_resize` and `CanvasNodeLabel`: the block writes \
                  no pointer code for the things on the board, because the arithmetic a node needs \
                  — a screen delta divided by the zoom — wants the canvas' context, and the node \
                  is already in it. The nodes are keyed on their ids rather than on their values, \
                  so a drag rewriting the list forty times a second changes the numbers in one \
                  `style` attribute instead of rebuilding the board.",
    code: EDITOR,
    view: editor,
}];
