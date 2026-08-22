use crate::core::position::Position;

#[derive(Debug)]
pub struct Selector {
    position: Position,
    board_height: usize,
    board_width: usize,
}

impl Selector {
    pub fn new(board_width: usize, board_height: usize) -> Self {
        assert!(
            board_height > 0,
            "Board dimensions must be positive non-zero"
        );
        assert!(
            board_width > 0,
            "Board dimensions must be positive non-zero"
        );

        Selector {
            position: (0, 0).into(),
            board_height,
            board_width,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn move_up(&mut self) {
        self.position -= (0, 1);
    }
    pub fn move_down(&mut self) {
        if self.position.y() + 1 < self.board_height {
            self.position += (0, 1);
        }
    }
    pub fn move_left(&mut self) {
        self.position -= (1, 0);
    }
    pub fn move_right(&mut self) {
        if self.position.x() + 1 < self.board_width {
            self.position += (1, 0);
        }
    }
}
