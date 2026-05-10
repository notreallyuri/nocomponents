#[derive(Default, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    #[default]
    Right,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum State {
    Open,
    #[default]
    Closed,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Side {
    Left,
    Right,
    #[default]
    Bottom,
    Top,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SideOffset(pub u64);

impl Default for SideOffset {
    fn default() -> Self {
        SideOffset(4)
    }
}
