#![allow(dead_code)]

fn blink(rocks: &Vec<usize>) -> Vec<usize> {
    let mut new_rocks = vec![];

    for rock in rocks {
        if *rock == 0 {
            new_rocks.push(1);
        } else if rock.to_string().len() % 2 == 0 {
            let string = rock.to_string();

            let left = &string[.. string.len() / 2];
            let right = &string[string.len() / 2 ..];

            new_rocks.push(left.parse().unwrap());
            new_rocks.push(right.parse().unwrap());
        } else {
            new_rocks.push(2024 * rock);
        }
    }

    new_rocks
}

fn solve(input: &str, blinks: usize, parse: fn(&str, usize) -> u32) -> u32 {
    parse(input, blinks)
}

fn parse1(input: &str, blinks: usize) -> u32 {
    let mut rocks: Vec<usize> = input
        .lines()
        .next().unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    println!("{:?}", rocks);

    for _ in 0 .. blinks {
        rocks = blink(&mut rocks);
    }

    rocks.len() as u32
}

fn parse2(input: &str, blinks: usize) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"0 1 10 99 999"#;

    const TEST2: &'static str = r#"125 17"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, 1, parse1), 7);
        assert_eq!(solve(TEST2, 6, parse1), 22);
        assert_eq!(solve(TEST2, 25, parse1), 55312);
        assert_eq!(solve(INPUT, 25, parse1), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, 0, parse2), 0);
        assert_eq!(solve(TEST2, 0, parse2), 0);
        assert_eq!(solve(INPUT, 25, parse2), 0);
    }
}
