#![allow(dead_code)]

use std::collections::HashMap;

fn solve(input: &str, parse: fn(&str) -> u64) -> u64 {
    parse(input)
}

fn coordinates(src: char, numeric: bool) -> (isize, isize) {
    match (src, numeric) {
        ('7', true) => (0, 0),
        ('8', true) => (1, 0),
        ('9', true) => (2, 0),
        ('4', true) => (0, 1),
        ('5', true) => (1, 1),
        ('6', true) => (2, 1),
        ('1', true) => (0, 2),
        ('2', true) => (1, 2),
        ('3', true) => (2, 2),
        ('0', true) => (1, 3),
        ('A', true) => (2, 3),

        ('^', false) => (1, 0),
        ('A', false) => (2, 0),
        ('<', false) => (0, 1),
        ('v', false) => (1, 1),
        ('>', false) => (2, 1),

        _ => panic!("panik coordinates: {src} {numeric}"),
    }
}

fn generate_move(src_c: char, dst_c: char, numeric: bool) -> String {
    let src = coordinates(src_c, numeric);
    let dst = coordinates(dst_c, numeric);

    let mut horizontal = String::with_capacity(2);
    let mut vertical = String::with_capacity(3);

    if dst.0 - src.0 < 0 {
        for _ in 0 .. src.0 - dst.0 {
            horizontal.push('<');
        }
    } else {
        for _ in 0 .. dst.0 - src.0 {
            horizontal.push('>');
        }
    }

    if dst.1 - src.1 < 0 {
        for _ in 0 .. src.1 - dst.1 {
            vertical.push('^');
        }
    } else {
        for _ in 0 .. dst.1 - src.1 {
            vertical.push('v');
        }
    }

    let mut last = String::with_capacity(6);
    let bad_row = if numeric { 3 } else { 0 };

    if dst.0 > src.0 && !(dst.1 == bad_row && src.0 == 0) {
        // if moving to the right, have to prioritize vertical
        // (only if we can move vertically first)
        last.push_str(&vertical);
        last.push_str(&horizontal);
    } else if !(src.1 == bad_row && dst.0 == 0) {
        // check if we can move horizontally, then vertically.
        last.push_str(&horizontal);
        last.push_str(&vertical);
    } else if !(dst.1 == bad_row && src.0 == 0) {
        // assert that we can move vertically, then horizontally.
        last.push_str(&vertical);
        last.push_str(&horizontal);
    } else {
        panic!()
    }

    last.push('A');
    last
}

fn dfs(subseq: String, depth: usize, memo: &mut HashMap<(String, usize), u64>) -> u64 {
    if let Some(&result) = memo.get(&(subseq.to_owned(), depth)) {
        return result;
    }

    // "off-by-one" because depth=1 is the last robot, and YOU are at depth=0.
    if depth == 1 {
        *memo.entry((subseq.to_owned(), depth)).or_insert(0) += subseq.len() as u64;
        return subseq.len() as u64;
    }

    let total = "A".chars().chain(subseq.chars())
        .zip(subseq.chars())
        .map(|(left, right)| {
            let pair_seq = generate_move(left, right, false);

            if let Some(&result) = memo.get(&(pair_seq.clone(), depth-1)) {
                result
            } else {
                dfs(pair_seq, depth-1, memo)
            }
        })
        .sum();

    *memo.entry((subseq.to_owned(), depth)).or_insert(0) += total;
    total
}

/// note that n_robots does NOT include the human presser
fn presses(input: &str, n_robots: usize) -> u64 {
    let sequence = input.to_string();

    let mut memo: HashMap<(String, usize), u64> = HashMap::new();

    // each robot starts on A
    "A".chars().chain(sequence.chars())
        .zip(sequence.chars())
        .map(|(left, right)| {
            let pair_seq = generate_move(left, right, true);
            let total = dfs(pair_seq.to_owned(), n_robots, &mut memo);
            *memo.entry((pair_seq, n_robots+1)).or_insert(0) += total;

            println!("Map has grown to len / cap: {} / {}", memo.len(), memo.capacity());

            total
        })
        .sum::<u64>()
}

fn to_number(code: &str) -> usize {
    code
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn calc_presses(input: &str, n_robots: usize) -> u64 {
    input
        .lines()
        // note that there are n robots + 1 human person
        .map(|code| to_number(code) as u64 * presses(code, n_robots))
        .sum()
}

fn parse1(input: &str) -> u64 {
    calc_presses(input, 3)
}

fn parse2(input: &str) -> u64 {
    calc_presses(input, 26)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"029A
980A
179A
456A
379A"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test_generate_moves() {
        assert_eq!(generate_move('7', '8', true), ">A");
        assert_eq!(generate_move('7', '4', true), "vA");
        assert_eq!(generate_move('9', '4', true), "<<vA");
        assert_eq!(generate_move('A', '1', true), "^<<A");
        assert_eq!(generate_move('^', 'v', false), "vA");
        assert_eq!(generate_move('<', 'A', false), ">>^A");
        assert_eq!(generate_move('^', '<', false), "v<A");
    }

    #[test]
    fn test1() {
        // each test case individually
        assert_eq!(solve("029A", parse1), 68 * 29);
        assert_eq!(solve("980A", parse1), 60 * 980);
        assert_eq!(solve("179A", parse1), 68 * 179);
        assert_eq!(solve("456A", parse1), 64 * 456);
        assert_eq!(solve("379A", parse1), 64 * 379);

        assert_eq!(solve(TEST1, parse1), 126384);
        assert_eq!(solve(INPUT, parse1), 202648);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(INPUT, parse2), 248_919_739_734_728);
    }
}
