#![allow(dead_code)]

fn solve(input: &str, parse: fn(&str) -> u64) -> u64 {
    parse(input)
}

fn make_block_map(input: &str) -> Vec<Option<usize>> {
    input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(idx, c)| {
            (0 .. c.to_digit(10).unwrap())
                .map(move |_| {
                    match idx % 2 {
                        0 => Some(idx / 2),
                        1 => None,
                        _ => panic!(),
                    }
                })
        })
        .collect()
}

fn parse1(input: &str) -> u64 {
    let mut blocks = make_block_map(input);

    let mut small = 0;
    let mut big = blocks.len();

    while small < big {
        small = blocks.iter().position(|e| e.is_none()).unwrap();
        big = blocks.len() - blocks.iter().rev().position(|e| e.is_some()).unwrap() - 1;

        if small >= big { break }

        blocks[small] = blocks[big];
        blocks[big] = None;
    }

    blocks
        .iter()
        .enumerate()
        .filter_map(|(idx, &maybe_b)| {
            match maybe_b {
                Some(b) => Some((idx * b) as u64),
                None => None,
            }
        })
        .sum()
}

fn parse2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"2333133121414131402"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 1928);
        assert_eq!(solve(INPUT, parse1), 6370402949053);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
