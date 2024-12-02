use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator_part_1(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l| {
            let mut it =
                l.split_whitespace().filter_map(
                    |s| {
                        if s.is_empty() {
                            None
                        } else {
                            s.parse().ok()
                        }
                    },
                );
            let x: i32 = it.next().unwrap();
            let y: i32 = it.next().unwrap();
            (x, y)
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut x, mut y) = input.clone();
    x.sort();
    y.sort();
    x.iter().zip(y).fold(0, |acc, (x, y)| acc + (y - x).abs())
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut x, mut y) = input.clone();
    x.sort();
    y.sort();
    let frequency_map = y.iter().fold(HashMap::new(), |mut acc, y| {
        *acc.entry(y).or_insert(0) += 1;
        acc
    });
    x.iter()
        .fold(0, |acc, x| acc + (x * frequency_map.get(x).unwrap_or(&0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator_part_1(
            r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#,
        );
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let input = input_generator_part_1(
            r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#,
        );
        assert_eq!(part2(&input), 31)
    }
}
