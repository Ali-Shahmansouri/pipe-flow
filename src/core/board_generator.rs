use std::collections::{HashMap, HashSet};

use rand::RngExt;

use crate::core::{
    board::Board,
    cell::{
        Cell, block::BlockCell, connections::Connections, l_shaped::LShapedCell,
        straight::StraightCell,
    },
    position::{Direction, Position},
    rotation::Rotation,
};

use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

pub trait RandomBoardGenator {
    fn generate(width: u16, height: u16) -> Board;
}

pub struct BackwardsBoardGenerator;

impl BackwardsBoardGenerator {
    fn random_edge() -> Edge {
        let mut rng = rand::rng();

        match rng.random_range(0..4) {
            0 => Edge::Top,
            1 => Edge::Bottom,
            2 => Edge::Left,
            _ => Edge::Right,
        }
    }

    fn random_edge_position(width: u16, height: u16, edge: Edge) -> (usize, usize) {
        let mut rng = rand::rng();

        match edge {
            Edge::Top => {
                let x = rng.random_range(0..width as usize);
                (x, 0)
            }

            Edge::Bottom => {
                let x = rng.random_range(0..width as usize);
                (x, height as usize - 1)
            }

            Edge::Left => {
                let y = rng.random_range(0..height as usize);
                (0, y)
            }

            Edge::Right => {
                let y = rng.random_range(0..height as usize);
                (width as usize - 1, y)
            }
        }
    }

    fn random_endpoints(width: u16, height: u16) -> (Position, Position) {
        let source_edge = Self::random_edge();
        let mut destination_edge = Self::random_edge();

        while destination_edge == source_edge {
            destination_edge = Self::random_edge();
        }

        let source = Self::random_edge_position(width, height, source_edge);

        let destination = Self::random_edge_position(width, height, destination_edge);

        (source.into(), destination.into())
    }

    fn generate_path(
        source: Position,
        destination: Position,
        width: u16,
        height: u16,
    ) -> Option<Vec<Position>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        Self::dfs(source, destination, width, height, &mut visited, &mut path).then_some(path)
    }

    fn dfs(
        current: Position,
        destination: Position,
        width: u16,
        height: u16,
        visited: &mut HashSet<Position>,
        path: &mut Vec<Position>,
    ) -> bool {
        visited.insert(current);
        path.push(current);

        if current == destination {
            return true;
        }

        let mut directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let mut rng = rand::rng();
        directions.shuffle(&mut rng);

        for direction in directions {
            let Some(next) = current.neighbor(direction) else {
                continue;
            };

            if next.x() >= width as usize || next.y() >= height as usize {
                continue;
            }

            if visited.contains(&next) {
                continue;
            }

            if Self::dfs(next, destination, width, height, visited, path) {
                return true;
            }
        }

        // This branch didn't lead to the destination.
        path.pop();
        visited.remove(&current);

        false
    }

    fn connections_for_path_position(path: &[Position], index: usize) -> Connections {
        let mut up = false;
        let mut down = false;
        let mut left = false;
        let mut right = false;

        let current = path[index];

        for neighbor in [path.get(index.wrapping_sub(1)), path.get(index + 1)]
            .into_iter()
            .flatten()
        {
            if neighbor.x() < current.x() {
                left = true;
            } else if neighbor.x() > current.x() {
                right = true;
            } else if neighbor.y() < current.y() {
                up = true;
            } else if neighbor.y() > current.y() {
                down = true;
            }
        }

        Connections::new(up, down, right, left)
    }

    fn outward_direction(position: Position, width: u16, height: u16) -> Option<Direction> {
        if position.y() == 0 {
            Some(Direction::Up)
        } else if position.y() == height as usize - 1 {
            Some(Direction::Down)
        } else if position.x() == 0 {
            Some(Direction::Left)
        } else if position.x() == width as usize - 1 {
            Some(Direction::Right)
        } else {
            None
        }
    }

    fn add_connection(connections: &mut Connections, direction: Direction) {
        match direction {
            Direction::Up => connections.set_up(true),
            Direction::Down => connections.set_down(true),
            Direction::Left => connections.set_left(true),
            Direction::Right => connections.set_right(true),
        }
    }

    fn cell_for_connections(connections: Connections, fixed: bool) -> Box<dyn Cell> {
        match (
            connections.up(),
            connections.down(),
            connections.right(),
            connections.left(),
        ) {
            // L shapes
            (true, false, true, false) => Box::new(LShapedCell::new(Rotation::Zero, fixed)),

            (false, true, true, false) => Box::new(LShapedCell::new(Rotation::Ninety, fixed)),

            (false, true, false, true) => Box::new(LShapedCell::new(Rotation::OneEighty, fixed)),

            (true, false, false, true) => Box::new(LShapedCell::new(Rotation::TwoSeventy, fixed)),

            // Straights
            (false, false, true, true) => Box::new(StraightCell::new(Rotation::Zero, fixed)),

            (true, true, false, false) => Box::new(StraightCell::new(Rotation::Ninety, fixed)),

            _ => panic!("invalid path connections: {:?}", connections),
        }
    }

    fn cells_from_path(
        path: &[Position],
        width: u16,
        height: u16,
        source: Position,
        destination: Position,
    ) -> HashMap<Position, Box<dyn Cell>> {
        let mut cells = HashMap::new();

        for (index, position) in path.iter().copied().enumerate() {
            let mut connections = Self::connections_for_path_position(path, index);

            if position == source || position == destination {
                if let Some(direction) = Self::outward_direction(position, width, height) {
                    Self::add_connection(&mut connections, direction);
                }
            }

            let fixed = position == source || position == destination;

            cells.insert(position, Self::cell_for_connections(connections, fixed));
        }

        cells
    }

    fn random_cell() -> Box<dyn Cell> {
        let mut rng = rand::rng();

        match rng.random_range(0..3) {
            0 => Box::new(BlockCell::new()),

            1 => Box::new(StraightCell::new(
                if rng.random_bool(0.5) {
                    Rotation::Zero
                } else {
                    Rotation::Ninety
                },
                false,
            )),

            _ => Box::new(LShapedCell::new(
                match rng.random_range(0..4) {
                    0 => Rotation::Zero,
                    1 => Rotation::Ninety,
                    2 => Rotation::OneEighty,
                    _ => Rotation::TwoSeventy,
                },
                false,
            )),
        }
    }

    fn populate_cells(
        width: u16,
        height: u16,
        mut path_cells: HashMap<Position, Box<dyn Cell>>,
    ) -> Vec<Box<dyn Cell>> {
        let mut cells = Vec::with_capacity(width as usize * height as usize);

        for y in 0..height as usize {
            for x in 0..width as usize {
                let position = Position::new(x, y);

                match path_cells.remove(&position) {
                    Some(cell) => cells.push(cell),
                    None => cells.push(Self::random_cell()),
                }
            }
        }

        cells
    }

    fn scramble(cells: &mut [Box<dyn Cell>], source: Position, destination: Position, width: u16) {
        let mut rng = rand::rng();

        for (index, cell) in cells.iter_mut().enumerate() {
            let x = index % width as usize;
            let y = index / width as usize;

            let position = Position::new(x, y);

            if position == source || position == destination {
                continue;
            }

            let rotations = rng.random_range(0..4);

            for _ in 0..rotations {
                if let Some(rotatable) = cell.as_rotatable() {
                    rotatable.rotate_clockwise();
                }
            }
        }
    }
}

impl RandomBoardGenator for BackwardsBoardGenerator {
    fn generate(width: u16, height: u16) -> Board {
        let (source, destination) = Self::random_endpoints(width, height);

        let path = Self::generate_path(source, destination, width, height)
            .expect("failed to generate path");

        let path_cells = Self::cells_from_path(&path, width, height, source, destination);

        let mut cells = Self::populate_cells(width, height, path_cells);

        Self::scramble(&mut cells, source, destination, width);

        Board::new(width, height, cells, source, destination)
    }
}
