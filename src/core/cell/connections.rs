#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Connections {
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

impl Connections {
    pub fn new(up: bool, down: bool, right: bool, left: bool) -> Self {
        Self {
            up,
            down,
            right,
            left,
        }
    }

    pub fn none() -> Self {
        Self {
            up: false,
            down: false,
            right: false,
            left: false,
        }
    }

    pub fn up_right() -> Self {
        Self::new(true, false, true, false)
    }

    pub fn right_down() -> Self {
        Self::new(false, true, true, false)
    }

    pub fn down_left() -> Self {
        Self::new(false, true, false, true)
    }

    pub fn left_up() -> Self {
        Self::new(true, false, false, true)
    }

    pub fn left_right() -> Self {
        Self::new(false, false, true, true)
    }

    pub fn up_down() -> Self {
        Self::new(true, true, false, false)
    }

    pub fn up(&self) -> bool {
        self.up
    }

    pub fn down(&self) -> bool {
        self.down
    }

    pub fn right(&self) -> bool {
        self.right
    }

    pub fn left(&self) -> bool {
        self.left
    }
}
