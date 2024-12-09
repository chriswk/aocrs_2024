use aoc_runner_derive::{aoc, aoc_generator};

fn solve(mut files: Vec<(usize, i32)>) -> usize {
    let mut i = files.len() - 1;
    while i > 0 {
        let (size, id) = files[i];
        if id == -1 {
            i -= 1;
            continue;
        }
        if let Some(j) = files[0..i]
            .iter()
            .position(|&(s, id)| id == -1 && size <= s)
        {
            let s = files[j].0;
            files[j] = (size, id);
            files[i] = (size, -1);
            if size < s {
                files.insert(j + 1, (s - size, -1));
            }
        }
        i -= 1;
    }
    files
        .iter()
        .flat_map(|&(s, id)| (0..s).map(move |_| id))
        .enumerate()
        .map(|(i, id)| if id == -1 { 0 } else { i * id as usize })
        .sum()
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut file_id = 0;
    let mut fs1 = Vec::new();
    for (i, b) in input.bytes().enumerate() {
        let v = if i % 2 == 0 {
            file_id += 1;
            file_id - 1
        } else {
            -1
        };
        fs1.extend((0..b - b'0').map(|_| (1, v)));
    }
    solve(fs1)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let mut file_id = 0;
    let mut fs2 = Vec::new();
    for (i, b) in input.bytes().enumerate() {
        let v = if i % 2 == 0 {
            file_id += 1;
            file_id - 1
        } else {
            -1
        };
        fs2.push(((b - b'0') as usize, v));
    }
    solve(fs2)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2858);
    }
}
