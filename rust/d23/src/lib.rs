#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse1(input: &str) -> u32 {
    let mut edges = HashMap::new();
    let mut nodes = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let (n1, n2) = (parts[0], parts[1]);

        if !nodes.contains(&n1) {
            nodes.insert(n1);
        }

        if !nodes.contains(&n2) {
            nodes.insert(n2);
        }

        edges.entry(n1).or_insert(vec![]).push(n2);
        edges.entry(n2).or_insert(vec![]).push(n1);
    }

    let mut three_groups = HashSet::new();

    for (node, neighbors) in &edges {
        for neighbor in neighbors {
            let others = edges.get(neighbor).unwrap();
            for other in others {
                if neighbors.contains(&other) {
                    let mut group = vec![node, neighbor, other];
                    group.sort();
                    if !three_groups.contains(&group) {
                        three_groups.insert(group);
                    }
                }
    }
        }
    }

    three_groups
        .iter()
        .filter(|group| {
            group
                .iter()
                .any(|n| n.starts_with('t'))
        })
        .count() as u32
}

fn parse2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"kh-tc
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

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 7);
        assert_eq!(solve(INPUT, parse1), 1411);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
