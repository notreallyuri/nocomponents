use leptos::prelude::Signal;
use leptos_node_ref::AnyNodeRef;

#[derive(Default, Copy, Clone)]
pub enum Direction {
    Left,
    #[default]
    Right,
}

#[derive(Default, Copy, Clone)]
pub enum State {
    Open,
    #[default]
    Closed,
}

#[derive(Default, Copy, Clone)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default, Copy, Clone)]
pub enum Side {
    Left,
    Right,
    #[default]
    Bottom,
    Top,
}

#[derive(Default, Copy, Clone)]
pub struct TriggerRect {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone)]
pub struct SideOffset(pub f64);

impl Default for SideOffset {
    fn default() -> Self {
        SideOffset(4.0)
    }
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}

impl State {
    pub fn as_str(&self) -> &'static str {
        match self {
            State::Open => "open",
            State::Closed => "closed",
        }
    }
}

impl Orientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
    }
}

impl Align {
    pub fn as_str(&self) -> &'static str {
        match self {
            Align::Start => "start",
            Align::Center => "center",
            Align::End => "end",
        }
    }
}

impl Side {
    pub fn as_str(&self) -> &'static str {
        match self {
            Side::Left => "left",
            Side::Right => "right",
            Side::Bottom => "bottom",
            Side::Top => "top",
        }
    }
}

pub trait AsClass {
    fn as_class(&self) -> &'static str;
}

/// Language hint for `CodeBlock`. Only affects highlighting, which is a no-op unless the
/// `highlight` feature is enabled.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Language {
    #[default]
    Rust,
    Shell,
    JavaScript,
    TypeScript,
    Html,
    Css,
    Plain,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Shell => "shell",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Html => "html",
            Language::Css => "css",
            Language::Plain => "plain",
        }
    }
}

/// What a styled part's `render` is handed when the caller draws the element themselves: the
/// classes the part would have rendered, and the node ref its state attributes go on.
///
/// A struct for the reason [`crate::primitives::floating::TriggerRender`] is one — that shape had
/// to grow, and a tuple has to be re-broken every time it does. It has since grown a `disabled`,
/// and every caller written against the two fields kept compiling, which is the whole argument.
/// The two are still separate rather than merged: a trigger is handed a click to run, and a
/// styled row has none.
#[derive(Copy, Clone)]
pub struct StyledRender {
    pub class: Signal<String>,
    pub node_ref: AnyNodeRef,
    /// Whatever the part resolved it to — `Button` folds in the field around it. Worth reading
    /// rather than ignoring: the element is the caller's, and on an `<a>`, which is what `render`
    /// mostly returns, the `disabled` attribute means nothing at all.
    pub disabled: Signal<bool>,
}

/// What a layer that opens on hover hands its `render`.
///
/// Only the node ref, and that is the honest shape: nothing here is clicked, so there is no
/// callback to hand over, and the trigger wears none of the layer's classes.
#[derive(Copy, Clone)]
pub struct AnchorRender {
    pub node_ref: AnyNodeRef,
}
