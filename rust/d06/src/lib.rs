#![allow(dead_code)]

use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Guard {
    x: isize,
    y: isize,
    // 0 is right, 1 is up, 2 is left, 3 is down
    facing: usize,
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse_input(input: &str) -> (Guard, HashSet<Point>, Point) {
    let mut obstacles = HashSet::new();
    let mut guard = Guard { x: 0, y: 0, facing: 1 };
    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        width = line.len() as isize;

        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => { _ = obstacles.insert(Point { x: x as isize, y: y as isize }) },
                '.' => {},
                '^' => {
                    guard = Guard { x: x as isize, y: y as isize, facing: 1 }
                },
                _ => panic!(),
            }
        }

        height = y as isize;
    }

    (
        guard,
        obstacles,
        Point { x: width, y: height + 1 },
    )
}

fn traverse(
    guard: &mut Guard,
    obstacles: &HashSet<Point>,
    bounds: &Point,
) -> Option<HashSet<(isize, isize)>> {
    let mut points = HashSet::new();
    points.insert(guard.clone());

    loop {
        match guard.facing {
            // right
            0 => {
                if !obstacles.contains(&Point { x: guard.x + 1, y: guard.y }) {
                    guard.x = guard.x + 1;
                } else {
                    guard.facing = 3;
                }
            },
            // up
            1 => {
                if !obstacles.contains(&Point { x: guard.x, y: guard.y - 1 }) {
                    guard.y = guard.y - 1;
                } else {
                    guard.facing = 0;
                }
            }
            // left
            2 => {
                if !obstacles.contains(&Point { x: guard.x - 1, y: guard.y }) {
                    guard.x = guard.x - 1;
                } else {
                    guard.facing = 1;
                }
            },
            // down
            3 => {
                if !obstacles.contains(&Point { x: guard.x, y: guard.y + 1 }) {
                    guard.y = guard.y + 1;
                } else {
                    guard.facing = 2;
                }
            }
            _ => panic!(),
        }

        if guard.x < 0 || guard.x >= bounds.x || guard.y < 0 || guard.y >= bounds.y {

            // // output display
            // for y in 0 .. bounds.y {
            //     for x in 0 .. bounds.x {
            //         if points.contains(&(x, y)) {
            //             print!("X");
            //         } else if obstacles.contains(&Point{ x, y }) {
            //             print!("#");
            //         } else {
            //             print!(".")
            //         }
            //     }
            //     println!();
            // }

            // for p2 we want to first know all the points on the path.
            // return points.len() as u32;
            // deduplicate
            let mut dedup = HashSet::new();
            for point in points.iter().map(|p| ( p.x, p.y )).collect::<Vec<(isize, isize)>>() {
                dedup.insert(point);
            }
            return Some(dedup);
        }

        if points.contains(&guard) {
            // guard's already been here while facing the current direction. we're in a loop.
            return None;
        } else {
            points.insert(guard.clone());
        }
    }
}

fn parse1(input: &str) -> u32 {
    let (mut guard, obstacles, bounds) = parse_input(input);
    // assumed to work for p1
    traverse(&mut guard, &obstacles, &bounds).unwrap().len() as u32
}

fn parse2(input: &str) -> u32 {
    let (mut guard, obstacles, bounds) = parse_input(input);
    let orig_guard: Guard = guard.clone();
    traverse(&mut guard, &obstacles, &bounds)
        .unwrap()
        // absolute efficiency hack, boom
        .par_iter()
        .filter_map(|point| {
            let mut new_obstacles = obstacles.clone();
            // new_obstacles.insert(Point { x: point.x, y: point.y });
            new_obstacles.insert(Point { x: point.0, y: point.1 });
            match traverse(&mut orig_guard.clone(), &new_obstacles, &bounds) {
                Some(_) => None,
                None => Some(1),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 41);
        assert_eq!(solve(INPUT, parse1), 4789);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 6);
        assert_eq!(solve(INPUT, parse2), 1304);
    }
}
