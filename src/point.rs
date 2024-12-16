use std::{
    fmt::Display,
    iter::successors,
    ops::{Add, Mul, Sub},
};

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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

impl Add<&Point> for Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
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

    pub fn navigate(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => self + Point::new(0, -1),
            Direction::East => self + Point::new(1, 0),
            Direction::South => self + Point::new(0, 1),
            Direction::West => self + Point::new(-1, 0),
        }
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

    pub fn bounded(&self, width: isize, height: isize) -> Self {
        Point {
            x: self.x.rem_euclid(width),
            y: self.y.rem_euclid(height),
        }
    }
}
