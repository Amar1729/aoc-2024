#![allow(dead_code)]

fn check_levels(report: Vec<i32>) -> bool {
    let mut it = report.iter();
    let mut curr = it.next().unwrap();
    let mut diff;
    let mut prev = 0;

    for lvl in it {
        diff = curr - lvl;
        curr = lvl;

        if diff < 0 && prev > 0 {
            return false;
        }

        if diff > 0 && prev < 0 {
            return false;
        }

        if diff < -3 || diff == 0 || diff > 3 {
            return false;
        }

        prev = diff;
    }

    true
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .map(|report| check_levels(report))
        .filter(|&b| b)
        .count() as u32
}

fn parse2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 2);
        assert_eq!(solve(INPUT, parse1), 572);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
