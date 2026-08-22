use crate::core::{
    cell::{
        connections::Connections,
        renderer::{CellRenderer, RenderArea},
        visual::CellVisual,
    },
    rotation::Rotation,
};

pub mod block;
pub mod connections;
pub mod l_shaped;
pub mod renderer;
pub mod straight;
pub mod visual;

pub trait Cell {
    fn visual(&self) -> CellVisual {
        CellVisual::from_connections(self.connections())
    }

    fn render(&self, renderer: &mut dyn CellRenderer, area: RenderArea) {
        let visual = self.visual();
        renderer.render(&visual, area);
    }
    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell>;

    fn connections(&self) -> Connections;
    fn flow_state(&self) -> FlowState;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowState {
    NotConnected,
    Connected,
    Flowing,
}

pub trait RotatableCell: Cell {
    fn rotation(&self) -> Rotation;
    fn rotation_mut(&mut self) -> &mut Rotation;

    fn can_rotate(&self) -> bool;

    fn rotate_clockwise(&mut self) {
        if self.can_rotate() {
            self.rotation_mut().clockwise();
            self.update_after_rotation();
        }
    }

    fn update_after_rotation(&mut self) {
        self.update_connections_after_rotation();
        self.update_flow_state_after_rotation();
    }

    fn update_flow_state_after_rotation(&mut self);
    fn update_connections_after_rotation(&mut self);
}
