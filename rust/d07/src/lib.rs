#![allow(dead_code)]

// need u64 for this problem
fn solve(input: &str, parse: fn(&str) -> u64) -> u64 {
    parse(input)
}

// i expected this to need some memoization (at least for part 2) but it turns out this
// implementation is actually just way faster than my original.
fn is_valid_line(total: usize, inputs: &[usize], bound: usize) -> bool {

    if inputs.len() == 1 {
        if inputs[0] == total {
            return true;
        } else {
            return false;
        }
    }

    let lhs = &inputs[.. inputs.len() - 1];
    let rhs = inputs[inputs.len() - 1];

    for op in 0 .. bound {
        match op {
            // addition
            0 => {
                if total > rhs && is_valid_line(total - rhs, lhs, bound) {
                    return true;
                }
            },
            // multiplication
            1 => {
                if total % rhs == 0 && is_valid_line(total / rhs, lhs, bound) {
                    return true;
                }
            },
            // concat
            2 => {
                let mut total_s = total.to_string();
                let rhs_s = rhs.to_string();
                if total_s.ends_with(&rhs_s) && total_s.len() > rhs_s.len() {
                    total_s = total_s[.. total_s.len() - rhs_s.len()].to_string();
                    if is_valid_line(total_s.parse().unwrap(), lhs, bound) {
                        return true;
                    }
                }
            },
            _ => panic!(),
        }
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
