use std::iter::successors;
use std::ops::{Add, Mul, Sub};

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::AHashMap as HashMap;
use ahash::AHashSet as Set;
use itertools::Itertools;

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
    fn delta(&self, other: &Self) -> Point {
        other - self
    }

    fn negative_delta(
        &self,
        other: &Self,
    ) -> std::iter::Successors<Point, impl FnMut(&Point) -> Option<Point>> {
        let delta = self.delta(other);
        successors(Some(self - &delta), move |d| Some(d - &delta))
    }

    fn positive_delta(
        &self,
        other: &Self,
    ) -> std::iter::Successors<Point, impl FnMut(&Point) -> Option<Point>> {
        let delta = self.delta(other);
        successors(Some(self + delta), move |d| Some(d + delta))
    }

    fn antinode(&self, other: &Self) -> Point {
        let twice_distance = self.delta(&other) * 2;
        self + twice_distance
    }
}

pub type Frequency = char;
pub type FrequencyMap = HashMap<Frequency, Vec<Point>>;
pub type Input = (FrequencyMap, usize, usize);
#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                let x = x as isize;
                let y = y as isize;
                map.entry(c).or_insert_with(Vec::new).push(Point { x, y });
            }
        });
    });
    (
        map,
        input.lines().count(),
        input.lines().next().unwrap().len(),
    )
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let width = input.2;
    let height = input.1;
    let all_antinodes: Set<Point> = input
        .0
        .iter()
        .flat_map(|(_, points)| {
            let antinodes: Set<Point> = points
                .iter()
                .flat_map(|p| {
                    points
                        .iter()
                        .filter(|&p2| p != p2)
                        .map(|p2| p.antinode(p2))
                        .filter(|p| {
                            p.x >= 0 && p.x < width as isize && p.y >= 0 && p.y < height as isize
                        })
                        .collect_vec()
                })
                .collect::<Set<Point>>();
            antinodes
        })
        .collect();
    all_antinodes.len()
}

fn in_grid(p: Point, width: usize, height: usize) -> bool {
    p.x >= 0 && p.x < width as isize && p.y >= 0 && p.y < height as isize
}
#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    let width = input.2;
    let height = input.1;
    let all_harmonics: Set<Point> = input
        .0
        .iter()
        .flat_map(|(freq, points)| {
            let antinodes: Set<Point> = points
                .iter()
                .flat_map(|p| {
                    points
                        .iter()
                        .filter(|&p2| p != p2)
                        .flat_map(|other| {
                            let mut negatives: Vec<Point> = p
                                .negative_delta(other)
                                .take_while(|new| in_grid(*new, width, height))
                                .collect_vec();
                            let positives: Vec<Point> = p
                                .positive_delta(other)
                                .take_while(|new| in_grid(*new, width, height))
                                .collect_vec();
                            negatives.extend(positives);
                            negatives
                        })
                        .collect_vec()
                })
                .collect::<Set<Point>>();
            antinodes
        })
        .collect();
    all_harmonics.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 34);
    }
}
