use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Debug)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,

    output: Vec<i64>,

    instructions: Vec<i64>,
    pointer: usize,
}

impl Computer {
    fn get_next(&mut self) -> Option<(i64, i64)> {
        if let Some((a, b)) = self
            .instructions
            .get(self.pointer)
            .zip(self.instructions.get(self.pointer + 1))
        {
            return Some((*a, *b));
        }
        None
    }
    fn get_combo(&self, v: i64) -> i64 {
        match v {
            0..=3 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }
    fn instruction(&mut self, instr: i64, literal: i64) {
        match instr {
            0 => self.a >>= self.get_combo(literal),
            1 => self.b ^= literal,
            2 => self.b = self.get_combo(literal) % 8,
            3 => {
                if self.a != 0 {
                    self.pointer = literal.try_into().unwrap();
                    return;
                }
            }
            4 => self.b ^= self.c,
            5 => self.output.push(self.get_combo(literal) % 8),
            6 => self.b = self.a >> self.get_combo(literal),
            7 => self.c = self.a >> self.get_combo(literal),
            _ => panic!(),
        }

        self.pointer += 2;
    }

    fn run_to_end(&mut self) {
        loop {
            let ins = self.get_next();
            match ins {
                None => break,
                Some((i, v)) => self.instruction(i, v),
            }
        }
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Computer {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut reg_lines = registers.lines();
    let a_reg = reg_lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .map(|f| f.trim())
        .unwrap()
        .parse()
        .unwrap();
    let b_reg = reg_lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .map(|f| f.trim())
        .unwrap()
        .parse()
        .unwrap();
    let c_reg = reg_lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .map(|f| f.trim())
        .unwrap()
        .parse()
        .unwrap();
    let instructions = program
        .trim()
        .split(":")
        .last()
        .unwrap()
        .split(",")
        .map(|f| f.trim().parse().unwrap())
        .collect();
    Computer {
        a: a_reg,
        b: b_reg,
        c: c_reg,
        output: vec![],
        instructions,
        pointer: 0,
    }
}

#[aoc(day17, part1)]
fn part1(input: &Computer) -> String {
    let mut cpu = input.clone();
    cpu.run_to_end();
    cpu.output.iter().join(",")
}

#[aoc(day17, part2)]
fn part2(input: &Computer) -> u64 {
    let mut factors = vec![0; input.instructions.len()];
    loop {
        let mut init_a = 0;
        for (i, f) in factors.iter().enumerate() {
            init_a += 8u64.pow(i as u32) * f;
        }
        let mut cpu = Computer {
            a: init_a as i64,
            ..input.clone()
        };
        cpu.run_to_end();
        if cpu.output == input.instructions {
            return init_a;
        }
        for i in (0..input.instructions.len()).rev() {
            if cpu.output.len() < i {
                factors[i] += 1;
                break;
            }
            if cpu.output[i] != cpu.instructions[i] {
                factors[i] += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const INPUT: &str = include_str!("../input/2024/day17.txt");

    pub const EXAMPLE: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    pub const CHECK_BEHAVIOUR: &str = r#"Register A: 0
Register B: 0
Register C: 9

Program: 2,6
    "#;

    #[test]
    fn verify_states() {
        let mut cp = parse(
            r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"#,
        );
        cp.run_to_end();
        assert_eq!(cp.output, vec![0, 1, 2]);
    }

    #[test]
    pub fn year_example() {
        let mut cp = parse(
            r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#,
        );
        cp.run_to_end();
        assert_eq!(cp.a, 0);
        assert_eq!(cp.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    pub fn math_works() {
        let mut cp = parse(
            r#"Register A: 0
Register B: 29
Register C: 0

Program: 1,7"#,
        );
        cp.run_to_end();
        assert_eq!(cp.b, 26);
    }

    #[test]
    pub fn years_are_magic() {
        let mut cp = parse(
            r#"Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0"#,
        );
        cp.run_to_end();
        assert_eq!(cp.b, 44354);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse(INPUT)), "2,3,4,7,5,7,3,0,7");
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&parse(INPUT)), 190384609508367);
    }
}
