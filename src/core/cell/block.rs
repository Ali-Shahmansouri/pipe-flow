use crate::core::cell::{
    Cell, FlowState, RotatableCell, connections::Connections, visual::CellVisual,
};

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
