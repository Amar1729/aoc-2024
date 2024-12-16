#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use utils::{Point, parse_with_lens};
use itertools::Itertools;

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse_grid(input: &str) -> (usize, usize, HashMap<Point, char>) {
    let ((width, height), it) = parse_with_lens(input, &|b| b as char);

    let points = it
        .filter_map(|(p, c)| {
            if c != '.' {
                Some((Point::from(p), c))
            } else {
                None
            }
        })
        .collect();

    (width, height, points)
}

// TODO: parametrize this and move to utils.
fn print_grid(width: usize, height: usize, nodes: &HashMap<Point, char>, antinodes: &HashSet<Point>) {
    for y in 0 .. height {
        for x in 0 .. width {
            let p = Point { x: x as isize, y: y as isize };
            if let Some(c) = nodes.get(&p) {
                print!("{}", c);
            } else if antinodes.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn find_antinodes(p1: &Point, p2: &Point, slope: &Point, width: usize, height: usize, part2: bool) -> Vec<Point> {
    let mut antinodes = Vec::new();

    if part2 {
        // check in one direction
        let mut curr = p1.clone();
        while curr.contained(width, height) {
            antinodes.push(curr);
            curr += *slope;
        }

        // check in the other direction
        let mut curr = p2.clone();
        while curr.contained(width, height) {
            antinodes.push(curr);
            curr -= *slope;
        }
    } else {
        for anti in &[
            *p1 + *slope,
            *p2 - *slope,
        ] {
            if anti.contained(width, height) {
                antinodes.push(*anti);
            }
        }
    }

    antinodes
}

fn parse_and_solve(input: &str, part2: bool) -> u32 {
    let (width, height, points) = parse_grid(input);

    let labels: HashSet<char> = points
        .iter()
        .map(|(_, &v)| v)
        .collect();

    let mut antinodes: HashSet<Point> = HashSet::new();

    for label in labels {
        let filtered = points
            .iter()
            .filter_map(|(k, &v)| {
                if v == label {
                    Some(k)
                } else {
                    None
                }
            });

        for vp in filtered.combinations(2) {
            let p1 = vp[0];
            let p2 = vp[1];
            let slope = *p1 - *p2;

            for anti in find_antinodes(p1, p2, &slope, width, height, part2) {
                antinodes.insert(anti);
            }
        }
    }

    // print_grid(width, height, &points, &antinodes);
    antinodes.len() as u32
}

fn parse1(input: &str) -> u32 {
    parse_and_solve(input, false)
}

fn parse2(input: &str) -> u32 {
    parse_and_solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 14);
        assert_eq!(solve(INPUT, parse1), 396);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 34);
        assert_eq!(solve(INPUT, parse2), 1200);
    }
}
