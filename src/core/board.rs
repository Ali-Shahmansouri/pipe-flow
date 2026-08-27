use std::collections::HashSet;

use crate::core::{
    board_generator::RandomBoardGenator,
    cell::{Cell, FlowState},
    position::Position,
};

pub struct Board {
    width: u16,
    height: u16,
    cells: Vec<Box<dyn Cell>>,
    start_position: Position,
    destination_position: Position,
}

impl Board {
    pub fn new(
        width: u16,
        height: u16,
        cells: Vec<Box<dyn Cell>>,
        start_position: Position,
        destination_position: Position,
    ) -> Self {
        assert_eq!(
            cells.len(),
            width as usize * height as usize,
            "number of cells must match board dimensions"
        );

        Self {
            width,
            height,
            cells,
            start_position,
            destination_position,
        }
    }

    pub fn generate_random<R: RandomBoardGenator>(width: u16, height: u16) -> Self {
        R::generate(width, height)
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn cell(&self, position: Position) -> Option<&dyn Cell> {
        let index = self.index(position)?;

        Some(self.cells[index].as_ref())
    }

    pub fn start_position(&self) -> Position {
        self.start_position
    }

    pub fn destination_position(&self) -> Position {
        self.destination_position
    }

    pub fn cell_mut(&mut self, position: Position) -> Option<&mut dyn Cell> {
        let index = self.index(position)?;

        Some(self.cells[index].as_mut())
    }

    pub fn rotate_cell(&mut self, position: Position) -> bool {
        if let Some(cell) = self.cell_mut(position) {
            if let Some(rotatable) = cell.as_rotatable() {
                return rotatable.rotate_clockwise();
            }
        }

        return false;
    }

    fn index(&self, position: Position) -> Option<usize> {
        if position.x() >= self.width as usize || position.y() >= self.height as usize {
            return None;
        }

        Some(position.y() * self.width as usize + position.x())
    }

    pub fn update_flow_states(&mut self, connected: &HashSet<Position>) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let position = Position::new(x as usize, y as usize);

                if let Some(cell) = self.cell_mut(position) {
                    let flow_state = if connected.contains(&position) {
                        FlowState::Flowing
                    } else {
                        FlowState::Dry
                    };

                    cell.set_flow_state(flow_state);
                }
            }
        }
    }
}
