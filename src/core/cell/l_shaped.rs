use crate::core::{
    cell::{Cell, RotatableCell},
    rotation::Rotation,
};

pub struct LShapedCell {
    rotation: Rotation,
    fixed: bool,
}

impl Cell for LShapedCell {
    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell> {
        Some(self)
    }
}

impl RotatableCell for LShapedCell {
    fn rotation(&self) -> Rotation {
        self.rotation
    }

    fn can_rotate(&self) -> bool {
        !self.fixed
    }

    fn rotate_clockwise(&mut self) {
        if self.can_rotate() {
            self.rotation.clockwise();
        }
    }
}
