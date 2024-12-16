#![allow(dead_code)]

use std::iter;
use itertools::Itertools;

// need u64 for this problem
fn solve(input: &str, parse: fn(&str) -> u64) -> u64 {
    parse(input)
}

fn is_valid_line(total: usize, inputs: &[usize], bound: usize) -> bool {

    // a bit annoying that product([], repeat=n) is so verbose
    for bitstring in iter::repeat_n(0..bound, inputs.len() - 1).multi_cartesian_product() {

        let tot = inputs
            .iter()
            .copied()
            .enumerate()
            .reduce(|(_, acc), (op_idx, num)| {
                match bitstring[op_idx - 1] {
                    0 => (0, acc + num),
                    1 => (1, acc * num),
                    2 => (
                        2,
                        (acc.to_string() + &num.to_string()).parse().unwrap(),
                    ),
                    _ => panic!(),
                }
            })
            .unwrap().1;

        if tot == total { return true };
    }

    false
}

fn parse_and_solve(input: &str, bound: usize) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let total: usize = parts[0].parse().unwrap();
            let nums: Vec<usize> = parts[1].split(' ').map(|n| n.parse().unwrap()).collect();

            if is_valid_line(total, &nums, bound) {
                Some(total as u64)
            } else {
                None
            }
        })
        .sum()
}

fn parse1(input: &str) -> u64 {
    parse_and_solve(input, 2)
}

fn parse2(input: &str) -> u64 {
    parse_and_solve(input, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test_is_valid_line() {
        assert_eq!(is_valid_line(190, &[10, 19], 2), true);
        assert_eq!(is_valid_line(3267, &[81, 40, 27], 2), true);
        assert_eq!(is_valid_line(83, &[17, 5], 2), false);
    }

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 3_749);
        assert_eq!(solve(INPUT, parse1), 1_298_300_076_754);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 11_387);
        assert_eq!(solve(INPUT, parse2), 248_427_118_972_289);
    }
}
