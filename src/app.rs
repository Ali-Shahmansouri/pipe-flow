use crate::{
    action::Action,
    core::{board::Board, selector::Selector},
};

pub struct AppState {
    should_quit: bool,
    board: Board,
    selector: Selector,
}

impl AppState {
    pub fn new() -> Self {
        let board = Board::demo();
        Self {
            should_quit: false,
            selector: Selector::new(board.width() as usize, board.height() as usize),
            board,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.quit();
            }

            Action::MoveSelectorUp => {
                self.selector.move_up();
            }

            Action::MoveSelectorDown => {
                self.selector.move_down();
            }

            Action::MoveSelectorLeft => {
                self.selector.move_left();
            }

            Action::MoveSelectorRight => {
                self.selector.move_right();
            }

            Action::RotateClockwise => {
                self.rotate_selected_cell();
            }
        }
    }

    fn rotate_selected_cell(&mut self) {
        let position = *self.selector.position();

        if let Some(cell) = self.board.cell_mut(position) {
            if let Some(rotatable) = cell.as_rotatable() {
                rotatable.rotate_clockwise();
            }
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}
