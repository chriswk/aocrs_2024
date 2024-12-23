use std::collections::BTreeSet;

use ahash::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    edges: Vec<usize>,
}

#[derive(Debug, Default)]
struct Graph {
    nodes: Vec<Node>,
    seen: HashMap<String, usize>,
}

impl Graph {
    fn add_node(&mut self, name: &str) -> usize {
        if let Some(&id) = self.seen.get(name) {
            return id;
        }
        let id = self.nodes.len();
        self.seen.insert(name.to_string(), id);
        self.nodes.push(Node {
            name: name.to_string(),
            edges: vec![],
        });
        id
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        let a = self.add_node(a);
        let b = self.add_node(b);
        self.nodes[a].edges.push(b);
        self.nodes[b].edges.push(a);
    }
}
type Cycle = BTreeSet<usize>;
fn find_cycle(
    g: &Graph,
    start: usize,
    cur: usize,
    seen: &mut BTreeSet<usize>,
    depth: usize,
    found: &mut HashSet<Cycle>,
) {
    seen.insert(cur);
    if depth == 0 {
        if g.nodes[cur].edges.contains(&start) {
            let cycle = seen.clone();
            found.insert(cycle);
        }
        seen.remove(&cur);
        return;
    }
    for &next in &g.nodes[cur].edges {
        if seen.contains(&next) {
            continue;
        }
        find_cycle(g, start, next, seen, depth - 1, found);
    }
    seen.remove(&cur);
}

fn find_max_clique(graph: &Graph) -> HashSet<usize> {
    fn bron_kerbosch(
        graph: &Graph,
        r: &mut HashSet<usize>,
        p: &mut HashSet<usize>,
        x: &mut HashSet<usize>,
        max_clique: &mut HashSet<usize>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > max_clique.len() {
                max_clique.clear();
                max_clique.extend(r.iter());
            }
            return;
        }
        let pivot = p
            .iter()
            .chain(x.iter())
            .max_by_key(|&&v| {
                graph.nodes[v]
                    .edges
                    .iter()
                    .cloned()
                    .filter(|&c| p.contains(&c))
                    .count()
            })
            .cloned();
        if let Some(u) = pivot {
            let pivot_neighbours = graph.nodes[u].edges.iter().cloned().collect::<HashSet<_>>();
            let p_copy = p.clone();
            for &v in p_copy.difference(&pivot_neighbours) {
                r.insert(v);
                let v_neighbours: HashSet<_> = graph.nodes[v].edges.iter().cloned().collect();
                let mut new_p = p
                    .intersection(&v_neighbours)
                    .cloned()
                    .collect::<HashSet<_>>();
                let mut new_x = x
                    .intersection(&v_neighbours)
                    .cloned()
                    .collect::<HashSet<_>>();
                bron_kerbosch(graph, r, &mut new_p, &mut new_x, max_clique);
                r.remove(&v);
                p.remove(&v);
                x.insert(v);
            }
        }
    }
    let mut max_clique = HashSet::default();
    let mut r = HashSet::default();
    let mut p: HashSet<_> = (0..graph.nodes.len()).collect();
    let mut x = HashSet::default();
    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut max_clique);
    max_clique
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Graph {
    let mut g = Graph::default();
    for line in input.lines() {
        let (a, b) = line.trim().split_once('-').unwrap();
        g.add_edge(a, b);
    }
    g
}

#[aoc(day23, part1)]
fn part1(graph: &Graph) -> usize {
    let mut cycle = HashSet::default();
    for (name, &id) in graph.seen.iter() {
        if name.starts_with('t') {
            find_cycle(graph, id, id, &mut BTreeSet::new(), 2, &mut cycle);
        }
    }
    cycle.len()
}

#[aoc(day23, part2)]
fn part2(graph: &Graph) -> String {
    find_max_clique(graph)
        .into_iter()
        .map(|i| graph.nodes[i].name.as_str())
        .sorted()
        .collect_vec()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &str = r#"kh-tc
    qp-kh
    de-cg
    ka-co
    yn-aq
    qp-ub
    cg-tb
    vc-aq
    tb-ka
    wh-tc
    yn-cg
    kh-ub
    ta-co
    de-co
    tc-td
    tb-wq
    wh-td
    ta-ka
    td-qp
    aq-cg
    wq-ub
    ub-vc
    de-ta
    wq-aq
    wq-vc
    wh-yn
    ka-de
    kh-ta
    co-tc
    wh-qp
    tb-vc
    td-yn"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
