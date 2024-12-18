use std::{collections::VecDeque, usize};

use ahash::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::point::Point;
#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|f| {
            let x = f
                .trim()
                .split(",")
                .map(|j| j.parse().unwrap())
                .collect::<Vec<isize>>();
            Point::new(x[0], x[1])
        })
        .collect()
}

fn solve(corrupted_nodes: HashSet<Point>, end: Point) -> Option<usize> {
    let mut seen = HashSet::default();
    let start = Point::new(0, 0);
    seen.insert(start);
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    while let Some((point, distance)) = q.pop_front() {
        let neighbours = point.bounded_new_neighbours(&seen, &end);
        for neighbour in neighbours {
            if neighbour.x >= 0
                && neighbour.y >= 0
                && neighbour.x < end.x + 1
                && neighbour.y < end.y + 1
            {
                if neighbour == end {
                    return Some(distance + 1);
                }
                if !corrupted_nodes.contains(&neighbour) {
                    q.push_back((neighbour, distance + 1));
                    seen.insert(neighbour);
                }
            }
        }
    }
    None
}

#[aoc(day18, part1)]
fn part1(input: &[Point]) -> usize {
    solve(
        input.iter().take(1024).cloned().collect::<HashSet<_>>(),
        Point::new(70, 70),
    )
    .unwrap()
}

fn blocking_byte(corrupted_locations: &[Point], exit: Point) -> Option<Point> {
    let byte_indexes = corrupted_locations.iter().enumerate().fold(
        HashMap::default(),
        |mut indexes, (index, location)| {
            indexes.entry(location).or_insert(index);
            indexes
        },
    );

    let start = Point::new(0, 0);
    let starting_index = if let Some(index) = byte_indexes.get(&start) {
        *index
    } else {
        corrupted_locations.len()
    };

    let mut queue = VecDeque::from([(start, starting_index)]);
    let mut visited = HashMap::default();
    visited.insert(start, starting_index);

    let mut blocking_byte = None;
    let mut max_index = 0;

    while let Some((position, index)) = queue.pop_front() {
        for next_position in [
            Point::new(position.x + 1, position.y),
            Point::new(position.x - 1, position.y),
            Point::new(position.x, position.y + 1),
            Point::new(position.x, position.y - 1),
        ] {
            if next_position.x >= 0
                && next_position.x <= exit.x
                && next_position.y >= 0
                && next_position.y <= exit.y
            {
                if next_position == exit && index >= max_index && index < corrupted_locations.len()
                {
                    blocking_byte = Some(corrupted_locations[index]);
                    max_index = index;
                }

                if let Some(prev_index) = visited.get(&next_position) {
                    if *prev_index >= index {
                        continue;
                    }
                }

                match byte_indexes.get(&next_position) {
                    Some(corrupted_index) if *corrupted_index < index => {
                        let new_index = *corrupted_index;

                        queue.push_back((next_position, new_index));
                        visited.insert(next_position, new_index);
                    }
                    _ => {
                        queue.push_back((next_position, index));
                        visited.insert(next_position, index);
                    }
                }
            }
        }
    }

    blocking_byte
}

#[aoc(day18, part2)]
fn part2(input: &[Point]) -> Point {
    blocking_byte(input, Point::new(70, 70)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"5,4
    4,2
    4,5
    3,0
    2,1
    6,3
    2,4
    1,5
    0,6
    3,3
    2,6
    5,1
    1,2
    5,5
    2,5
    6,5
    1,4
    0,4
    6,4
    1,1
    6,1
    1,0
    0,5
    1,6
    2,0"#;

    #[test]
    fn part1_example() {
        let corrupted = parse(EXAMPLE);
        let corrupted = corrupted.into_iter().take(12).collect::<HashSet<_>>();
        assert_eq!(solve(corrupted, Point::new(6, 6)), Some(22));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), Point::new(6, 1));
    }
}
