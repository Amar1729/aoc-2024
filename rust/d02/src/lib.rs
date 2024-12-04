#![allow(dead_code)]

fn diffs(report: Vec<isize>) -> Vec<isize> {
    let mut it = report.iter();
    let mut curr = it.next().unwrap();

    it
        .map(|level| {
            let d = level - curr;
            curr = level;
            d
        })
        .collect()
}

fn check_levels(report: Vec<isize>) -> bool {
    let mut prev = 0;

    for d in diffs(report) {
        if d < 0 && prev > 0 {
            return false;
        }

        if d > 0 && prev < 0 {
            return false;
        }

        if d < -3 || d == 0 || d > 3 {
            return false;
        }

        prev = d;
    }

    true
}

fn check_levels_dumb(report: Vec<isize>) -> bool {
    // i apologize for my crimes against humanity

    if check_levels(report.clone()) {
        return true;
    }

    for x in 0 .. report.len() {
        let mut new_report = report[0 .. x].to_vec();
        new_report.extend_from_slice(&report[x+1 .. report.len()]);
        if check_levels(new_report) {
            return true;
        }
    }

    false
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .map(|report| check_levels(report))
        .filter(|&b| b)
        .count() as u32
}

fn parse2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .map(|report| check_levels_dumb(report))
        .filter(|&b| b)
        .count() as u32
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
        assert_eq!(solve(TEST1, parse2), 4);
        assert_eq!(solve(INPUT, parse2), 612);
    }
}
