use crate::core::cell::{Cell, RotatableCell, visual::CellVisual};

pub struct BlockCell {
    visual: CellVisual,
}

impl BlockCell {
    pub fn new() -> Self {
        Self {
            visual: CellVisual::new(["█████".into(), "█████".into(), "█████".into()]),
        }
    }
}

impl Cell for BlockCell {
    fn visual(&self) -> &CellVisual {
        &self.visual
    }

    fn update_visual(&mut self) {}

    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell> {
        None
    }
}
