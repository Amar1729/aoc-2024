#![allow(dead_code)]

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
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"1
10
100
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
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
