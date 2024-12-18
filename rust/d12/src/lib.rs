#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use utils::{Point, parse_with_lens};

const DIRECTIONS: &[(isize, isize); 4] = &[
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

fn pricing(region: &HashSet<Point>) -> u32 {
    let a = region.len();
    let p = region
        .iter()
        .flat_map(|p| {
            DIRECTIONS.iter().map(|&d| {
                !region.contains(&(*p + Point::from(d))) as usize
            })
        })
        .sum::<usize>();

    (a * p) as u32
}

fn parse_input(input: &str) -> HashMap<Point, char> {
    let (_, it) = parse_with_lens(input, &|b| b as char);
    it.map(|(p, c)| (Point::from(p), c)).collect()
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse1(input: &str) -> u32 {
    let mut price = 0;

    let mut grid: Vec<(Point, char)> = parse_input(input).into_iter().collect();

    // for (k, v) in &grid {
    //     println!("{:?}, {v}", k);
    // }

    while !grid.is_empty() {
        let mut region = HashSet::new();
        let mut adj_queue = Vec::new();
        let (point, label) = grid.pop().unwrap();

        region.insert(point);
        adj_queue.extend(DIRECTIONS.iter().map(|&d| point + Point::from(d)));

        while let Some(np) = adj_queue.pop() {
            if let Some(idx) = grid.iter().position(|&e| e == (np, label)) {
                grid.remove(idx);
                region.insert(np);
                adj_queue.extend(DIRECTIONS.iter().map(|&d| np + Point::from(d)));
            }
        }

        price += pricing(&region);
    }

    price
}

fn parse2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const TEST2: &'static str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 140);
        assert_eq!(solve(TEST2, parse1), 1930);
        assert_eq!(solve(INPUT, parse1), 1467094);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(TEST2, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
