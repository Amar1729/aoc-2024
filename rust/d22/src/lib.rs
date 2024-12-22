#![allow(dead_code)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn solve(input: &str, parse: fn(&str) -> u64) -> u64 {
    parse(input)
}

/// 1. s * 64
/// 1a. mix, then prune
/// 2. s / 32
/// 2a. mix, then prune
/// 3. s * 2048
/// 3a. mix, then prune
///
/// mix = bitwise XOR
/// prune = mod 16777216
fn evolve(sec: u64) -> u64 {
    let m = (2 as u64).pow(24);

    let sec = ((sec << 6) ^ sec) % m;
    let sec = ((sec >> 5) ^ sec) % m;
    let sec = ((sec << 11) ^ sec) % m;

    sec
}

fn parse1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut sec = line.parse().unwrap();
            for _ in 0 .. 2000 {
                sec = evolve(sec);
            }
            sec
        })
        .sum()
}

fn parse2(input: &str) -> u64 {
    let mut summed = HashMap::new();

    // TODO: profile this? runs ~slowly (~4.7s).
    for line in input.lines() {
        // experimentally determined (slightly helps runtime, avoids lots small memallocs)
        let mut seen = HashSet::with_capacity(3584);
        let mut sec = line.parse().unwrap();
        let mut prev = sec % 10;
        let mut offsets = VecDeque::with_capacity(3);
        for idx in 0 .. 2000 {
            sec = evolve(sec);
            let digit = sec % 10;
            if idx > 2 {
                let leadup = (
                    offsets[0],
                    offsets[1],
                    offsets[2],
                    digit as isize - prev as isize,
                );

                if !seen.contains(&leadup) {
                    seen.insert(leadup);
                    *summed.entry(leadup).or_insert(0) += digit as usize;
                }

                offsets.pop_front();
            }
            offsets.push_back(digit as isize - prev as isize);
            prev = sec % 10;
        }
    }

    let result = summed
        .iter()
        // sort in descending order, by number of bananas of this sequence
        .sorted_by(|a, b| Ord::cmp(b.1, a.1))
        .next()
        .unwrap();

    *result.1 as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"1
10
100
2024"#;

    // sneaky aoc writers swapping out the example on us
    const TEST2: &'static str = r#"1
2
3
2024"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test_evolve() {
        assert_eq!(evolve(123), 15887950);
    }

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 37327623);
        assert_eq!(solve(INPUT, parse1), 19877757850);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST2, parse2), 23);
        assert_eq!(solve(INPUT, parse2), 2399);
    }
}
