use std::{
    fmt::Display,
    iter::successors,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn no_uturn(&self) -> [Direction; 3] {
        match self {
            Direction::North => [Direction::North, Direction::East, Direction::West],
            Direction::East => [Direction::East, Direction::North, Direction::South],
            Direction::West => [Direction::West, Direction::North, Direction::South],
            Direction::South => [Direction::South, Direction::West, Direction::East],
        }
    }
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

    pub fn navigate(&self, direction: &Direction) -> Self {
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

    pub fn index(&self, width: &usize) -> usize {
        (self.y * *width as isize + self.x) as usize
    }

    pub fn inbounds(&self, width: &usize, height: &usize) -> bool {
        self.x >= 0 && self.x < *width as isize && self.y >= 0 && self.y < *height as isize
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
