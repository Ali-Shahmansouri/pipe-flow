use std::collections::HashSet;

use crate::{
    action::Action,
    core::{
        board::Board,
        flow_detector::{DFSFlowDetector, FlowDetector},
        position::Position,
        selector::Selector,
    },
};

pub struct AppState {
    should_quit: bool,
    board: Board,
    selector: Selector,
    connected_cells: HashSet<Position>,
}

impl AppState {
    pub fn new() -> Self {
        let mut board = Board::demo();

        let connected_cells = DFSFlowDetector::detect(&board);
        board.update_flow_states(&connected_cells);

        Self {
            should_quit: false,
            selector: Selector::new(board.width() as usize, board.height() as usize),
            board,
            connected_cells,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    pub fn connected_cells(&self) -> &HashSet<Position> {
        &self.connected_cells
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

        let rotated = self.board.rotate_cell(position);

        if rotated {
            self.recalculate_flow();
        }
    }

    fn recalculate_flow(&mut self) {
        self.connected_cells = DFSFlowDetector::detect(&self.board);
        self.board.update_flow_states(&self.connected_cells);
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}
