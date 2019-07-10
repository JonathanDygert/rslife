//! 2D Cartesian Coordinate

use std::ops::*;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
/// A 2-dimensional point.
pub struct Point<T> {
    /// The x-value of the point.
    pub x: T,
    /// The y-value of the point.
    pub y: T,
}

impl Point<i32> {
    /// Calculates the 8 neighboring points.
    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        (self.x - 1..=self.x + 1)
            .flat_map(move |x| (self.y - 1..=self.y + 1).map(move |y| Point { x, y }))
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y
    }
}
