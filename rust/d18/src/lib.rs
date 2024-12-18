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

    let bytes = into_grid(bytes_v);

    let falling_bytes: Vec<Point> = iter
        .map(|b| {
            let parts: Vec<usize> = b.split(',')
                .map(|c| c.parse().unwrap())
                .collect();
            Point::from((parts[0] as isize, parts[1] as isize))
        })
        .collect();

    let mut small = initial;
    let mut big = initial + falling_bytes.len();
    let mut idx = small + (big - small) / 2;

    // nice and simple binary search over the rest of the bytes
    while idx != small {
        println!("{small} {idx} {big}");
        let mut fallen = bytes.clone();
        for byte in &falling_bytes[..idx-initial] {
            fallen.insert(*byte);
        }

        match traverse(bounds, &fallen) {
            // failed to find a path: lower the upper bound
            None => big = idx,
            // still found a path: increase the lower bound
            _ => small = idx,
        };

        idx = small + (big - small) / 2;
    }

    let byte = falling_bytes[idx - initial];
    format!("{},{}", byte.x, byte.y)
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
