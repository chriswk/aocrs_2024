use aoc_runner_derive::{aoc, aoc_generator};

pub type Report = Vec<i32>;
pub type Reports = Vec<Report>;
#[aoc_generator(day2)]
fn parse(input: &str) -> Reports {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn safe(report: &Report, can_remove: bool) -> bool {
    let line_direction = (report[1] - report[0]).signum();
    for i in 0..report.len() - 1 {
        let mut diff = report[i + 1] - report[i];
        let diff_sign = diff.signum();
        diff = diff.abs();
        if line_direction == 0 || diff_sign != line_direction || diff > 3 || diff < 1 {
            if can_remove {
                return report
                    .iter()
                    .enumerate()
                    .any(|(j, _)| safe(&remove_at(report, j), false));
            }
            return false;
        }
    }
    true
}

fn remove_at(report: &Report, index: usize) -> Report {
    let mut new_report = report.to_vec();
    new_report.remove(index);
    new_report
}

#[aoc(day2, part1)]
fn part1(input: &Reports) -> usize {
    input.iter().filter(|report| safe(report, false)).count()
}

#[aoc(day2, part2)]
fn part2(input: &Reports) -> usize {
    input.iter().filter(|report| safe(report, true)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r#"7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"#
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r#"7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"#
            )),
            4
        );
    }
}
