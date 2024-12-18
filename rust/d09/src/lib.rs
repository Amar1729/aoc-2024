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

fn block_checksum(blocks: &[Option<usize>]) -> u64 {
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

fn print_blocks(blocks: &[Option<usize>]) {
    for b in blocks {
        match b {
            Some(b) => print!("{b}"),
            None => print!("."),
        };
    }
    println!();
}

fn parse1(input: &str) -> u64 {
    let mut blocks = make_block_map(input);

    let mut small = 0;
    let mut big = blocks.len();

    while small < big {
        // inefficient, but whatever
        small = blocks.iter().position(|e| e.is_none()).unwrap();
        big = blocks.len() - blocks.iter().rev().position(|e| e.is_some()).unwrap() - 1;

        if small >= big { break }

        blocks[small] = blocks[big];
        blocks[big] = None;
    }

    block_checksum(&blocks)
}

fn parse2(input: &str) -> u64 {
    let mut blocks = make_block_map(input);

    let mut big = blocks.len() - blocks.iter().rev().position(|e| e.is_some()).unwrap() - 1;

    // avoiding off-by-one errors during this was a real pain
    while big > 0 {
        let mut big_range = 1;
        while big > big_range && blocks[big - big_range + 1] == blocks[big] {
            big_range += 1;
        }
        if big <= big_range {
            break;
        }
        big_range -= 1;

        let ok_space = (1 .. big - big_range)
            .find(|small| {
                (0 .. big_range)
                    .all(|idx| blocks[small + idx].is_none())
            });

        if let Some(small) = ok_space {
            let label = blocks[big].unwrap();
            for idx in 0 .. big_range {
                blocks[small + idx] = Some(label);
                blocks[big - idx] = None;
            }
        }

        big -= big_range;
        while blocks[big].is_none() && big > 0 {
            big -= 1;
        }
    }

    // print_blocks(&blocks);
    block_checksum(&blocks)
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
        assert_eq!(solve(TEST1, parse2), 2858);
        assert_eq!(solve(INPUT, parse2), 6398096697992);
    }
}
