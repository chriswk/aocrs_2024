use std::collections::VecDeque;

use ahash::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::point::Point;

pub type Grid = HashMap<Point, char>;

#[aoc_generator(day12)]
fn parse(input: &str) -> Grid {
    let lines = input.lines();
    lines
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim().chars().enumerate().map(move |(x, c)| {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                (point, c)
            })
        })
        .collect::<HashMap<Point, char>>()
}

fn solve(grid: &Grid, height: usize, width: usize) -> usize {
    let mut visited: HashSet<Point> = HashSet::default();
    let mut queue = VecDeque::with_capacity(32);
    let mut cost = 0;
    for r in 0..=height {
        for c in 0..=width {
            let starting_point = Point {
                x: c as isize,
                y: r as isize,
            };
            if visited.contains(&starting_point) {
                continue;
            }
            let starting_val = grid.get(&starting_point).unwrap();
            let mut area = 0;
            let mut edge_cost = 0;
            queue.push_back(starting_point);
            visited.insert(starting_point);
            while let Some(current_point) = queue.pop_front() {
                area += 1;
                for neighbour in current_point.cardinal_neighbours() {
                    if !grid.contains_key(&neighbour) {
                        edge_cost += 1;
                    } else if grid.get(&neighbour) == Some(starting_val) {
                        if !visited.contains(&neighbour) {
                            visited.insert(neighbour);
                            queue.push_back(neighbour);
                        }
                    } else {
                        edge_cost += 1;
                    }
                }
            }
            cost += area * edge_cost;
        }
    }
    cost
}

fn get_number_of_corners(grid: &Grid, point: Point) -> usize {
    let mut number_of_corners = 0;
    let mut matches = [[false; 3]; 3];
    match grid.get(&point) {
        Some(p) => {
            for r_d in 0..3 {
                for c_d in 0..3 {
                    let compare_point = &point
                        + Point {
                            x: c_d as isize - 1,
                            y: r_d as isize - 1,
                        };
                    matches[r_d][c_d] =
                        grid.contains_key(&compare_point) && grid.get(&compare_point) == Some(p);
                }
            }
            if matches[0][1] {
                if matches[1][0] && !matches[0][0] {
                    number_of_corners += 1;
                }
                if matches[1][2] && !matches[0][2] {
                    number_of_corners += 1;
                }
            } else {
                if !matches[1][0] {
                    number_of_corners += 1;
                }
                if !matches[1][2] {
                    number_of_corners += 1;
                }
            }
            if matches[2][1] {
                if matches[1][2] && !matches[2][2] {
                    number_of_corners += 1;
                }

                if matches[1][0] && !matches[2][0] {
                    number_of_corners += 1;
                }
            } else {
                if !matches[1][0] {
                    number_of_corners += 1;
                }

                if !matches[1][2] {
                    number_of_corners += 1;
                }
            }
            number_of_corners
        }
        None => 0,
    }
}
fn solve_part2(grid: &Grid, height: usize, width: usize) -> usize {
    let mut visited: HashSet<Point> = HashSet::default();
    let mut queue = VecDeque::with_capacity(32);
    let mut cost = 0;
    for r in 0..=height {
        for c in 0..=width {
            let starting_point = Point {
                x: c as isize,
                y: r as isize,
            };
            if visited.contains(&starting_point) {
                continue;
            }
            let starting_val = grid.get(&starting_point).unwrap();
            let mut area = 0;
            let mut edge_cost = 0;
            queue.push_back(starting_point);
            visited.insert(starting_point);
            while let Some(current_point) = queue.pop_front() {
                area += 1;
                edge_cost += get_number_of_corners(&grid, current_point);
                for neighbour in current_point.cardinal_neighbours() {
                    if grid.get(&neighbour) == Some(starting_val) && !visited.contains(&neighbour) {
                        visited.insert(neighbour);
                        queue.push_back(neighbour);
                    }
                }
            }
            cost += area * edge_cost;
        }
    }
    cost
}

#[aoc(day12, part1)]
fn part1(input: &Grid) -> usize {
    let biggest = input.iter().max().unwrap();
    solve(input, biggest.0.y as usize, biggest.0.x as usize)
}

#[aoc(day12, part2)]
fn part2(input: &Grid) -> usize {
    let biggest = input.iter().max().unwrap();
    solve_part2(input, biggest.0.y as usize, biggest.0.x as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SMALL_EXAMPLE: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    pub const TIC_TAC_TOE: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    pub const LARGE_EXAMPLE: &str = r#"RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SMALL_EXAMPLE)), 140);
        assert_eq!(part1(&parse(TIC_TAC_TOE)), 772);
        assert_eq!(part1(&parse(LARGE_EXAMPLE)), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SMALL_EXAMPLE)), 80);
        assert_eq!(part2(&parse(TIC_TAC_TOE)), 436);
        assert_eq!(part2(&parse(LARGE_EXAMPLE)), 1206);
    }
}
