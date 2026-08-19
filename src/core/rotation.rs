#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl Rotation {
    pub fn default() -> Self {
        Self::Zero
    }

    pub fn clockwise(self) -> Self {
        match self {
            Self::Zero => Self::Ninety,
            Self::Ninety => Self::OneEighty,
            Self::OneEighty => Self::TwoSeventy,
            Self::TwoSeventy => Self::Zero,
        }
    }
}
