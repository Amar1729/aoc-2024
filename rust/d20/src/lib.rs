#![allow(dead_code)]

use pathfinding::prelude::astar_bag;
use itertools::Itertools;
use utils::{Point, parse_with_lens};
use std::collections::HashSet;

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
                    Some(Point::from(p))
                    // None
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

fn print_grid(start: &Point, end: &Point, cheat: &Point, allowed: &HashSet<Point>, path1: &[Point], path2: &[Point]) {
    let (mx, my) = max_xy(allowed);

    for y in 0 .. my+1 {
        for x in 0 .. mx+1 {
            let p = &Point { x, y };
            if start == p { print!("S") }
            else if end == p { print!("E") }
            else if cheat == p { print!("X") }
            else if path1.contains(&p) || path2.contains(&p) { print!("O") }
            else if allowed.contains(&p) { print!(".") }
            else { print!("#") }
        }
        println!();
    }
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn find_path(start: &Point, end: &Point, allowed: &HashSet<Point>) -> Option<Vec<Vec<Point>>> {
    let result = astar_bag(
        start,
        |&p| {
            [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ]
                .into_iter()
                .filter_map(|adj_step| {
                    let adj = p + Point::from(adj_step);
                    match allowed.contains(&adj) {
                        true => Some((adj, 1)),
                        false => None,
                    }
                })
                .collect::<Vec<(Point, usize)>>()
        },
        |&p| ((end.x as isize).abs_diff(p.x) + (end.y as isize).abs_diff(p.y)) / 2,
        |&p| p == *end,
    );

    if let Some(result) = result {
        Some(result.0.collect())
    } else {
        None
    }
}

fn max_xy(allowed: &HashSet<Point>) -> (isize, isize) {
    let mut mx = 0;
    let mut my = 0;
    for p in allowed {
        if p.x > mx { mx = p.x }
        if p.y > my { my = p.y }
    }

    (mx, my)
}

fn find_possible_cheats(allowed: &HashSet<Point>) -> HashSet<Point> {
    let (mx, my) = max_xy(allowed);

    let mut cheats = HashSet::new();

    for x in 0 .. mx+1 {
        for y in 0 .. my+1 {

            let directions = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ].into_iter();

            for v in directions.permutations(2) {
                let p = Point { x, y };
                let first = p + Point::from(v[0]);
                let second = p + Point::from(v[1]);

                if !allowed.contains(&p) && allowed.contains(&first) && allowed.contains(&second) {
                    cheats.insert(p);
                }
            }
        }
    }

    cheats
}

fn find_cheats(start: &Point, end: &Point, allowed: &HashSet<Point>) -> Vec<(usize, usize)> {
    let shortest_path = find_path(&start, &end, &allowed).unwrap();
    let shortest_len = shortest_path.iter().next().unwrap().len();

    let full = find_possible_cheats(&allowed).len();

    find_possible_cheats(&allowed)
        .iter()
        .enumerate()
        .flat_map(|(idx, &cheat)| {
            let mut new_allowed = allowed.clone();
            new_allowed.insert(cheat);

            let mut possible_cheats = Vec::new();

            println!("Checking cheat: {idx} / {full} \t {cheat:?}");

            if let Some(paths_first) = find_path(&start, &cheat, &new_allowed) {
                for path1 in paths_first {
                    let mut so_far = new_allowed.clone();
                    for so_far_point in path1.iter() {
                        so_far.insert(*so_far_point);
                    }

                    if let Some(paths_second) = find_path(&cheat, &end, &so_far) {
                        for path2 in paths_second {
                            // don't double-count cheat node
                            let distance = path1.len() + path2.len() - 1;
                            if shortest_len > distance {
                                // return Some(shortest_len - distance)
                                possible_cheats.push((shortest_len - distance, (cheat, path2[1])));

                                let diff = shortest_len - distance;
                                // println!("\n{diff}");
                                // print_grid(&start, &end, &cheat, &allowed, &path1, &path2);
                                // println!();
                            }
                        }
                    }
                }

            }

            possible_cheats
            // None
        })
        .sorted_by(|(d1, _), (d2, _)| Ord::cmp(d1, d2))
        .map(|(d, _)| d)
        .dedup_with_count()
        .collect()
}

fn parse1(input: &str) -> u32 {
    let (start, end, allowed) = parse_grid(input);
    find_cheats(&start, &end, &allowed)
        .iter()
        .map(|&(c, d)| {
            match d >= 100 {
                true => c,
                false => 0,
            }
        })
        .sum::<usize>() as u32
}

fn parse2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        // all cheats for sample (but none save more than 64)
        // assert_eq!(solve(TEST1, parse1), 44);
        assert_eq!(solve(INPUT, parse1), 1289);
    }

    #[test]
    fn test2() {
        // assert_eq!(solve(TEST1, parse2), 0);
        // assert_eq!(solve(INPUT, parse2), 0);
    }
}
