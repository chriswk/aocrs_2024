use ahash::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

pub struct Onsen {
    pub towels: HashSet<String>,
    pub displays: Vec<String>,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Onsen {
    let (available, desired) = input.split_once("\n\n").unwrap();
    Onsen {
        towels: available.split(", ").map(|l| l.to_string()).collect(),
        displays: desired.lines().map(|l| l.to_string()).collect(),
    }
}

pub fn count_valid_patterns<'a>(
    pattern: &'a str,
    stripes: &HashSet<String>,
    already_computed: &mut HashMap<&'a str, usize>,
    max_length: usize,
) -> usize {
    let mut combinations = 0;
    if already_computed.contains_key(pattern) {
        return *already_computed.get(pattern).unwrap();
    }
    if pattern.is_empty() {
        return 1;
    }
    for i in 1..=max_length.min(pattern.len()) {
        if stripes.contains(&pattern[..i].to_string()) {
            combinations +=
                count_valid_patterns(&pattern[i..], stripes, already_computed, max_length);
        }
    }
    already_computed.insert(pattern, combinations);
    combinations
}

#[aoc(day19, part1)]
fn part1(input: &Onsen) -> usize {
    let max_length = input.towels.iter().map(|v| v.len()).max().unwrap();
    let mut already_computed = HashMap::default();
    input
        .displays
        .iter()
        .filter(|p| count_valid_patterns(p, &input.towels, &mut already_computed, max_length) > 0)
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &Onsen) -> usize {
    let max_length = input.towels.iter().map(|v| v.len()).max().unwrap();
    let mut already_computed = HashMap::default();
    input
        .displays
        .iter()
        .map(|p| count_valid_patterns(p, &input.towels, &mut already_computed, max_length))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 421);
    }
}
