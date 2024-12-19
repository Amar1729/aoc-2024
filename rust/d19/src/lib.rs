#![allow(dead_code)]

use std::collections::HashSet;

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
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

fn is_possible(pattern: &str, towels: &HashSet<&str>) -> bool {
    println!("Pattern: {pattern}");

    if pattern.trim().is_empty() {
        println!("Success!");
        return true;
    }

    for towel in towels {
        if pattern.starts_with(towel) {
            println!("Using towel: {towel}");
            match is_possible(&pattern[towel.len()..], &towels) {
                true => return true,
                // otherwise keep going
                false => {},
            };
        }
    }

    false
}

fn parse1(input: &str) -> u32 {
    let (towels, patterns) = parse_input(input);

    println!("{towels:?}");

    patterns
        .iter()
        .filter(|&pattern| is_possible(&pattern, &towels))
        .collect::<Vec<_>>()
        .len() as u32
}

fn parse2(input: &str) -> u32 {
    0
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
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
