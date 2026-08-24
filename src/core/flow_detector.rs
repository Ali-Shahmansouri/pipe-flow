use std::collections::HashSet;

use crate::core::{
    board::Board,
    position::{Direction, Position},
};

pub struct FlowDetector<'a> {
    board: &'a Board,
    source_position: Position,
}

impl<'a> FlowDetector<'a> {
    pub fn new(board: &'a Board, source_position: Position) -> Self {
        Self {
            board,
            source_position,
        }
    }

    pub fn detect(&self) -> HashSet<Position> {
        let mut connected = HashSet::new();
        let mut unchecked = vec![self.source_position];

        while let Some(position) = unchecked.pop() {
            if !connected.insert(position) {
                continue;
            }

            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Right,
                Direction::Left,
            ] {
                if let Some(neighbor) = self.connected_neighbor(position, direction) {
                    if !connected.contains(&neighbor) {
                        unchecked.push(neighbor);
                    }
                }
            }
        }

        connected
    }

    fn connected_neighbor(&self, position: Position, direction: Direction) -> Option<Position> {
        let cell = self.board.cell(position)?;

        let neighbor_position = position.neighbor(direction)?;
        let neighbor_cell = self.board.cell(neighbor_position)?;

        let connected = match direction {
            Direction::Up => cell.connections().up() && neighbor_cell.connections().down(),
            Direction::Down => cell.connections().down() && neighbor_cell.connections().up(),
            Direction::Right => cell.connections().right() && neighbor_cell.connections().left(),
            Direction::Left => cell.connections().left() && neighbor_cell.connections().right(),
        };

        connected.then_some(neighbor_position)
    }
}
