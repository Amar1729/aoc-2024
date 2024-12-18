#![allow(dead_code)]

use pathfinding::prelude::astar;
use utils::Point;
use std::collections::HashSet;

fn into_grid(input: Vec<&str>) -> HashSet<Point> {
    input
        .iter()
        .map(|line| {
            let parts: Vec<usize> = line
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect();
            Point::from((parts[0] as isize, parts[1] as isize))
        })
        .collect()
}

fn print_grid(bytes: HashSet<Point>, path: &[(isize, isize)], end: (isize, isize)) {
    for y in 0 .. (end.1 + 1) as usize {
        for x in 0 .. end.0 + 1 {
            if bytes.contains(&Point::from((x as isize, y  as isize))) { print!("#") }
            else if path.contains(&(x as isize, y  as isize)) { print!("O") }
            else { print!(".") }
        }
        println!();
    }
}

fn traverse(end: (isize, isize), bytes: &HashSet<Point>) -> Option<u32> {
    let result = astar(
        &(0, 0),
        |&(x, y)| {
            [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ]
                .into_iter()
                .filter_map(|p| {
                    let nc = (x + p.0, y + p.1);
                    let np = Point::from(nc);
                    let in_bounds = np.contained((end.0 + 1) as usize, (end.1 + 1) as usize);
                    match in_bounds && !bytes.contains(&np) {
                        true => Some((nc, 1)),
                        false => None,
                    }
                })
                .collect::<Vec<((isize, isize), usize)>>()
        },
        |&(x, y)| ((end.0 as isize).abs_diff(x) + (end.1 as isize).abs_diff(y)) / 2,
        |&p| p == end,
    );

    if let Some(result) = result {
        Some((result.0.len() - 1) as u32)
    } else {
        None
    }

}

fn parse1(input: &str, bounds: (isize, isize), initial: usize) -> u32 {
    let bytes = into_grid(input.lines().take(initial).collect());
    traverse(bounds, &bytes).unwrap()
}

fn parse2(input: &str, bounds: (isize, isize), initial: usize) -> String {
    let mut bytes_v = vec![];
    let mut iter = input.lines();

    // start from here because we know it's fine
    for _ in 0 .. initial {
        bytes_v.push(iter.next().unwrap());
    }

    let mut bytes = into_grid(bytes_v);

    for new_byte in iter {
        let parts: Vec<usize> = new_byte
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        let p = Point::from((parts[0] as isize, parts[1] as isize));
        bytes.insert(p);

        match traverse(bounds, &bytes) {
            None => {
                return new_byte.to_string();
            },
            _ => {},
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(parse1(TEST1, (6, 6), 12), 22);
        assert_eq!(parse1(INPUT, (70, 70), 1024), 292);
    }

    #[test]
    fn test2() {
        assert_eq!(parse2(TEST1, (6, 6), 12), "6,1");
        assert_eq!(parse2(INPUT, (70, 70), 1024), "58,44");
    }
}
