use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
enum Key {
    Up = 0,
    Activate = 1,
    Left = 2,
    Down = 3,
    Right = 4,
}

fn get_paths<const HOLE_Y: usize>(
    paths: &mut Vec<Vec<Key>>,
    direction_key_positions: &[[usize; 2]],
    start: usize,
    end: usize,
) {
    let [start_x, start_y] = direction_key_positions[start];
    let [end_x, end_y] = direction_key_positions[end];

    if !(start_x == 0 && end_y == HOLE_Y) {
        // Start by going vertically and then horizontally
        // This must not be done if we start on the left button and go to the top row, as that would make us pass
        // over an empty space.
        let mut path = Vec::new();
        if start_y < end_y {
            path.extend((start_y..end_y).map(|_| Key::Down));
        } else if start_y > end_y {
            path.extend((end_y..start_y).map(|_| Key::Up));
        }
        if start_x < end_x {
            path.extend((start_x..end_x).map(|_| Key::Right));
        } else if start_x > end_x {
            path.extend((end_x..start_x).map(|_| Key::Left));
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(Key::Activate);
        paths.push(path);
    }

    if start_x != end_x && start_y != end_y && !(start_y == HOLE_Y && end_x == 0) {
        // If we need to both vertically and horizontally, we can also do it by going horizontally first.
        // This must not be done if we end on the left button, as that would make us pass over an empty space.
        let mut path = Vec::new();
        if start_x < end_x {
            path.extend((start_x..end_x).map(|_| Key::Right));
        } else if start_x > end_x {
            path.extend((end_x..start_x).map(|_| Key::Left));
        }
        if start_y < end_y {
            path.extend((start_y..end_y).map(|_| Key::Down));
        } else if start_y > end_y {
            path.extend((end_y..start_y).map(|_| Key::Up));
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(Key::Activate);
        paths.push(path);
    }

    // It is never worth zigzagging, as such paths can be reduced into a non-zigzagging path just by duplicating
    // some presses while eliminating others, to get a path that takes fewer presses in total.
}

fn calc_level_costs(previous_costs: &[usize], paths: &[Vec<Vec<Key>>]) -> Vec<usize> {
    paths
        .iter()
        .map(|paths| {
            paths
                .iter()
                .map(|path| {
                    // Sum up the costs of going from each button to the next one and pressing it, starting from Activate
                    let mut pos = Key::Activate;
                    path.iter()
                        .map(|&new_pos| {
                            let cost = previous_costs[pos as usize * 5 + new_pos as usize];
                            pos = new_pos;
                            cost
                        })
                        .sum()
                })
                .min()
                .unwrap()
        })
        .collect()
}
fn calc_key_costs(size: usize) -> Vec<usize> {
    let key_pos = [[1, 0], [2, 0], [0, 1], [1, 1], [2, 1]];
    let paths: Vec<_> = (0..25)
        .map(|i| {
            let mut paths = Vec::new();
            let start = i / 5;
            let end = i % 5;
            get_paths::<0>(&mut paths, &key_pos, start, end);
            paths
        })
        .collect();
    let mut path_costs = paths
        .iter()
        .map(|paths| paths.iter().map(|path| path.len()).min().unwrap())
        .collect::<Vec<usize>>();
    for _ in 0..size - 1 {
        path_costs = calc_level_costs(&path_costs, &paths);
    }
    path_costs
}

fn solve(input: &str, keypads: usize) -> usize {
    let key_costs = calc_key_costs(keypads);
    let key_pos = [
        [1, 3],
        [0, 2],
        [1, 2],
        [2, 2],
        [0, 1],
        [1, 1],
        [2, 1],
        [0, 0],
        [1, 0],
        [2, 0],
        [2, 3],
    ];
    let mut paths = Vec::new();
    input
        .lines()
        .map(|combo| {
            let number: usize = combo.split('A').next().unwrap().parse().unwrap();
            let mut pos = key_pos.len() - 1;
            combo
                .chars()
                .map(|key| {
                    let new_pos = match key {
                        '0'..='9' => key as usize - '0' as usize,
                        'A' => 10,
                        _ => panic!("Invalid character: {}", key),
                    };
                    get_paths::<3>(&mut paths, &key_pos, pos, new_pos);
                    let cost: usize = paths
                        .iter()
                        .map(|path| {
                            let mut pos = Key::Activate;
                            path.iter()
                                .map(|&new_pos| {
                                    let cost = key_costs[pos as usize * 5 + new_pos as usize];
                                    pos = new_pos;
                                    cost
                                })
                                .sum()
                        })
                        .min()
                        .unwrap();
                    pos = new_pos;
                    paths.clear();
                    cost
                })
                .sum::<usize>()
                * number
        })
        .sum()
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
