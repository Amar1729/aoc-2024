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

fn is_correct_update(update: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    for (idx, page) in update.iter().enumerate() {
        for already_page in update[0 .. idx].iter() {
            if let Some(deps) = rules.get(page) {
                if deps.contains(already_page) {
                    return false;
                }
            }
        }
    }

    true
}

fn parse1(input: &str) -> u32 {
    let (rules, updates) = rules_and_updates(input);

    updates
        .filter_map(|update| {
            if is_correct_update(&update, &rules) {
                Some(update[(update.len() - 1) / 2] as u32)
            } else {
                None
            }
        })
        .sum()
}

fn parse2(input: &str) -> u32 {
    let (rules, updates) = rules_and_updates(input);

    updates
        .filter_map(|update| {
            if is_correct_update(&update, &rules) {
                None
            } else {
                let mut new_update = Vec::new();
                let mut queue = update.clone();

                while !queue.is_empty() {
                    match queue
                        .clone()
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, page)| {
                            match rules.get(page) {
                                Some(deps) => {
                                    for d in deps {
                                        if queue.contains(d) {
                                            return None;
                                        }
                                    }
                                    Some((idx, page))
                                },
                                None => Some((idx, page)),
                            }
                        })
                        .next() {
                        Some((idx, page)) => {
                            new_update.push(*page);
                            queue.remove(idx);
                        },
                        None => {},
                    }
                }

                Some(new_update[(new_update.len() - 1) / 2])
            }
        })
        .sum::<usize>() as u32
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
        assert_eq!(solve(TEST1, parse2), 123);
        assert_eq!(solve(INPUT, parse2), 5502);
    }
}
