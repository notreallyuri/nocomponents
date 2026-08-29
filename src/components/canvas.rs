use crate::{
    cn,
    icons::{media::Maximize, minus::Minus, plus::Plus},
    primitives::canvas::{
        CanvasBackgroundRoot, CanvasDrag, CanvasGesture, CanvasMarqueeRoot, CanvasNodeLabelRoot,
        CanvasNodeRoot, CanvasPoint, CanvasRoot, CanvasViewportRoot, use_canvas,
    },
    utils::{
        types::AsClass,
        viewport::{Viewport, WorldBox, ZoomLimits},
    },
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum StickyColor {
    #[default]
    Yellow,
    Green,
    Blue,
    Pink,
}

impl AsClass for StickyColor {
    fn as_class(&self) -> &'static str {
        match self {
            StickyColor::Yellow => "bg-amber-200 text-amber-950",
            StickyColor::Green => "bg-emerald-200 text-emerald-950",
            StickyColor::Blue => "bg-sky-200 text-sky-950",
            StickyColor::Pink => "bg-pink-200 text-pink-950",
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShapeKind {
    #[default]
    Rectangle,
    Ellipse,
}

impl AsClass for ShapeKind {
    fn as_class(&self) -> &'static str {
        match self {
            ShapeKind::Rectangle => "rounded-md",
            ShapeKind::Ellipse => "rounded-full",
        }
    }
}

#[component]
pub fn Canvas(
    #[prop(optional, into)] viewport: RwSignal<Viewport>,
    #[prop(optional)] limits: ZoomLimits,
    #[prop(default = false)] pan_on_drag: bool,
    #[prop(default = None, into)] on_surface_click: Option<Callback<CanvasPoint>>,
    #[prop(default = None, into)] on_surface_drag: Option<Callback<CanvasDrag>>,
    #[prop(default = true)] controls: bool,
    #[prop(default = 24.0)] gap: f64,
    #[prop(optional)] node_ref: Option<AnyNodeRef>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_default();

    view! {
        <CanvasRoot
            node_ref=node_ref
            viewport=viewport
            limits=limits
            pan_on_drag=pan_on_drag
            on_surface_click=on_surface_click
            on_surface_drag=on_surface_drag
            class=move || {
                cn!(
                    "relative touch-none overflow-hidden rounded-lg border bg-muted/20 outline-none select-none",
                    "cursor-default data-[pan=ready]:cursor-grab data-[pan=active]:cursor-grabbing",
                    "focus-visible:ring-2 focus-visible:ring-ring/50",
                    class.get(),
                )
            }
        >
            <CanvasBackground gap=gap />
            <CanvasViewportRoot class="absolute inset-0 will-change-transform">
                {children()}
            </CanvasViewportRoot>
            {controls.then(|| view! { <CanvasControls /> })}
        </CanvasRoot>
    }
}

#[component]
pub fn CanvasBackground(
    #[prop(default = 24.0)] gap: f64,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <CanvasBackgroundRoot
            gap=gap
            class=move || {
                cn!(
                    "pointer-events-none absolute inset-0 bg-[radial-gradient(circle,var(--color-border)_1px,transparent_1px)]",
                    class.get(),
                )
            }
        />
    }
}

#[component]
pub fn CanvasMarquee(
    #[prop(into)] rect: Signal<Option<WorldBox>>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <CanvasMarqueeRoot
            rect=rect
            class=move || {
                cn!(
                    "border-[length:calc(1px/var(--canvas-zoom))] border-primary bg-primary/10",
                    class.get(),
                )
            }
        />
    }
}

#[component]
pub fn CanvasControls(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let ctx = use_canvas();
    let button = "inline-flex size-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring/50 focus-visible:outline-none";

    view! {
        <div
            data-slot="canvas-controls"
            class=move || {
                cn!(
                    "absolute right-3 bottom-3 flex items-center gap-0.5 rounded-lg border bg-background/90 p-1 shadow-sm backdrop-blur",
                    class.get(),
                )
            }
        >
            <button
                type="button"
                aria-label="Zoom out"
                class=button
                on:click=move |_| ctx.zoom_out()
            >
                <Minus class="size-3.5" />
            </button>
            <button
                type="button"
                aria-label="Reset zoom"
                class="min-w-12 rounded-md px-1 text-center font-mono text-xs tabular-nums text-muted-foreground transition-colors hover:bg-muted hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring/50 focus-visible:outline-none"
                on:click=move |_| ctx.reset()
            >
                {move || format!("{}%", (ctx.viewport.get().zoom * 100.0).round())}
            </button>
            <button type="button" aria-label="Zoom in" class=button on:click=move |_| ctx.zoom_in()>
                <Plus class="size-3.5" />
            </button>
            <div class="mx-0.5 h-4 w-px bg-border" />
            <button
                type="button"
                aria-label="Fit to view"
                class=button
                on:click=move |_| ctx.fit_content(48.0)
            >
                <Maximize class="size-3.5" />
            </button>
        </div>
    }
}

#[component]
pub fn CanvasNode(
    #[prop(into)] x: Signal<f64>,
    #[prop(into)] y: Signal<f64>,
    #[prop(default = None, into)] width: Option<Signal<f64>>,
    #[prop(default = None, into)] height: Option<Signal<f64>>,
    #[prop(default = None, into)] on_move: Option<Callback<CanvasPoint>>,
    #[prop(default = None, into)] on_resize: Option<Callback<WorldBox>>,
    #[prop(default = None, into)] on_gesture_end: Option<Callback<CanvasGesture>>,
    #[prop(default = None, into)] aspect: Option<f64>,
    #[prop(optional, into)] angle: Signal<f64>,
    #[prop(default = 24.0)] min_size: f64,
    #[prop(optional, into)] z: Signal<i32>,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let movable = on_move.is_some();

    view! {
        <CanvasNodeRoot
            x=x
            y=y
            width=width
            height=height
            on_move=on_move
            on_resize=on_resize
            on_gesture_end=on_gesture_end
            aspect=aspect
            angle=angle
            min_size=min_size
            z=z
            selected=selected
            class=move || {
                cn!(
                    "select-none",
                    if movable { "cursor-grab data-[dragging=true]:cursor-grabbing" } else { "" },
                    "data-[selected=true]:outline-primary data-[selected=true]:outline-[length:calc(2px/var(--canvas-zoom))] data-[selected=true]:outline-offset-[calc(1px/var(--canvas-zoom))]",
                    "[&>[data-slot=canvas-node-handle]]:rounded-[calc(2px/var(--canvas-zoom))] [&>[data-slot=canvas-node-handle]]:border-[length:calc(1px/var(--canvas-zoom))] [&>[data-slot=canvas-node-handle]]:border-primary [&>[data-slot=canvas-node-handle]]:bg-background",
                    "[&>[data-slot=canvas-node-handle][data-handle=top-left]]:cursor-nwse-resize [&>[data-slot=canvas-node-handle][data-handle=bottom-right]]:cursor-nwse-resize",
                    "[&>[data-slot=canvas-node-handle][data-handle=top-right]]:cursor-nesw-resize [&>[data-slot=canvas-node-handle][data-handle=bottom-left]]:cursor-nesw-resize",
                    "[&>[data-slot=canvas-node-handle][data-handle=top]]:cursor-ns-resize [&>[data-slot=canvas-node-handle][data-handle=bottom]]:cursor-ns-resize",
                    "[&>[data-slot=canvas-node-handle][data-handle=left]]:cursor-ew-resize [&>[data-slot=canvas-node-handle][data-handle=right]]:cursor-ew-resize",
                    "[&:not([data-selected=true])>[data-slot=canvas-node-handle]]:hidden",
                    class.get(),
                )
            }
        >
            {children.map(|children| children())}
        </CanvasNodeRoot>
    }
}

#[component]
pub fn CanvasNodeLabel(
    #[prop(into)] value: Signal<String>,
    #[prop(default = None, into)] on_change: Option<Callback<String>>,
    #[prop(optional, into)] editing: RwSignal<bool>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <CanvasNodeLabelRoot
            value=value
            on_change=on_change
            editing=editing
            class=move || {
                cn!(
                    "block w-full min-w-0 border-0 bg-transparent p-0 whitespace-pre-wrap outline-none [font:inherit] text-inherit [text-align:inherit]",
                    "data-[editing=true]:cursor-text data-[editing=true]:field-sizing-content data-[editing=true]:resize-none data-[editing=true]:overflow-hidden",
                    class.get(),
                )
            }
        />
    }
}

#[component]
pub fn CanvasSticky(
    #[prop(into)] x: Signal<f64>,
    #[prop(into)] y: Signal<f64>,
    #[prop(default = Signal::stored(160.0), into)] size: Signal<f64>,
    #[prop(optional)] color: StickyColor,
    #[prop(default = None, into)] on_move: Option<Callback<CanvasPoint>>,
    #[prop(default = None, into)] on_resize: Option<Callback<WorldBox>>,
    #[prop(default = None, into)] on_gesture_end: Option<Callback<CanvasGesture>>,
    #[prop(optional, into)] angle: Signal<f64>,
    #[prop(default = 60.0)] min_size: f64,
    #[prop(optional, into)] z: Signal<i32>,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <CanvasNode
            x=x
            y=y
            width=size
            height=size
            on_move=on_move
            on_resize=on_resize
            on_gesture_end=on_gesture_end
            aspect=1.0
            angle=angle
            min_size=min_size
            z=z
            selected=selected
            class=move || {
                cn!(
                    "flex flex-col gap-1 rounded-sm p-3 text-sm leading-snug shadow-md",
                    color.as_class(),
                    class.get(),
                )
            }
        >
            {children()}
        </CanvasNode>
    }
}

#[component]
pub fn CanvasShape(
    #[prop(into)] x: Signal<f64>,
    #[prop(into)] y: Signal<f64>,
    #[prop(default = Signal::stored(160.0), into)] width: Signal<f64>,
    #[prop(default = Signal::stored(100.0), into)] height: Signal<f64>,
    #[prop(optional)] kind: ShapeKind,
    #[prop(default = None, into)] on_move: Option<Callback<CanvasPoint>>,
    #[prop(default = None, into)] on_resize: Option<Callback<WorldBox>>,
    #[prop(default = None, into)] on_gesture_end: Option<Callback<CanvasGesture>>,
    #[prop(default = None, into)] aspect: Option<f64>,
    #[prop(optional, into)] angle: Signal<f64>,
    #[prop(default = 32.0)] min_size: f64,
    #[prop(optional, into)] z: Signal<i32>,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <CanvasNode
            x=x
            y=y
            width=width
            height=height
            on_move=on_move
            on_resize=on_resize
            on_gesture_end=on_gesture_end
            aspect=aspect
            angle=angle
            min_size=min_size
            z=z
            selected=selected
            class=move || {
                cn!(
                    "flex items-center justify-center border-2 border-primary/60 bg-primary/10 p-2 text-center text-sm text-foreground",
                    kind.as_class(),
                    class.get(),
                )
            }
        >
            {children.map(|children| children())}
        </CanvasNode>
    }
}

#[component]
pub fn CanvasText(
    #[prop(into)] x: Signal<f64>,
    #[prop(into)] y: Signal<f64>,
    #[prop(default = None, into)] on_move: Option<Callback<CanvasPoint>>,
    #[prop(default = None, into)] on_gesture_end: Option<Callback<CanvasGesture>>,
    #[prop(optional, into)] angle: Signal<f64>,
    #[prop(optional, into)] z: Signal<i32>,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <CanvasNode
            x=x
            y=y
            on_move=on_move
            on_gesture_end=on_gesture_end
            angle=angle
            z=z
            selected=selected
            class=move || cn!("text-sm font-medium whitespace-nowrap text-foreground", class.get())
        >
            {children()}
        </CanvasNode>
    }
}
