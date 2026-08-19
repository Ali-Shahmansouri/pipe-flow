use crate::core::cell::{Cell, RotatableCell};

pub struct BlockCell;

impl Cell for BlockCell {
    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell> {
        None
    }
}
