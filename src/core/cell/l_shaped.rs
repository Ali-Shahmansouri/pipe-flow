use crate::core::{
    cell::{Cell, FlowState, RotatableCell, connections::Connections},
    rotation::Rotation,
};

pub struct LShapedCell {
    rotation: Rotation,
    fixed: bool,
    connections: Connections,
    flow_state: FlowState,
}

impl LShapedCell {
    pub fn new(rotation: Rotation, fixed: bool) -> Self {
        let connections = Self::connections_for(rotation);
        let flow_state = Self::flow_state_for(rotation);

        Self {
            rotation,
            fixed,
            connections,
            flow_state,
        }
    }

    fn connections_for(rotation: Rotation) -> Connections {
        match rotation {
            Rotation::Zero => Connections::up_right(),
            Rotation::Ninety => Connections::right_down(),
            Rotation::OneEighty => Connections::down_left(),
            Rotation::TwoSeventy => Connections::left_up(),
        }
    }

    fn flow_state_for(rotation: Rotation) -> FlowState {
        FlowState::NotConnected
    }
}

impl Cell for LShapedCell {
    fn connections(&self) -> Connections {
        self.connections
    }

    fn flow_state(&self) -> FlowState {
        self.flow_state
    }

    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell> {
        Some(self)
    }
}

impl RotatableCell for LShapedCell {
    fn rotation(&self) -> Rotation {
        self.rotation
    }
    fn rotation_mut(&mut self) -> &mut Rotation {
        &mut self.rotation
    }

    fn can_rotate(&self) -> bool {
        !self.fixed
    }

    fn update_connections_after_rotation(&mut self) {
        self.connections = Self::connections_for(self.rotation);
    }

    fn update_flow_state_after_rotation(&mut self) {
        self.flow_state = Self::flow_state_for(self.rotation);
    }
}
