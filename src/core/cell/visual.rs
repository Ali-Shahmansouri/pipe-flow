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
