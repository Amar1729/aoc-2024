#![allow(dead_code)]

use std::collections::HashMap;

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

fn blink_better(rocks: &HashMap<usize, u64>) -> HashMap<usize, u64> {
    let mut new_rocks = HashMap::new();

    for (rock, count) in rocks.iter() {
        if *rock == 0 {
            *new_rocks.entry(1).or_insert(0) += count;
        } else if rock.to_string().len() % 2 == 0 {
            let string = rock.to_string();

            let left = &string[.. string.len() / 2];
            let right = &string[string.len() / 2 ..];

            *new_rocks.entry(left.parse().unwrap()).or_insert(0) += count;
            *new_rocks.entry(right.parse().unwrap()).or_insert(0) += count;
        } else {
            *new_rocks.entry(2024 * *rock).or_insert(0) += count;
        }
    }

    new_rocks
}

fn solve(input: &str, blinks: usize, parse: fn(&str, usize) -> u64) -> u64 {
    parse(input, blinks)
}

fn parse1(input: &str, blinks: usize) -> u64 {
    let rocks: Vec<usize> = input
        .lines()
        .next().unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut rocks_count: HashMap<usize, u64> = HashMap::new();
    for rock in &rocks {
        *rocks_count.entry(*rock).or_insert(0) += 1;
    }

    for _ in 0 .. blinks {
        // println!("{}", rocks.len());
        rocks_count = blink_better(&rocks_count);
    }

    rocks_count
        .iter()
        .map(|(_, v)| v)
        .sum()
}

fn parse2(input: &str, blinks: usize) -> u64 {
    parse1(input, blinks)
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
        assert_eq!(solve(INPUT, 25, parse1), 197357
);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(INPUT, 75, parse2), 234568186890978);
    }
}
