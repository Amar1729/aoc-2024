#![allow(dead_code)]

use regex::Regex;

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [left, right])| { left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap() })
        .sum()
}

fn sanitize_str(input: &str) -> String {
    let re = Regex::new(r"\n").unwrap();
    let oneline = re.replace_all(input, "");

    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let cleaned = re.replace_all(&oneline, "");

    // tricksy hobbitses
    let re = Regex::new(r"don't\(\).*$").unwrap();
    re.replace_all(&cleaned, "").to_string()
}

fn parse2(input: &str) -> u32 {
    parse1(&sanitize_str(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    const TEST2: &'static str = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 161);
        assert_eq!(solve(INPUT, parse1), 171183089);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST2, parse2), 48);
        assert_eq!(solve(INPUT, parse2), 63866497);
    }
}
