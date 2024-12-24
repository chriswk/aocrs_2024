use std::{iter::successors, mem::zeroed};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

pub fn mix(secret: usize, other: usize) -> usize {
    secret ^ other
}

pub fn prune(secret: usize) -> usize {
    secret % 16777216
}

pub fn mul_64(secret: usize) -> usize {
    prune(mix(secret, secret * 64))
}

pub fn div_32(secret: usize) -> usize {
    prune(mix(secret, secret / 32))
}

pub fn mul_2048(secret: usize) -> usize {
    prune(mix(secret, secret * 2048))
}

pub fn evolve(secret: usize) -> std::iter::Successors<usize, impl FnMut(&usize) -> Option<usize>> {
    successors(Some(secret), |previous| Some(evolve_one(*previous)))
}

pub fn evolve_one(secret: usize) -> usize {
    mul_2048(div_32(mul_64(secret)))
}

#[aoc(day22, part1)]
fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .map(|initial| evolve(*initial).nth(2000).unwrap())
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &[usize]) -> usize {
    let mut result = vec![0; 130321];
    let mut seen = vec![usize::MAX; 130321];

    let to_index = |previous: usize, current: usize| 9 + current % 10 - previous % 10;
    for (id, number) in input.iter().enumerate() {
        let initial = *number;
        let first = evolve_one(initial);
        let second = evolve_one(first);
        let third = evolve_one(second);

        let mut a;
        let mut b = to_index(initial, first);
        let mut c = to_index(first, second);
        let mut d = to_index(second, third);
        let mut number = third;
        for _ in 3..2000 {
            let previous = number;
            number = evolve_one(number);
            (a, b, c, d) = (b, c, d, to_index(previous, number));
            let key = 6859 * a + 361 * b + 19 * c + d;
            if seen[key] != id {
                result[key] += number % 10;
                seen[key] = id;
            }
        }
    }
    *result.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    pub const EXAMPLE: &str = r#"1
    10
    100
    2024"#;

    #[test]
    fn part1_ops() {
        let next_numbers = evolve(123).take(6).collect_vec();
        assert_eq!(
            next_numbers,
            [123, 15887950, 16495136, 527345, 704524, 1553684]
        )
    }

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 37327623);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 24);
    }
}
