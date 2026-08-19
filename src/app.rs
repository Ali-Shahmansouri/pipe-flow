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
        Self {
            should_quit: false,
            board: Board::new(5, 5),
            selector: Selector::new(),
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
                println!("rotate");
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
