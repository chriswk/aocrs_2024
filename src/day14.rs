use std::iter::successors;

use ahash::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

use crate::point::Point;

#[derive(Debug, Clone)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn tick(&self, ticks: isize, width: usize, height: usize) -> Robot {
        let pos = &self.pos + self.vel * ticks;
        Robot {
            pos: pos.bounded(width as isize, height as isize),
            vel: self.vel,
        }
    }
}

pub type Quadrants = (usize, usize, usize, usize);

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    let pattern = Regex::new(r#"p=(\-?\d+),(\-?\d+)[^v]*v=(\-?\d+),(\-?\d+)"#).unwrap();
    input
        .lines()
        .map(|l| {
            let captures = pattern.captures(l).unwrap();
            Robot {
                pos: Point {
                    x: captures[1].parse().unwrap(),
                    y: captures[2].parse().unwrap(),
                },
                vel: Point {
                    x: captures[3].parse().unwrap(),
                    y: captures[4].parse().unwrap(),
                },
            }
        })
        .collect()
}

pub const GRID_HEIGHT: usize = 103;
pub const GRID_WIDTH: usize = 101;

fn safety_factor(robots: &[Robot], grid_width: usize, grid_height: usize) -> Quadrants {
    let quadrant_width = grid_width / 2;
    let quadrant_height = grid_height / 2;
    robots.iter().fold((0, 0, 0, 0), |quadrants, robot| {
        let end = robot.tick(100, grid_width, grid_height);
        match end.pos.x.cmp(&(quadrant_width as isize)) {
            std::cmp::Ordering::Less => match end.pos.y.cmp(&(quadrant_height as isize)) {
                std::cmp::Ordering::Less => {
                    (quadrants.0 + 1, quadrants.1, quadrants.2, quadrants.3)
                }
                std::cmp::Ordering::Greater => {
                    (quadrants.0, quadrants.1 + 1, quadrants.2, quadrants.3)
                }
                _ => quadrants,
            },
            std::cmp::Ordering::Greater => match end.pos.y.cmp(&(quadrant_height as isize)) {
                std::cmp::Ordering::Less => {
                    (quadrants.0, quadrants.1, quadrants.2 + 1, quadrants.3)
                }
                std::cmp::Ordering::Greater => {
                    (quadrants.0, quadrants.1, quadrants.2, quadrants.3 + 1)
                }
                _ => quadrants,
            },
            _ => quadrants,
        }
    })
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> usize {
    let (zone1, zone2, zone3, zone4) = safety_factor(input, GRID_WIDTH, GRID_HEIGHT);
    zone1 * zone2 * zone3 * zone4
}

fn calculate_variance(positions: &[Point]) -> f64 {
    let mean = positions.iter().map(|&p| p.x as f64).sum::<f64>() / positions.len() as f64;
    let variance = positions
        .iter()
        .map(|&p| {
            let diff = p.x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / positions.len() as f64;
    variance
}
fn find_best_offset(robots: &[Robot], modulo: usize, x_axis: bool) -> usize {
    let mut best_variance = f64::MAX;
    let mut best_offset = 0;
    for offset in 0..modulo {
        let positions: Vec<_> =
            move_robots(robots, modulo as isize, modulo as isize, offset as isize).collect();
        let variance = if x_axis {
            calculate_variance(&positions)
        } else {
            calculate_variance(
                &positions
                    .iter()
                    .map(|p| Point { x: p.y, y: p.x })
                    .collect_vec(),
            )
        };
        if variance < best_variance {
            best_variance = variance;
            best_offset = offset;
        }
    }
    best_offset
}

fn move_robots<'a>(
    robots: &'a [Robot],
    width: isize,
    height: isize,
    steps: isize,
) -> impl Iterator<Item = Point> + 'a {
    let steps_x = steps % width;
    let steps_y = steps % height;
    robots.iter().map(move |robot| {
        let new_x = (robot.pos.x + steps_x as isize * robot.vel.x).rem_euclid(width);
        let new_y = (robot.pos.y + steps_y as isize * robot.vel.y).rem_euclid(height);
        Point { x: new_x, y: new_y }
    })
}

#[aoc(day14, part2)]
fn part2(robots: &[Robot]) -> usize {
    successors(Some(robots.to_vec()), |robots| {
        let new_robots = robots
            .iter()
            .map(|r| r.tick(1, GRID_WIDTH, GRID_HEIGHT))
            .collect_vec();
        Some(new_robots)
    })
    .enumerate()
    .find(|(_i, new_robots)| {
        new_robots.clone().len() == HashSet::from_iter(new_robots.iter().map(|r| r.pos)).len()
    })
    .unwrap()
    .0
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3"#;

    #[test]
    fn part1_example() {
        assert_eq!(safety_factor(&parse(EXAMPLE), 11, 7), (1, 4, 3, 1));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 1);
    }
}
