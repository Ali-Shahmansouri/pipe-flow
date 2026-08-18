#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,

    MoveSelectorUp,
    MoveSelectorDown,
    MoveSelectorLeft,
    MoveSelectorRight,

    RotateClockwise,
}
