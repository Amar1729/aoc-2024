#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

fn make_graph(input: &str) -> (HashSet<&str>, HashMap<&str, Vec<&str>>) {
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

    (nodes, edges)
}

fn parse1(input: &str) -> u32 {
    let (_, edges) = make_graph(&input);

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

fn parse2(input: &str) -> String {
    let (nodes, edges) = make_graph(&input);

    let mut largest_group = vec![];
    for node in nodes {
        let mut group = vec![node];
        if let Some(neighbors) = edges.get(node) {
            for neighbor in neighbors {
                if group
                    .iter()
                    .all(|n| {
                        edges.get(neighbor).unwrap()
                            .contains(n)
                    }) {
                        group.push(neighbor);
                }
            }

            group.sort();
            if group.len() > largest_group.len() {
                largest_group = group;
            }
        }
    }

    largest_group
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .join(",")
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
        assert_eq!(parse1(TEST1), 7);
        assert_eq!(parse1(INPUT), 1411);
    }

    #[test]
    fn test2() {
        assert_eq!(parse2(TEST1), "co,de,ka,ta");
        assert_eq!(parse2(INPUT), "aq,bn,ch,dt,gu,ow,pk,qy,tv,us,yx,zg,zu");
    }
}
