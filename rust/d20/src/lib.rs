#![allow(dead_code)]

use rayon::prelude::*;
use utils::{Point, parse_with_lens};
use std::collections::{HashMap, HashSet};

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

fn print_grid(start: &Point, end: &Point, cheats: &[Point], allowed: &HashSet<Point>, path1: &[Point], path2: &[Point]) {
    let (mx, my) = max_xy(allowed);

    for y in 0 .. my+1 {
        for x in 0 .. mx+1 {
            let p = &Point { x, y };
            if start == p { print!("S") }
            else if end == p { print!("E") }
            else if cheats.contains(&p) { print!(" ") }
            else if path1.contains(&p) || path2.contains(&p) { print!("O") }
            else if allowed.contains(&p) { print!(".") }
            else { print!("#") }
        }
        println!();
    }
}

fn solve(input: &str, threshold: usize, parse: fn(&str, usize) -> u32) -> u32 {
    parse(input, threshold)
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

/// nice and simple, since we're on a track with no dead-ends only 1 point wide.
fn make_path(start: &Point, end: &Point, allowed: &HashSet<Point>) -> Vec<Point> {
    let mut curr = *start;
    let mut path = vec![curr];

    while curr != *end {
        for succ in curr.successors(1) {
            if allowed.contains(&succ) && !path.contains(&succ) {
                curr = succ;
                path.push(curr);
                break;
            }
        }
    }

    path
}

fn count_cheats(input: &str, cheat_len: usize, threshold: usize) -> u32 {
    let (start, end, allowed) = parse_grid(input);
    // let path = make_path(&start, &end, &allowed);
    let path_with_distances: HashMap<Point, usize> = make_path(&start, &end, &allowed)
        .into_iter()
        .rev()
        .enumerate()
        .map(|(idx, p)| (p, idx))
        .collect();

    path_with_distances
        // slap a par in there, get a 4x speedup, ez
        .par_iter()
        .map(|(p, orig)| {
            // find locations at most cheat_len spaces away
            p.successors(cheat_len)
                .iter()
                .filter_map(|other| {
                    if let Some(dist) = path_with_distances.get(other) {
                        if *dist > orig + p.manhattan(other) && dist - orig - p.manhattan(other) >= threshold {
                            // println!("Distance: {}", dist - orig - p.manhattan(other));
                            // print_grid(&start, &end, &[*p, *other], &allowed, &path, &[]);
                            // println!();
                            return Some(dist - orig - p.manhattan(other))
                        }
                    }

                    None
                })
                .count()
        })
        .sum::<usize>() as u32
}

fn parse1(input: &str, threshold: usize) -> u32 {
    count_cheats(input, 2, threshold)
}

fn parse2(input: &str, threshold: usize) -> u32 {
    count_cheats(input, 20, threshold)
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
        assert_eq!(solve(TEST1, 0, parse1), 44);
        assert_eq!(solve(INPUT, 100, parse1), 1289);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, 50, parse2), 285);
        assert_eq!(solve(INPUT, 100, parse2), 982425);
    }
}
