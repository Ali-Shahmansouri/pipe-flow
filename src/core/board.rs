use std::collections::HashSet;

use crate::core::{
    board_generator::RandomBoardGenator,
    cell::{Cell, FlowState, block::BlockCell, l_shaped::LShapedCell, straight::StraightCell},
    position::Position,
    rotation::Rotation,
};

pub struct Board {
    width: u16,
    height: u16,
    cells: Vec<Box<dyn Cell>>,
    source_position: Position,
    destination_position: Position,
}

impl Board {
    pub fn new(
        width: u16,
        height: u16,
        cells: Vec<Box<dyn Cell>>,
        source_position: Position,
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
            source_position,
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

    pub fn source_position(&self) -> Position {
        self.source_position
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

    pub fn demo() -> Self {
        let cells: Vec<Box<dyn Cell>> = vec![
            // row 0
            Box::new(LShapedCell::new(Rotation::Zero, false)),
            Box::new(StraightCell::new(Rotation::Zero, false)),
            Box::new(BlockCell::new()),
            Box::new(LShapedCell::new(Rotation::Ninety, true)),
            Box::new(StraightCell::new(Rotation::Ninety, false)),
            // row 1
            Box::new(LShapedCell::new(Rotation::Ninety, false)),
            Box::new(LShapedCell::new(Rotation::TwoSeventy, false)),
            Box::new(BlockCell::new()),
            Box::new(StraightCell::new(Rotation::Zero, true)),
            Box::new(LShapedCell::new(Rotation::OneEighty, false)),
            // row 2
            Box::new(BlockCell::new()),
            Box::new(StraightCell::new(Rotation::Zero, false)),
            Box::new(LShapedCell::new(Rotation::Zero, false)),
            Box::new(BlockCell::new()),
            Box::new(LShapedCell::new(Rotation::TwoSeventy, true)),
        ];

        Self::new(5, 3, cells, Position::new(0, 0), Position::new(4, 2))
    }
}
