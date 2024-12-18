#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

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

fn count_sides(min: usize, max: usize, direction: Point, region: &HashSet<Point>) -> usize {
    let p_check = |p: Point| {
        if direction.y == 0 { p.x }
        else { p.y }
    };

    let p_other = |p: Point| {
        if direction.y == 0 { p.y }
        else { p.x }
    };

    (min .. max)
        .map(|c| {
            region
                .iter()
                .filter_map(|&p| {
                    if (p_check(p) as usize) == c && !region.contains(&(p + direction)) {
                        Some(p_other(p) as usize)
                    } else {
                        None
                    }
                })
                .sorted()
                .into_iter()
                .coalesce(|a, b| {
                    if (b - a) == 1 {
                        Ok(b)
                    } else {
                        Err((a, b))
                    }
                })
                .collect::<Vec<usize>>()
                .len()
        })
        .sum()
}

fn pricing_2(region: &HashSet<Point>) -> u32 {
    // area is easy
    let a = region.len();

    // this one is tougher
    let (mut sx, mut sy, mut mx, mut my) = (usize::MAX, usize::MAX, 0, 0);
    for p in region {
        if (p.x as usize) < sx { sx = p.x as usize }
        if (p.y as usize) < sy { sy = p.y as usize }
        if (p.x as usize) > mx { mx = p.x as usize }
        if (p.y as usize) > my { my = p.y as usize }
    }

    let n_sides = &[
        // left
        (sx, mx, Point { x: -1, y: 0 }),
        // right
        (sx, mx, Point { x: 1, y: 0 }),

        // down
        (sy, my, Point { x: 0, y: 1 }),
        // up
        (sy, my, Point { x: 0, y: -1 }),
    ]
        .iter()
        .map(|&(min_r, max_r, direction)| count_sides(min_r, max_r + 1, direction, &region))
        .sum();

    (a * n_sides) as u32
}

fn parse_input(input: &str) -> HashMap<Point, char> {
    let (_, it) = parse_with_lens(input, &|b| b as char);
    it.map(|(p, c)| (Point::from(p), c)).collect()
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse_regions(input: &str) -> Vec<HashSet<Point>> {
    let mut regions = Vec::new();
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

        regions.push(region);
    }

    regions
}

fn parse1(input: &str) -> u32 {
    parse_regions(input)
        .iter()
        .map(|region| pricing(&region))
        .sum()
}

fn parse2(input: &str) -> u32 {
    parse_regions(input)
        .iter()
        .map(|region| pricing_2(&region))
        .sum()
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
        // TODO - this kind of takes long? ~7 sec?
        // assert_eq!(solve(INPUT, parse1), 1467094);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 80);
        assert_eq!(solve(TEST2, parse2), 1206);
        assert_eq!(solve(INPUT, parse2), 881182);
    }
}
