use std::iter::successors;

use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub type Stones = HashMap<u64, usize>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Stones {
    input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}

fn apply_rules(input: &Stones) -> Stones {
    let mut stones = HashMap::with_capacity(input.len());
    for (&s, &v) in input {
        match s {
            0 => *stones.entry(1).or_default() += v,
            _ => {
                let digits = s.ilog10() + 1;
                if digits % 2 == 0 {
                    let pow = 10u64.pow(digits / 2);
                    *stones.entry(s % pow).or_default() += v;
                    *stones.entry(s / pow).or_default() += v;
                } else {
                    *stones.entry(s * 2024).or_default() += v;
                }
            }
        }
    }
    stones
}

#[aoc(day11, part1)]
fn part1(input: &Stones) -> usize {
    successors(Some(input.clone()), |input| Some(apply_rules(input)))
        .nth(25)
        .map(|v| v.values().sum())
        .unwrap()
}

#[aoc(day11, part2)]
fn part2(input: &Stones) -> usize {
    successors(Some(input.clone()), |input| Some(apply_rules(input)))
        .nth(75)
        .map(|v| v.values().sum())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 65601038650482);
    }
}
