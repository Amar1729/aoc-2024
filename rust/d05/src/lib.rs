#![allow(dead_code)]

use std::collections::HashMap;

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn rules_and_updates<'a>(
    input: &'a str,
) -> (HashMap<usize, Vec<usize>>, impl Iterator<Item = Vec<usize>> + 'a) {
    let mut rules = HashMap::new();

    let mut it = input.lines();

    loop {
        if let Some(line) = it.next() {
            if line.trim().is_empty() { break; }

            let new_rule = line.split('|').map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let rule = rules.entry(new_rule[0]).or_insert(Vec::new());
            rule.push(new_rule[1]);
        } else {
            // not possible in first part of input (there must be more).
            unreachable!();
        }
    }

    // remaining lines are usize vecs, split by commas
    let updates_it = it.map(|line| {
        line.split(',').map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>()
    });

    (rules, updates_it)
}

fn parse1(input: &str) -> u32 {
    let (rules, updates) = rules_and_updates(input);

    updates
        .filter_map(|update| {
            let mut medianth = 0;

            for (idx, page) in update.iter().enumerate() {
                if idx % 2 == 0 { medianth += 1; }

                for already_page in update[0 .. idx].iter() {
                    // invalid!
                    if let Some(deps) = rules.get(page) {
                        if deps.contains(already_page) {
                            return None;
                        }
                    }
                }
            }

            Some(update[medianth-1] as u32)
        })
        .sum()
}

fn parse2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    // const TEST2: &'static str = r#""#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 143);
        assert_eq!(solve(INPUT, parse1), 5747);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
