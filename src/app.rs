use crate::action::Action;

pub struct AppState {
    should_quit: bool,
    board: Board,
}

#[derive(Default)]
pub struct Board;

impl AppState {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            board: Board,
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.quit();
            }

            Action::MoveSelectorUp => {
                println!("move up");
            }

            Action::MoveSelectorDown => {
                println!("move down");
            }

            Action::MoveSelectorLeft => {
                println!("move left");
            }

            Action::MoveSelectorRight => {
                println!("move right");
            }

            Action::RotateClockwise => {
                println!("rotate");
            }
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
