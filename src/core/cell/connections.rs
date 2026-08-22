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
