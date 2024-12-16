#![allow(dead_code)]

use utils::{Point, parse_with_lens};
use std::collections::{HashMap, HashSet};

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse_grid(input: &str) -> HashMap<Point, usize> {
    let (_, it) = parse_with_lens(input, &|b| b as char);
    it
        .map(|(p, c)| {
            (
                Point::from(p),
                c.to_string().parse().unwrap(),
            )
        })
        .collect()
}

fn traverse(input: &str, part1: bool) -> u32 {
    let grid = parse_grid(input);

    grid
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(p, d)| {
            let mut finishes = 0;
            let mut visited = HashSet::new();
            let mut queue = vec![(*p, *d)];

            while let Some((cp, cd)) = queue.pop() {
                if part1 {
                    if visited.contains(&cp) { continue }
                    visited.insert(cp);
                }

                for adj in &[
                    cp + Point::from((-1, 0)),
                    cp + Point::from((1, 0)),
                    cp + Point::from((0, -1)),
                    cp + Point::from((0, 1)),
                ] {
                    if let Some(cost) = grid.get(&adj) {
                        if *cost == cd + 1 {
                            if *cost == 9 && (!part1 || part1 && !visited.contains(&adj)) {
                                // done!
                                finishes += 1;
                                if part1 {
                                    visited.insert(*adj);
                                }
                            } else {
                                // keep looking
                                queue.push((*adj, *cost));
                            }
                        }
                    }
                }
            }

            finishes
        })
        .sum()
}

fn parse1(input: &str) -> u32 {
    traverse(input, true)
}

fn parse2(input: &str) -> u32 {
    traverse(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"0123
1234
8765
9876"#;

    const TEST2: &'static str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 1);
        assert_eq!(solve(TEST2, parse1), 36);
        assert_eq!(solve(INPUT, parse1), 652);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST2, parse2), 81);
        assert_eq!(solve(INPUT, parse2), 1432);
    }
}
