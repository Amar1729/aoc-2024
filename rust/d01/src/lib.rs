#![allow(dead_code)]

use std::collections::HashMap;
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

fn map_inc(map: &mut HashMap<u32, usize>, k: u32) {
    *map.entry(k).or_default() += 1;
}

fn parse2(input: &str) -> u32 {
    let mut vals: HashMap<u32, usize> = HashMap::from([]);
    let mut sims: HashMap<u32, usize> = HashMap::from([]);

    for line in input
        .lines()
        .filter(|line| !line.trim().is_empty()) {

        // still learning, sue me.
        let mut parts = line.split_whitespace();
        let left = parts.next().unwrap().parse::<u32>().ok().unwrap();
        let right = parts.next().unwrap().parse::<u32>().ok().unwrap();

        map_inc(&mut vals, left);
        map_inc(&mut sims, right);
    }

    println!("{:?}", vals);
    println!("{:?}", sims);

    vals
        .iter()
        .map(|(&val, count)| {
            let similarity = sims.entry(val).or_default();

            val * (*count as u32) * (*similarity as u32)
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

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 31);
        assert_eq!(solve(INPUT, parse2), 23741109);
    }
}
