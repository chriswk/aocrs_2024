use std::collections::VecDeque;

use crate::point::Point;
use ahash::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub type Grid = Vec<Vec<u8>>;
pub type Input = (Grid, Point, Point);

#[aoc_generator(day20)]
fn parse(input: &str) -> Input {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let (mut start, mut end) = (Point::default(), Point::default());
    for (y, l) in grid.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == b'S' {
                start = Point {
                    x: x as isize,
                    y: y as isize,
                };
            } else if *c == b'E' {
                end = Point {
                    x: x as isize,
                    y: y as isize,
                };
            }
        }
    }
    (grid, start, end)
}

fn solve(grid: Vec<Vec<u8>>, start: Point, end: Point, disable_collision_for: usize) -> usize {
    let mut queue = VecDeque::from([(start, 0usize)]);
    let mut distances = HashMap::default();

    while let Some((p, distance)) = queue.pop_front() {
        if distances.contains_key(&p) {
            continue;
        }
        distances.insert(p, distance);
        if p == end {
            continue;
        }
        for neighbour in p.cardinal_neighbours() {
            if grid[neighbour.y as usize][neighbour.x as usize] != b'#' {
                queue.push_back((neighbour, distance + 1));
            }
        }
    }
    let mut improved_routes = 0;
    for ((&p, n), (&p2, n2)) in distances.iter().tuple_combinations() {
        let d = p.manhattan_distance(&p2);
        if d <= disable_collision_for && n2.abs_diff(*n) >= d + 100 {
            improved_routes += 1;
        }
    }
    improved_routes
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> usize {
    solve(input.0.clone(), input.1, input.2, 2)
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> usize {
    solve(input.0.clone(), input.1, input.2, 20)
}
