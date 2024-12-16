#![allow(dead_code)]

// TODO:
// this solution is somewhat slow (why?). can i learn some profiling tools?

use std::collections::{BinaryHeap, HashMap, HashSet};
use utils::{Point, parse_with_lens};

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    facing: usize,
    pos: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.facing.cmp(&other.facing))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse_grid(input: &str) -> (Point, Point, HashSet<Point>) {
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };

    let (_, it) = parse_with_lens(input, &|b| b as char);
    let points = it
        .filter_map(|(p, c)| {
            match c {
                '.' => Some(Point::from(p)),
                'S' => {
                    start = Point::from(p);
                    None
                },
                'E' => {
                    end = Point::from(p);
                    Some(Point::from(p))
                },
                _ => None,
            }
        })
        .collect();

    (start, end, points)
}

fn parse1(input: &str) -> u32 {
    let (start, end, points) = parse_grid(input);
    let mut dist: HashMap<(Point, usize), usize> = points
        .iter()
        .flat_map(|p| {
            (0..4).map(move |f| ((*p, f), usize::MAX))
        })
        .collect();
    let mut heap = BinaryHeap::new();

    // set start to 0 cost
    dist.entry((start, 0)).insert_entry(0);
    heap.push(State { cost: 0, facing: 0, pos: start });

    while let Some(State { cost, facing, pos }) = heap.pop() {
        if pos == end { return cost as u32 }

        if let Some(new_cost) = dist.get(&(pos, facing)) {
            if cost > *new_cost { continue; }
        }

        for (adj, nf) in [
            (pos + Point::from((-1, 0)), 2),
            (pos + Point::from((1, 0)), 0),
            (pos + Point::from((0, -1)), 3),
            (pos + Point::from((0, 1)), 1),
        ] {
            let mvmt_cost = match (4 + nf - facing) % 4 {
                0 => 0,
                1 => 1000,
                2 => 2000,
                3 => 1000,
                _ => panic!(),
            } + 1;

            let next = State { cost: mvmt_cost + cost, facing: nf, pos: adj };

            if let Some(prev_adj_cost) = dist.get(&(adj, facing)) {
                if next.cost < *prev_adj_cost {
                    heap.push(next.clone());
                    dist.entry((next.pos, next.facing)).insert_entry(next.cost);
                }
            }
        }
    }

    panic!();
}

fn parse2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const TEST2: &'static str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 7036);
        assert_eq!(solve(TEST2, parse1), 11048);
        assert_eq!(solve(INPUT, parse1), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
