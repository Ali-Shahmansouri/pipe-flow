use crate::core::{
    cell::{Cell, block::BlockCell, l_shaped::LShapedCell, straight::StraightCell},
    position::Position,
    rotation::Rotation,
};

pub struct Board {
    width: u16,
    height: u16,
    cells: Vec<Box<dyn Cell>>,
}

impl Board {
    pub fn new(width: u16, height: u16, cells: Vec<Box<dyn Cell>>) -> Self {
        assert_eq!(
            cells.len(),
            width as usize * height as usize,
            "number of cells must match board dimensions"
        );

        Self {
            width,
            height,
            cells,
        }
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

    pub fn cell_mut(&mut self, position: Position) -> Option<&mut dyn Cell> {
        let index = self.index(position)?;

        Some(self.cells[index].as_mut())
    }

    pub fn rotate_cell(&mut self, position: Position) {
        if let Some(cell) = self.cell_mut(position) {
            if let Some(rotatable) = cell.as_rotatable() {
                rotatable.rotate_clockwise();
            }
        }
    }

    fn index(&self, position: Position) -> Option<usize> {
        if position.x() >= self.width as usize || position.y() >= self.height as usize {
            return None;
        }

        Some(position.y() * self.width as usize + position.x())
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
            Box::new(StraightCell::new(Rotation::Ninety, false)),
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

        Self::new(5, 3, cells)
    }
}
