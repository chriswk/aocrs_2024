use crate::point::Point;
use ahash::{AHashMap as HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

pub type TopoMap = HashMap<Point, u32>;

#[aoc_generator(day10)]
fn parse(input: &str) -> TopoMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '.' {
                    return None;
                }
                let height = c.to_digit(10).unwrap();
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                Some((point, height))
            })
        })
        .collect::<HashMap<Point, u32>>()
}

fn valid_neighbours(guide_map: &TopoMap, point: &Point) -> Vec<Point> {
    let height = guide_map.get(point).unwrap();
    point
        .cardinal_neighbours()
        .iter()
        .filter(|p| guide_map.get(p) == Some(&(height + 1)))
        .cloned()
        .collect()
}

fn rating(guide: &TopoMap, pos: &Point) -> usize {
    if let Some(height) = guide.get(pos) {
        if height == &9 {
            return 1;
        }
        valid_neighbours(guide, pos)
            .iter()
            .map(|n| rating(guide, n))
            .sum()
    } else {
        0
    }
}

fn possible_route(
    guide: &TopoMap,
    pos: &Point,
    peaks_seen_so_far: &mut HashSet<Point>,
) -> HashSet<Point> {
    let height = guide.get(pos).unwrap();
    if height == &9 {
        peaks_seen_so_far.insert(*pos);
        return peaks_seen_so_far.clone();
    }
    valid_neighbours(guide, pos)
        .iter()
        .flat_map(|n| possible_route(guide, n, peaks_seen_so_far))
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &TopoMap) -> usize {
    input
        .iter()
        .filter(|(_p, height)| *height == &0)
        .map(|(p, _)| possible_route(input, p, &mut HashSet::default()).len())
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &TopoMap) -> usize {
    input
        .iter()
        .filter(|(_p, height)| *height == &0)
        .map(|(p, _)| rating(input, p))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"0123
1234
8765
9876"#;

    pub const FORK: &str = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

    pub const FOUR_SCORE: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    pub const TWO_TRAILHEADS: &str = r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#;

    pub const LARGER_EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    pub const THREE_RATING: &str = r#".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."#;

    pub const THIRTEEN_RATING: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1);
        assert_eq!(part1(&parse(FORK)), 2);
        assert_eq!(part1(&parse(FOUR_SCORE)), 4);
        assert_eq!(part1(&parse(TWO_TRAILHEADS)), 3);
        assert_eq!(part1(&parse(LARGER_EXAMPLE)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(THREE_RATING)), 3);
        assert_eq!(part2(&parse(THIRTEEN_RATING)), 13);
    }
}
