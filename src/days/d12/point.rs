use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(y: i64, x: i64) -> Point {
        Point { x, y }
    }

    pub fn rotate(&mut self, rot: i64) {
        let mut npos = (self.x, self.y);

        match rot {
            90 => npos = (self.y, -self.x),
            180 => npos = (-self.x, -self.y),
            270 => npos = (-self.y, self.x),
            _ => (),
        }
        self.x = npos.0;
        self.y = npos.1;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        (*self).x += other.x;
        (*self).y += other.y;
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.y + self.y * other.x,
            y: self.y * other.x + self.x * other.y,
        }
    }
}

impl Mul<i64> for Point {
    type Output = Self;

    fn mul(self, other: i64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
