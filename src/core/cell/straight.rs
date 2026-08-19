use crate::core::{
    cell::{Cell, RotatableCell, visual::CellVisual},
    rotation::Rotation,
};

pub struct StraightCell {
    rotation: Rotation,
    fixed: bool,
    visual: CellVisual,
}

impl StraightCell {
    pub fn new(rotation: Rotation, fixed: bool) -> Self {
        let visual = Self::visual_for(rotation);

        Self {
            rotation,
            fixed,
            visual,
        }
    }

    fn visual_for(rotation: Rotation) -> CellVisual {
        match rotation {
            Rotation::Zero | Rotation::OneEighty => {
                CellVisual::new(["     ".into(), "████ ".into(), "     ".into()])
            }

            Rotation::Ninety | Rotation::TwoSeventy => {
                CellVisual::new(["  █  ".into(), "  █  ".into(), "  █  ".into()])
            }
        }
    }
}

impl Cell for StraightCell {
    fn visual(&self) -> &CellVisual {
        &self.visual
    }

    fn update_visual(&mut self) {
        self.visual = Self::visual_for(self.rotation);
    }

    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell> {
        Some(self)
    }
}

impl RotatableCell for StraightCell {
    fn rotation(&self) -> Rotation {
        self.rotation
    }

    fn rotation_mut(&mut self) -> &mut Rotation {
        &mut self.rotation
    }

    fn can_rotate(&self) -> bool {
        !self.fixed
    }
}
