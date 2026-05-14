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
