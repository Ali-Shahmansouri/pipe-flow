use crate::core::cell::{
    Cell, FlowState, RotatableCell, connections::Connections, visual::CellVisual,
};

pub struct BlockCell;

impl BlockCell {
    pub fn new() -> Self {
        Self
    }
}

impl Cell for BlockCell {
    fn visual(&self) -> CellVisual {
        CellVisual::block()
    }

    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell> {
        None
    }

    fn connections(&self) -> Connections {
        Connections::none()
    }

    fn flow_state(&self) -> FlowState {
        FlowState::NotConnected
    }
}
