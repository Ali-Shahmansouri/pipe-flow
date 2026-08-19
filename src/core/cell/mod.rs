use crate::core::{
    cell::{
        renderer::{CellRenderer, RenderArea},
        visual::CellVisual,
    },
    rotation::Rotation,
};

pub mod block;
pub mod l_shaped;
pub mod renderer;
pub mod straight;
pub mod visual;

pub trait Cell {
    fn visual(&self) -> &CellVisual;
    fn update_visual(&mut self);

    fn render(&self, renderer: &mut dyn CellRenderer, area: RenderArea) {
        renderer.render(self.visual(), area);
    }

    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell>;
}
pub trait RotatableCell: Cell {
    fn rotation(&self) -> Rotation;
    fn rotation_mut(&mut self) -> &mut Rotation;

    fn can_rotate(&self) -> bool;

    fn rotate_clockwise(&mut self) {
        if self.can_rotate() {
            self.rotation_mut().clockwise();
            self.update_visual();
        }
    }
}
