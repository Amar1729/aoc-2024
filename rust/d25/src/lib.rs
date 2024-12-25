#![allow(dead_code)]

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn to_locks_and_keys(blocks: Vec<&str>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for block in blocks {
        let lines: Vec<&str> = block.lines().collect();
        assert_eq!(lines.len(), 7);
        let width = lines[0].len();

        let mut heights = vec![];

        for x in 0 .. width {
            heights.push(
                lines
                    .iter()
                    .filter(|row| row.chars().nth(x).unwrap() == '#')
                    .count()
                    - 1
            );
        }

        match block.chars().nth(0).unwrap() {
            // lock
            '#' => locks.push(heights),
            // key
            '.' => keys.push(heights),
            _ => panic!(),
        }
    }

    (locks, keys)
}

fn parse1(input: &str) -> u32 {
    let (locks, keys) = to_locks_and_keys(input.split("\n\n").collect());

    locks
        .iter()
        .map(|lock| {
            keys
                .iter()
                .filter(|key| {
                    // check_overlap(lock, key)
                    lock.iter().zip(key.iter())
                        .all(|(l, k)| l + k <= 5)
                })
                .count()
        })
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 3);
        assert_eq!(solve(INPUT, parse1), 3451);
    }
}
