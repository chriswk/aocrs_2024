use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub type Grid = Vec<Vec<u8>>;
#[aoc_generator(day6)]
fn parse(input: &str) -> Grid {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn start(grid: &Grid) -> (usize, usize) {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(r, c)| grid[r][c] == b'^')
        .unwrap()
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn walk(
    m: &Grid,
    mut row: usize,
    mut col: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut seen = vec![vec![[false; 4]; m[0].len()]; m.len()];
    let mut direction = 0;
    loop {
        if seen[row][col][direction] {
            return None;
        }
        seen[row][col][direction] = true;
        let (delta_row, delta_col) = DIRS[direction];
        let (new_row, new_col) = (row + delta_row as usize, col + delta_col as usize);
        if !(0..m.len()).contains(&new_row) || !(0..m[0].len()).contains(&new_col) {
            if !return_squares {
                return Some(Vec::new());
            }
            let visited = (0..m.len())
                .cartesian_product(0..m[0].len())
                .filter(|&(r, c)| seen[r][c].iter().any(|&b| b))
                .collect();
            return Some(visited);
        }
        if m[new_row][new_col] == b'#' {
            direction = (direction + 1) % 4;
        } else {
            (row, col) = (new_row, new_col);
        }
    }
}

#[aoc(day6, part1)]
fn part1(grid: &Grid) -> usize {
    let start = start(grid);
    walk(grid, start.0, start.1, true).unwrap().len()
}

#[aoc(day6, part2)]
fn part2(grid: &Grid) -> usize {
    let start = start(grid);
    let mut obstacles = grid.clone();
    walk(grid, start.0, start.1, true)
        .unwrap()
        .iter()
        .filter(|&&(row, col)| {
            obstacles[row][col] = b'#';
            let ok = walk(&obstacles, start.0, start.1, false).is_none();
            obstacles[row][col] = b'.';
            ok
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
}
