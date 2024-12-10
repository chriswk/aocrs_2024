use std::{
    iter::successors,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
    pub fn delta(&self, other: &Self) -> Point {
        other - self
    }

    pub fn negative_delta(
        &self,
        other: &Self,
    ) -> std::iter::Successors<Point, impl FnMut(&Point) -> Option<Point>> {
        let delta = self.delta(other);
        successors(Some(self - &delta), move |d| Some(d - &delta))
    }

    pub fn positive_delta(
        &self,
        other: &Self,
    ) -> std::iter::Successors<Point, impl FnMut(&Point) -> Option<Point>> {
        let delta = self.delta(other);
        successors(Some(self + delta), move |d| Some(d + delta))
    }

    pub fn antinode(&self, other: &Self) -> Point {
        let twice_distance = self.delta(other) * 2;
        self + twice_distance
    }

    pub fn cardinal_neighbours(&self) -> [Point; 4] {
        [
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
        ]
    }
}
