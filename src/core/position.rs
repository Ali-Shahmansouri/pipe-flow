use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position { x, y }
    }
}

impl Add<(usize, usize)> for Position {
    type Output = Self;

    fn add(self, (x, y): (usize, usize)) -> Self::Output {
        Self {
            x: self.x.saturating_add(x),
            y: self.y.saturating_add(y),
        }
    }
}

impl AddAssign<(usize, usize)> for Position {
    fn add_assign(&mut self, (x, y): (usize, usize)) {
        self.x = self.x.saturating_add(x);
        self.y = self.y.saturating_add(y);
    }
}

impl Sub<(usize, usize)> for Position {
    type Output = Self;

    fn sub(self, (x, y): (usize, usize)) -> Self::Output {
        Self {
            x: self.x.saturating_sub(x),
            y: self.y.saturating_sub(y),
        }
    }
}

impl SubAssign<(usize, usize)> for Position {
    fn sub_assign(&mut self, (x, y): (usize, usize)) {
        self.x = self.x.saturating_sub(x);
        self.y = self.y.saturating_sub(y);
    }
}
