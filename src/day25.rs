use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Schematic {
    pub is_key: bool,
    pub heights: Vec<usize>,
}

impl Schematic {
    fn fits(&self, other: &Schematic) -> bool {
        (self.is_key ^ other.is_key)
            && self
                .heights
                .iter()
                .zip(&other.heights)
                .all(|(&a, b)| a + b <= 7)
    }
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s
            .lines()
            .map(|l| l.trim().chars().collect_vec())
            .collect_vec();
        let is_key = lines[0][0] == '.';
        if !is_key {
            lines.reverse();
        }
        let heights = (0..lines[0].len())
            .map(|col| lines.iter().map(|l| l[col]).filter(|&c| c == '#').count())
            .collect_vec();
        Ok(Self { is_key, heights })
    }
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Vec<Schematic> {
    input
        .split("\n\n")
        .map(|x| Schematic::from_str(x).unwrap())
        .collect_vec()
}

#[aoc(day25, part1)]
fn part1(input: &[Schematic]) -> usize {
    let (keys, locks): (Vec<_>, Vec<_>) = input.iter().partition(|s| s.is_key);

    keys.iter()
        .map(|key| locks.iter().filter(|lock| key.fits(lock)).count())
        .sum()
}

#[aoc(day25, part2)]
fn part2(input: &[Schematic]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"#####
    .####
    .####
    .####
    .#.#.
    .#...
    .....

    #####
    ##.##
    .#.##
    ...##
    ...#.
    ...#.
    .....

    .....
    #....
    #....
    #...#
    #.#.#
    #.###
    #####

    .....
    .....
    #.#..
    ###..
    ###.#
    ###.#
    #####

    .....
    .....
    .....
    #....
    #.#..
    #.#.#
    #####"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
