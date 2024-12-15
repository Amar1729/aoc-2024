#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Vector {
    p: (isize, isize),
    v: (isize, isize),
}

impl Vector {
    /// e.g.
    /// p=0,4 v=3,-3
    fn from(line: &str) -> Vector {
        let parts: Vec<&str> = line.split(' ').collect();

        let p: Vec<isize> = parts[0][2..].split(',').map(|c| c.parse::<isize>().unwrap()).collect();
        let v: Vec<isize> = parts[1][2..].split(',').map(|c| c.parse::<isize>().unwrap()).collect();

        Vector {
            p: (p[0], p[1]),
            v: (v[0], v[1]),
        }
    }
}

fn time_step(robots: &mut Vec<Vector>, bounds: (isize, isize)) {
    for robot in robots {
        robot.p.0 = (robot.p.0 + robot.v.0 + bounds.0) % bounds.0;
        robot.p.1 = (robot.p.1 + robot.v.1 + bounds.1) % bounds.1;
    }
}

fn quad_count(robots: &Vec<Vector>, bounds: (isize, isize)) -> (usize, usize, usize, usize) {
    let mid_x = bounds.0 / 2;
    let mid_y = bounds.1 / 2;

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    for robot in robots {
        if robot.p.0 < mid_x && robot.p.1 < mid_y {
            tl += 1;
        } else if robot.p.0 > mid_x && robot.p.1 < mid_y {
            tr += 1;
        } else if robot.p.0 < mid_x && robot.p.1 > mid_y {
            bl += 1;
        } else if robot.p.0 > mid_x && robot.p.1 > mid_y {
            br += 1;
        }
    }

    (tl, tr, bl, br)
}

fn construct_grid(robots: &Vec<Vector>, bounds: (isize, isize)) -> String {
    let mut mapping = HashSet::new();

    for robot in robots {
        mapping.insert(robot.p);
    }

    let mut s = String::new();

    for y in 0 .. (bounds.1 as usize) {
        for x in 0 .. (bounds.0 as usize) {
            if mapping.contains(&(x as isize, y as isize)) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }

    s
}

fn solve(input: &str, bounds: (isize, isize), parse: fn(&str, (isize, isize)) -> u32) -> u32 {
    parse(input, bounds)
}

fn parse1(input: &str, bounds: (isize, isize)) -> u32 {
    let mut robots: Vec<Vector> = input.lines().map(|line| Vector::from(line)).collect();

    for _ in 0 .. 100 {
        time_step(&mut robots, bounds);
    }


    let c = quad_count(&robots, bounds);

    (c.0 * c.1 * c.2 * c.3) as u32
}

fn parse2(input: &str, bounds: (isize, isize)) -> u32 {
    // lol. i think i know what p2 will look like!
    // update - i did not! it's a graphics challenge!!!

    let mut robots: Vec<Vector> = input.lines().map(|line| Vector::from(line)).collect();

    // total assumption that the problem is looking for something vaguely pretty.
    // just watch what happens and ctrl-c 
    for i in 0 .. {

        let s = construct_grid(&robots, bounds);

        if s.contains("###########") {
            println!("==== {i} ====");
            print!("{}", s);
            println!("==== {i} ====");

            return i;
        }

        time_step(&mut robots, bounds);
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 11 wide, 7 tall
    const TEST1_BOUNDS: (isize, isize) = (11, 7);
    const TEST1: &'static str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    // 101 wide, 103 tall
    const INPUT_BOUNDS: (isize, isize) = (101, 103);
    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, TEST1_BOUNDS, parse1), 12);
        assert_eq!(solve(INPUT, INPUT_BOUNDS, parse1), 225521010);
    }

    #[test]
    fn test2() {
        // test input won't make funny pic
        // assert_eq!(solve(TEST1, TEST1_BOUNDS, parse2), 0);
        assert_eq!(solve(INPUT, INPUT_BOUNDS, parse2), 7774);
    }
}
