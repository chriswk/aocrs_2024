use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

use crate::point::Point;

pub type Machine = (Point, Point, Point);

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Machine> {
    let actions = Regex::new(r"X[\+=](\d+)[^Y]*Y[\+=](\d+)").unwrap();
    let points: Vec<Point> = actions
        .captures_iter(&input.lines().join(" "))
        .map(|cap| {
            let x = cap[1].parse::<isize>().unwrap();
            let y = cap[2].parse::<isize>().unwrap();
            Point { x, y }
        })
        .collect();
    points.chunks(3).map(|p| (p[0], p[1], p[2])).collect()
}

pub fn find_target((a_button, b_button, target): &Machine) -> Option<usize> {
    let b = (target.y * a_button.x - target.x * a_button.y)
        / (b_button.y * a_button.x - b_button.x * a_button.y);
    let a = (target.x - b * b_button.x) / a_button.x;
    if (a_button.x * a + b_button.x * b) == target.x
        && (a_button.y * a + b_button.y * b) == target.y
    {
        Some((a * 3 + b) as usize)
    } else {
        None
    }
}

#[aoc(day13, part1)]
fn part1(input: &[Machine]) -> usize {
    input.iter().filter_map(find_target).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Machine]) -> usize {
    let offset = Point {
        x: 10000000000000,
        y: 10000000000000,
    };
    input
        .iter()
        .map(|machine| (machine.0, machine.1, &machine.2 + offset))
        .filter_map(|machine| find_target(&machine))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 875318608908);
    }
}
