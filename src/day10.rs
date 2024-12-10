use crate::point::Point;
use ahash::{AHashMap as HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

pub type TopoMap = HashMap<Point, u32>;
pub type Peaks = Vec<Point>;
pub type Input = (TopoMap, Peaks);

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    let topo = input
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
        .collect::<HashMap<Point, u32>>();
    (
        topo.clone(),
        topo.iter()
            .filter(|(_p, height)| *height == &9)
            .map(|(p, _height)| *p)
            .collect(),
    )
}

fn valid_neighbours(guide_map: &TopoMap, point: &Point) -> Vec<Point> {
    point
        .cardinal_neighbours()
        .iter()
        .filter(|p| guide_map.contains_key(p))
        .cloned()
        .collect()
}

fn dfs(guide: &TopoMap, pos: &Point) -> usize {
    if let Some(height) = guide.get(pos) {
        if height == &9 {
            return 1;
        }
        valid_neighbours(guide, pos)
            .iter()
            .filter(|neighbour| guide.get(*neighbour) == Some(&(height + 1)))
            .map(|n| dfs(guide, n))
            .sum()
    } else {
        0
    }
}

fn possible_route(guide_map: &TopoMap, trail_head: Point, peak: Point) -> Option<Vec<Point>> {
    let mut visited = HashSet::default();
    let mut stack = vec![trail_head];
    while let Some(p) = stack.pop() {
        if p == peak {
            return Some(visited.into_iter().collect());
        }
        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);
        let height = guide_map.get(&p).unwrap();
        p.cardinal_neighbours()
            .iter()
            .filter_map(|n| {
                if let Some(next_height) = guide_map.get(n) {
                    if next_height == &(height + 1) && !visited.contains(n) {
                        Some(*n)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .for_each(|n| stack.push(n));
    }
    None
}

fn rating(guide_map: &TopoMap, trail_head: &Point) -> usize {
    dfs(guide_map, trail_head)
}

fn reachable_peaks(guide_map: &TopoMap, trail_head: Point, peaks: Vec<Point>) -> usize {
    peaks
        .iter()
        .filter_map(|peak| possible_route(guide_map, trail_head, peak.clone()))
        .count()
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> usize {
    let trailheads: Vec<Point> = input
        .0
        .iter()
        .filter(|(_p, height)| *height == &0)
        .map(|(p, _)| p)
        .cloned()
        .collect();
    trailheads
        .into_iter()
        .map(|p| reachable_peaks(&input.0, p, input.1.clone()))
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> usize {
    let trailheads: Vec<Point> = input
        .0
        .iter()
        .filter(|(_p, height)| *height == &0)
        .map(|(p, _)| p)
        .cloned()
        .collect();
    trailheads.iter().map(|p| rating(&input.0, p)).sum()
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
