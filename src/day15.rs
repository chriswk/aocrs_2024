use std::collections::VecDeque;

use ahash::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day15)]
pub fn parse(input: &str) -> (String, String) {
    let (a, insts) = input.split_once("\n\n").unwrap();
    (a.to_string(), insts.to_string())
}

fn solve(mut g: Vec<Vec<u8>>, insts: &str) -> usize {
    let (mut r, mut c) = (0..g.len())
        .cartesian_product(0..g[0].len())
        .find(|&(r, c)| g[r][c] == b'@')
        .unwrap();
    'outer: for i in insts.bytes() {
        let (dr, dc) = match i {
            b'^' => (-1, 0),
            b'>' => (0, 1),
            b'v' => (1, 0),
            b'<' => (0, -1),
            _ => continue,
        };
        let mut q = VecDeque::from([(r, c)]);
        let mut seen = HashSet::default();
        while let Some((rr, cc)) = q.pop_front() {
            if !seen.insert((rr, cc)) {
                continue;
            }
            let (r2, c2) = (rr + dr as usize, cc + dc as usize);
            match g[r2][c2] {
                b'#' => continue 'outer,
                b'O' => q.push_back((r2, c2)),
                b'[' => q.extend([(r2, c2), (r2, c2 + 1)]),
                b']' => q.extend([(r2, c2), (r2, c2 - 1)]),
                _ => continue,
            }
        }
        let boxes = seen
            .iter()
            .sorted_by_key(|&&(rr, cc)| (c.abs_diff(cc), r.abs_diff(rr)))
            .rev();
        for &(rr, cc) in boxes {
            let (r2, c2) = (rr + dr as usize, cc + dc as usize);
            g[r2][c2] = g[rr][cc];
            g[rr][cc] = b'.';
        }
        (r, c) = (r + dr as usize, c + dc as usize);
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
