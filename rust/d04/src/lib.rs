#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

const DIRECTIONS: &[(isize, isize); 8] = &[
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

/// Find XMAS
fn parse1(input: &str) -> u32 {
    let (_, it) = utils::parse_with_lens(input, &|b| {b as char});

    let mut map = HashMap::new();
    let mut start_points = HashSet::new();

    for (coord, letter) in it {
        // shift the coordinate so i can keep it as usize and still do subtraction.
        let shifted = (coord.0 + 3, coord.1 + 3);

        map.insert(shifted, letter);

        if letter == 'X' {
            start_points.insert(shifted);
        }
    }

    start_points.iter()
        .map(|(x, y)| {
            DIRECTIONS.iter()
                .map(|(dx, dy)| {
                    for (d, target) in ['M', 'A', 'S'].iter().enumerate() {
                        // gross? what am i missing here
                        let nx = ((d+1) as isize * dx + *x as isize) as usize;
                        let ny = ((d+1) as isize * dy + *y as isize) as usize;

                        match map.get(&(nx as isize, ny as isize)) {
                            Some(b) => {
                                if b != target { return 0; }
                            },
                            None => { return 0; }
                        };
                    }

                    return 1;
                })
                .sum::<u32>()
        })
        .sum()
}

/// Find an X'd MAS (two MAS in shape of X)
fn parse2(input: &str) -> u32 {
    let (_, it) = utils::parse_with_lens(input, &|b| {b as char});

    let mut map = HashMap::new();
    let mut start_points = HashSet::new();

    for (coord, letter) in it {
        // shift the coordinate so i can keep it as usize and still do subtraction.
        let shifted = (coord.0 + 3, coord.1 + 3);

        map.insert(shifted, letter);

        if letter == 'A' {
            start_points.insert(shifted);
        }
    }

    start_points.iter()
        .filter_map(|(x, y)| {
            // let tl = *map.get(&(x-1, y+1))? as u8;
            // let tr = *map.get(&(x+1, y+1))? as u8;
            // let bl = *map.get(&(x-1, y-1))? as u8;
            // let br = *map.get(&(x+1, y-1))? as u8;

            let mut p1 = [
                *map.get(&(x-1, y+1))? as u8,
                *map.get(&(x+1, y-1))? as u8,
            ];
            p1.sort();

            let mut p2 = [
                *map.get(&(x+1, y+1))? as u8,
                *map.get(&(x-1, y-1))? as u8,
            ];
            p2.sort();

            // i had this idea for checking that each pair consists of exactly one S and one M.
            // i wasn't confident it would give the right answer - turns out it does, same as
            // creating the sorted slice and checking the values explicitly.
            // if (tl ^ br == 30) && (tr ^ bl == 30) {

            if (p1 == [77, 83]) && (p2 == [77, 83]) {
                Some(1)
            } else {
                Some(0)
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 18);
        assert_eq!(solve(INPUT, parse1), 2464);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 9);
        assert_eq!(solve(INPUT, parse2), 1982);
    }
}
