use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Hash)]
pub struct Vec2 {
    pub x: u32,
    pub y: u32,
}

impl Default for Rect {
    fn default() -> Self {
        Self::empty()
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Vec2 {
    pub fn from_y(y: u32) -> Self {
        Self { x: 0, y }
    }
    pub fn from_x(x: u32) -> Self {
        Self { x, y: 0 }
    }
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

#[derive(Debug, Hash)]
pub struct Rect {
    pub start: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(start: Vec2, size: Vec2) -> Self {
        Self { start, size }
    }
    pub fn from_0_0(size: Vec2) -> Self {
        Self {
            start: Vec2::ZERO,
            size,
        }
    }
    pub fn empty() -> Self {
        Self {
            start: Vec2::ZERO,
            size: Vec2::ZERO,
        }
    }
    pub fn offset(self, offset: Vec2) -> Self {
        Self {
            start: self.start + offset,
            size: self.size,
        }
    }
}
