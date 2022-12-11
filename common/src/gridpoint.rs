use std::fmt::Display;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

impl GridPoint {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
}

impl std::ops::Add<(i32, i32)> for GridPoint {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl PartialEq<(i32, i32)> for GridPoint {
    fn eq(&self, other: &(i32, i32)) -> bool { self.x == other.0 && self.y == other.1 }
}

impl Display for GridPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for GridPoint {
    fn from(tuple: (i32, i32)) -> Self { Self::new(tuple.0, tuple.1) }
}
