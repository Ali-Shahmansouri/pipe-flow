use crate::core::{
    cell::{Cell, FlowState, RotatableCell, connections::Connections},
    rotation::Rotation,
};

pub struct StraightCell {
    rotation: Rotation,
    fixed: bool,
    connections: Connections,
    flow_state: FlowState,
}

impl StraightCell {
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
            Rotation::Zero | Rotation::OneEighty => Connections::left_right(),

            Rotation::Ninety | Rotation::TwoSeventy => Connections::up_down(),
        }
    }

    fn flow_state_for(rotation: Rotation) -> FlowState {
        FlowState::NotConnected
    }
}

impl Cell for StraightCell {
    fn as_rotatable(&mut self) -> Option<&mut dyn RotatableCell> {
        Some(self)
    }

    fn connections(&self) -> Connections {
        self.connections
    }

    fn flow_state(&self) -> FlowState {
        self.flow_state
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

    fn update_connections_after_rotation(&mut self) {
        self.connections = Self::connections_for(self.rotation);
    }

    fn update_flow_state_after_rotation(&mut self) {
        self.flow_state = Self::flow_state_for(self.rotation);
    }
}
