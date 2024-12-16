use std::collections::VecDeque;

use ahash::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day15)]
pub fn parse(input: &str) -> (String, String) {
    let (a, insts) = input.split_once("\n\n").unwrap();
    (a.to_string(), insts.to_string())
}

const ROBOT: u8 = b'@';
const WALL: u8 = b'#';
const BOX: u8 = b'0';

fn solve(mut g: Vec<Vec<u8>>, instructions: &str) -> usize {
    let (mut row, mut column) = (0..g.len())
        .cartesian_product(0..g[0].len())
        .find(|&(row, col)| g[row][col] == ROBOT)
        .unwrap();
    'outer: for i in instructions.bytes() {
        let (dr, dc) = match i {
            b'^' => (-1 as i32, 0 as i32),
            b'>' => (0, 1),
            b'v' => (1, 0),
            b'<' => (0, -1),
            _ => continue,
        };
        let mut q = VecDeque::from([(row, column)]);
        let mut seen = HashSet::default();
        while let Some((rr, cc)) = q.pop_front() {
            if !seen.insert((rr, cc)) {
                continue;
            }
            let (r2, c2) = (rr + dr as usize, cc + dc as usize);
            match g[r2][c2] {
                WALL => continue 'outer,
                BOX => q.push_back((r2, c2)),
                b'[' => q.extend([(r2, c2), (r2, c2 + 1)]),
                b']' => q.extend([(r2, c2), (r2, c2 - 1)]),
                _ => continue,
            }
        }
        let boxes = seen
            .iter()
            .sorted_by_key(|&&(rr, cc)| (column.abs_diff(cc), row.abs_diff(rr)))
            .rev();
        for &(rr, cc) in boxes {
            let (r2, c2) = (rr + dr as usize, cc + dc as usize);
            g[r2][c2] = g[rr][cc];
            g[rr][cc] = b'.';
        }
        (row, column) = (row + dr as usize, column + dc as usize);
    }
    (0..g.len())
        .cartesian_product(0..g[0].len())
        .filter(|&(r, c)| matches!(g[r][c], b'O' | b'['))
        .map(|(r, c)| r * 100 + c)
        .sum()
}

#[aoc(day15, part1)]
fn part1((grid, instructions): &(String, String)) -> usize {
    let grid = grid.lines().map(|l| l.as_bytes().to_vec()).collect();
    solve(grid, &instructions)
}

#[aoc(day15, part2)]
fn part2((grid, instructions): &(String, String)) -> usize {
    let g2 = grid
        .lines()
        .map(|l| {
            l.bytes()
                .flat_map(|b| match b {
                    b'#' => b"##",
                    b'O' => b"[]",
                    b'.' => b"..",
                    b'@' => b"@.",
                    _ => unreachable!(),
                })
                .copied()
                .collect()
        })
        .collect();
    solve(g2, &instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE_LARGE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
    pub const EXAMPLE_SMALL: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_SMALL)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_SMALL)), 2);
    }
}
