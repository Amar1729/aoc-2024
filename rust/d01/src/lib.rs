#![allow(dead_code)]

use std::iter::zip;

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            // still learning, sue me.
            let mut parts = line.split_whitespace();
            let left = parts.next()?.parse::<u32>().ok();
            let right = parts.next()?.parse::<u32>().ok();
            if let (Some(l), Some(r)) = (left, right) {
                Some((l, r))
            } else {
                None
            }
        })
        .unzip();

    left.sort();
    right.sort();

    zip(left, right)
        .map(|(l, r)| {
            println!("{} {}", l, r);
            if r > l {
                r - l
            } else {
                l - r
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 11);
        assert_eq!(solve(INPUT, parse1), 2166959);
    }

    // #[test]
    // fn test2() {
    //     assert_eq!(solve(TEST2, parse2), 11);
    //     assert_eq!(solve(INPUT, parse2), 0);
    // }
}
