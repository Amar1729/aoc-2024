#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

fn solve(input: &str, parse: fn(&str) -> u64) -> u64 {
    parse(input)
}

fn parse_input(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let mut it = input.lines();

    let towels = it.next().unwrap()
        .split(", ")
        .collect();

    _ = it.next();

    let patterns = it.collect();

    (towels, patterns)
}

fn count_possible<'a>(pattern: &'a str, towels: &HashSet<&str>, memo: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(&c) = memo.get(pattern) { return c; }

    if pattern.trim().is_empty() { return 1; }

    let mut count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            count += count_possible(&pattern[towel.len()..], &towels, memo);
        }
    }

    *memo.entry(pattern).or_insert(0) += count;
    count
}

fn parse1(input: &str) -> u64 {
    let (towels, patterns) = parse_input(input);
    let mut memo = HashMap::new();

    patterns
        .iter()
        .filter(|&pattern| count_possible(&pattern, &towels, &mut memo) > 0)
        .collect::<Vec<_>>()
        .len() as u64
}

fn parse2(input: &str) -> u64 {
    let (towels, patterns) = parse_input(input);
    let mut memo = HashMap::new();

    patterns
        .iter()
        .map(|&pattern| count_possible(&pattern, &towels, &mut memo))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 6);
        assert_eq!(solve(INPUT, parse1), 296);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 16);
        assert_eq!(solve(INPUT, parse2), 619970556776002);
    }
}
