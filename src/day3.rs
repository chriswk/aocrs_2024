use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    return input.trim().to_string();
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).fold(0, |acc, cap| {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        acc + a * b
    })
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .fold((0, true), |(sum, doing), cap| match &cap[0] {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            _ => {
                if cap[0].starts_with("mul") && doing {
                    let a = cap[1].parse::<i32>().unwrap();
                    let b = cap[2].parse::<i32>().unwrap();
                    (sum + (a * b), doing)
                } else {
                    (sum, doing)
                }
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#
            )),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
            )),
            48
        );
    }
}
