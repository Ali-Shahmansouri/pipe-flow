use crate::core::{
    cell::{self, Cell},
    position::{self, Position},
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
}
