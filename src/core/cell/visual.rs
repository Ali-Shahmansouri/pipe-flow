use crate::core::cell::{FlowState, connections::Connections};

pub struct CellVisual {
    pub rows: [String; 3],
}

impl CellVisual {
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

    pub fn from_connections(connections: Connections, style: FlowState) -> Self {
        let mut rows = [[' '; 5], [' '; 5], [' '; 5]];

        let horizontal = match style {
            FlowState::Dry => '─',
            FlowState::Flowing => '═',
        };

        let vertical = match style {
            FlowState::Dry => '│',
            FlowState::Flowing => '║',
        };

        // Horizontal connection
        if connections.left() {
            rows[1][0] = horizontal;
            rows[1][1] = horizontal;
        }

        if connections.right() {
            rows[1][3] = horizontal;
            rows[1][4] = horizontal;
        }

        // Vertical connection
        if connections.up() {
            rows[0][2] = vertical;
        }

        if connections.down() {
            rows[2][2] = vertical;
        }

        let center = match style {
            FlowState::Dry => match (
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
            },

            FlowState::Flowing => match (
                connections.up(),
                connections.right(),
                connections.down(),
                connections.left(),
            ) {
                (true, true, false, false) => '╚',
                (false, true, true, false) => '╔',
                (false, false, true, true) => '╗',
                (true, false, false, true) => '╝',

                (false, true, false, true) => '═',
                (true, false, true, false) => '║',

                (false, false, false, false) => ' ',

                _ => '╬',
            },
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
