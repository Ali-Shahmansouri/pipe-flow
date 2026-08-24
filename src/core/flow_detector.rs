use std::collections::HashSet;

use crate::core::{
    board::Board,
    position::{Direction, Position},
};

pub trait FlowDetector {
    fn detect(board: &Board) -> HashSet<Position>;
}

pub struct DFSFlowDetector;

impl DFSFlowDetector {
    fn connected_neighbor(
        board: &Board,
        position: Position,
        direction: Direction,
    ) -> Option<Position> {
        let cell = board.cell(position)?;

        let neighbor_position = position.neighbor(direction)?;
        let neighbor_cell = board.cell(neighbor_position)?;

        let connected = match direction {
            Direction::Up => cell.connections().up() && neighbor_cell.connections().down(),
            Direction::Down => cell.connections().down() && neighbor_cell.connections().up(),
            Direction::Right => cell.connections().right() && neighbor_cell.connections().left(),
            Direction::Left => cell.connections().left() && neighbor_cell.connections().right(),
        };

        connected.then_some(neighbor_position)
    }
}

impl FlowDetector for DFSFlowDetector {
    fn detect(board: &Board) -> HashSet<Position> {
        let mut connected = HashSet::new();
        let mut unchecked = vec![board.source_position()];

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
                if let Some(neighbor) = Self::connected_neighbor(board, position, direction) {
                    if !connected.contains(&neighbor) {
                        unchecked.push(neighbor);
                    }
                }
            }
        }

        connected
    }
}
