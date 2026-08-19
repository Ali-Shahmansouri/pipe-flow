use crate::core::rotation::Rotation;

pub mod block;
pub mod l_shaped;
pub mod straight;

pub trait Cell {
    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell>;
}
pub trait RotatableCell: Cell {
    fn rotation(&self) -> Rotation;
    fn can_rotate(&self) -> bool;
    fn rotate_clockwise(&mut self);
}

pub struct StraightCell {
    rotation: Rotation,
    can_rotate: bool,
}

pub struct BlockCell {
    rotation: Rotation,
}
