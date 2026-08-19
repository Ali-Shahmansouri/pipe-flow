use crate::core::position::Position;

#[derive(Debug)]
pub struct Selector {
    position: Position,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            position: (0, 0).into(),
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn move_up(&mut self) {
        self.position += (0, 1);
    }
    pub fn move_down(&mut self) {
        self.position -= (0, 1);
    }
    pub fn move_left(&mut self) {
        self.position -= (1, 0);
    }
    pub fn move_right(&mut self) {
        self.position += (1, 0);
    }
}
