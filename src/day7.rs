use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(u64, Vec<u64>)>;

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut split = l.trim().split(":");
            let target: u64 = split
                .next()
                .expect("Didn't find separator in line")
                .parse()
                .expect("Could not parse operator");
            let numbers: Vec<u64> = split
                .next()
                .map(|l| {
                    l.split_whitespace()
                        .map(|f| f.trim().parse().unwrap())
                        .collect()
                })
                .expect("Could not parse numbers");
            (target, numbers)
        })
        .collect()
}

fn hold_true_part_1(target: u64, so_far: u64, rest: &[u64]) -> bool {
    if so_far > target {
        return false;
    }
    if rest.is_empty() {
        return so_far == target;
    }
    let mul = if so_far == 0 { 1 } else { so_far } * rest[0];
    let add = so_far + rest[0];
    hold_true_part_1(target, add, &rest[1..]) || hold_true_part_1(target, mul, &rest[1..])
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn hold_true_part_2(target: u64, so_far: u64, rest: &[u64]) -> bool {
    if so_far > target {
        return false;
    }
    if rest.is_empty() {
        return so_far == target;
    }
    let mul = if so_far == 0 { 1 } else { so_far } * rest[0];
    let add = so_far + rest[0];
    let concat_op = if so_far == 0 {
        rest[0]
    } else {
        concat(so_far, rest[0])
    };
    hold_true_part_2(target, add, &rest[1..])
        || hold_true_part_2(target, mul, &rest[1..])
        || hold_true_part_2(target, concat_op, &rest[1..])
}

fn hold_true_part_2_string(target: u64, so_far: u64, rest: &[u64]) -> bool {
    if so_far > target {
        return false;
    }
    if rest.is_empty() {
        return so_far == target;
    }
    let mul = if so_far == 0 { 1 } else { so_far } * rest[0];
    let add = so_far + rest[0];
    let concat_op = if so_far == 0 {
        rest[0]
    } else {
        string_concat(so_far, rest[0])
    };
    hold_true_part_2_string(target, add, &rest[1..])
        || hold_true_part_2_string(target, mul, &rest[1..])
        || hold_true_part_2_string(target, concat_op, &rest[1..])
}

fn string_concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(sum, values)| hold_true_part_1(*sum, 0, values))
        .fold(0, |acc, (sum, _)| acc + sum)
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(sum, values)| hold_true_part_2(*sum, 0, values))
        .fold(0, |acc, (sum, _)| acc + sum)
}

#[aoc(day7, part2, STRING_CONCAT)]
fn part2_string_concat(input: &Input) -> u64 {
    input
        .iter()
        .filter(|(sum, values)| hold_true_part_2_string(*sum, 0, values))
        .fold(0, |acc, (sum, _)| acc + sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3749);
    }

    #[test]
    fn concatting_works() {
        let a = 15u64;
        let b = 6u64;
        assert_eq!(concat(a, b), 156u64);
        let a = 15123u64;
        let b = 1231u64;
        assert_eq!(concat(a, b), 151231231u64);
    }

    #[test]
    fn test_ops() {
        assert!(hold_true_part_2(156, 0, &[15, 6]));
        assert!(hold_true_part_2(7290, 0, &[6, 8, 6, 15]));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 11387);
    }
}
