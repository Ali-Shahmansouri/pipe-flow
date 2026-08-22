use crate::core::cell::connections::Connections;

pub struct CellVisual {
    pub rows: [String; 3],
}

impl CellVisual {
    pub fn new(rows: [String; 3]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[String; 3] {
        &self.rows
    }
}

impl CellVisual {
    pub fn block() -> Self {
        Self {
            rows: ["█████".into(), "█████".into(), "█████".into()],
        }
    }

    pub fn from_connections(connections: Connections) -> Self {
        let mut rows = [[' '; 5], [' '; 5], [' '; 5]];

        // Horizontal connection
        if connections.left() {
            rows[1][0] = '─';
            rows[1][1] = '─';
        }

        if connections.right() {
            rows[1][3] = '─';
            rows[1][4] = '─';
        }

        // Vertical connection
        if connections.up() {
            rows[0][2] = '│';
        }

        if connections.down() {
            rows[2][2] = '│';
        }

        let center = match (
            connections.up(),
            connections.right(),
            connections.down(),
            connections.left(),
        ) {
            (true, true, false, false) => '└',
            (false, true, true, false) => '┌',
            (false, false, true, true) => '┐',
            (true, false, false, true) => '┘',

            (false, true, false, true) => '─',
            (true, false, true, false) => '│',

            (false, false, false, false) => ' ',

            _ => '┼',
        };

        rows[1][2] = center;

        Self {
            rows: [
                rows[0].iter().collect(),
                rows[1].iter().collect(),
                rows[2].iter().collect(),
            ],
        }
    }
}
