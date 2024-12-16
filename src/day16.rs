use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::point::{Direction, Point};

type Input = (Vec<u8>, usize, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Robot {
    pub score: usize,
    pub location: Point,
    pub facing: Direction,
    pub path: Vec<Point>,
}

impl Ord for Robot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Robot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Input {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    (grid, width, height)
}

fn solve(grid: &[u8], start: Point, width: &usize, height: &usize, part_1: bool) -> usize {
    let mut queue = BinaryHeap::new();
    let starting = Robot {
        score: 0,
        location: start,
        facing: Direction::East,
        path: Vec::new(),
    };
    queue.push(Reverse(starting));

    let mut seen = vec![usize::MAX - 1000; width * height];
    let mut min = usize::MAX;
    let mut paths = Vec::new();
    while let Some(Reverse(Robot {
        score,
        location,
        facing,
        path,
    })) = queue.pop()
    {
        if grid[location.index(width)] == b'E' {
            if score > min {
                break;
            }
            paths.push(path.clone());
            min = score;
        }

        for dir in facing.no_uturn() {
            let new = location.navigate(&dir);
            let nscore = if (facing == dir) {
                score + 1
            } else {
                score + 1001
            };
            let last_seen_score = seen[new.index(width)];
            if new.inbounds(width, height)
                && grid[new.index(width)] != b'#'
                && nscore <= last_seen_score + 1000
            {
                seen[new.index(width)] = nscore;
                let mut new_path = path.clone();
                new_path.push(new);
                queue.push(Reverse(Robot {
                    score: nscore,
                    location: new,
                    facing: dir,
                    path: new_path,
                }));
            }
        }
    }
    if part_1 {
        min
    } else {
        let mut places_to_sit = vec![false; width * height];
        let mut total = 0;
        for path in &paths {
            for &point in path {
                if !places_to_sit[point.index(width)] {
                    places_to_sit[point.index(width)] = true;
                    total += 1;
                }
            }
        }
        total + 1
    }
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> usize {
    let (grid, width, height) = input;
    let mut start = Point::new(0, 0);
    'outer: for y in 0..*height {
        for x in 0..*width {
            if grid[y * width + x] == b'S' {
                start = Point::new(x as isize, y as isize);
                break 'outer;
            }
        }
    }

    solve(grid, start, width, height, true)
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> usize {
    let (grid, width, height) = input;
    let mut start = Point::new(0, 0);
    'outer: for y in 0..*height {
        for x in 0..*width {
            if grid[y * width + x] == b'S' {
                start = Point::new(x as isize, y as isize);
                break 'outer;
            }
        }
    }

    solve(grid, start, width, height, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const MAZE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
