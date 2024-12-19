#![allow(dead_code)]

use core::fmt;
use std::collections::{HashMap, HashSet};

struct Map {
    walls: HashSet<(usize, usize)>,
    boxes: HashMap<(usize, usize), char>,
    bot: (usize, usize),

    height: usize,
    width: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for y in 0 .. self.height {
            for x in 0 .. self.width {
                if self.walls.contains(&(x, y)) {
                    s.push('#');
                } else if let Some(&c) = self.boxes.get(&(x, y)) {
                    s.push(c);
                } else if self.bot == (x, y) {
                    s.push('@');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }

        write!(f, "{s}")
    }
}

impl Map {
    fn from(lines: Vec<&str>, part2: bool) -> Map {
        let mut walls = HashSet::new();
        let mut boxes = HashMap::new();
        let mut bot = (0, 0);
        let mut width = 0;

        for (y, line) in lines.iter().enumerate() {
            width = line.len() + (part2 as usize * line.len());

            for (x, c) in line.chars().enumerate() {
                if !part2 {
                    match c {
                        '#' => _ = walls.insert((x, y)),
                        'O' => _ = boxes.insert((x, y), 'O'),
                        '@' => bot = (x, y),
                        _ => {},
                    };
                } else {
                    match c {
                        '#' => {
                            walls.insert((2*x, y));
                            walls.insert((2*x+1, y));
                        },
                        'O' => {
                            boxes.insert((2*x, y), '[');
                            boxes.insert((2*x+1, y), ']');
                        },
                        '@' => bot = (2*x, y),
                        _ => {},
                    };
                }
            }
        }

        Map {
            walls,
            boxes,
            bot,
            height: lines.len(),
            width,
        }
    }

    fn get_movement(&self, direction: char) -> (isize, isize) {
        match direction {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        }
    }

    fn try_move(&mut self, direction: char, coord: (isize, isize)) -> bool {
        let uc = (coord.0 as usize, coord.1 as usize);
        let m = self.get_movement(direction);
        if self.walls.contains(&uc) {
            false
        } else if let Some(c) = self.boxes.get(&uc).cloned() {

            match (c, direction == '<' || direction == '>') {
                ('O', _) => {
                    let nc = (coord.0 + m.0, coord.1 + m.1);
                    let b = self.try_move(direction, nc);
                    if b {
                        self.boxes.remove(&uc);
                        self.boxes.insert((nc.0 as usize, nc.1 as usize), 'O');
                        true
                    } else {
                        false
                    }
                },

                // left/right
                (_, true) => {
                    let nc = (coord.0 + m.0, coord.1 + m.1);
                    let b = self.try_move(direction, nc);
                    if b {
                        self.boxes.remove(&uc);
                        self.boxes.insert((nc.0 as usize, nc.1 as usize), c);
                        true
                    } else {
                        false
                    }
                },

                // up/down
                (_, false) => {
                    let other_half = match c {
                        '[' => (coord.0 + 1, coord.1),
                        ']' => (coord.0 - 1, coord.1),
                        _ => panic!(),
                    };
                    let other_c = match c {
                        '[' => ']',
                        ']' => '[',
                        _ => panic!(),
                    };
                    let nc = (coord.0 + m.0, coord.1 + m.1);
                    let nc_other = (other_half.0 + m.0, other_half.1 + m.1);
                    let b = self.try_move(direction, nc) &&
                        self.try_move(direction, nc_other);

                    if b {
                        self.boxes.remove(&uc);
                        self.boxes.remove(&(other_half.0 as usize, other_half.1 as usize));
                        self.boxes.insert((nc.0 as usize, nc.1 as usize), c);
                        self.boxes.insert((nc_other.0 as usize, nc_other.1 as usize), other_c);
                        true
                    } else {
                        false
                    }
                },
            }
        } else {
            true
        }
    }

    fn move_(&mut self, direction: char) {
        let c = self.get_movement(direction);
        let orig = self.boxes.clone();

        let pt = (self.bot.0 as isize + c.0, self.bot.1 as isize + c.1);
        if self.try_move(direction, pt) {
            self.bot = (pt.0 as usize, pt.1 as usize);
        } else {
            // sometimes when checking up or down movements, some of the boxes may successfully
            // move while others do not. if that is the case, reset the box state back to what it
            // was originally.
            self.boxes = orig;
        }
    }
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse_input(input: &str, part2: bool) -> (Map, String) {
    let mut lines = input.lines();
    let mut line = lines.next().unwrap();

    let mut map_lines = vec![];
    while !line.trim().is_empty() {
        map_lines.push(line);
        line = lines.next().unwrap();
    }

    let mut directions = String::new();
    line = lines.next().unwrap();
    while !line.trim().is_empty() {
        for c in line.chars() {
            directions.push(c);
        }

        match lines.next() {
            Some(line_) => { line = line_ },
            None => break,
        };
    }

    (
        Map::from(map_lines, part2),
        directions,
    )
}

fn parse1(input: &str) -> u32 {
    let (mut map, directions) = parse_input(input, false);

    for c in directions.chars() {
        map.move_(c);
    }

    map.boxes
        .iter()
        .map(|((x, y), _)| x + 100*y)
        .sum::<usize>() as u32
}

fn parse2(input: &str) -> u32 {
    let (mut map, directions) = parse_input(input, true);

    // println!("initial: {map}\n");

    for c in directions.chars() {
        map.move_(c);
        // println!("\n{map}\n");
    }

    map.boxes
        .iter()
        .filter(|(_, &c)| c == '[')
        .map(|((x, y), _)| x + 100*y)
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const TEST2: &'static str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    const TEST3: &'static str = r#"########
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 2028);
        assert_eq!(solve(TEST2, parse1), 10092);
        assert_eq!(solve(INPUT, parse1), 1526673);
    }

    #[test]
    fn test2() {
        // TEST3 only for testing map steps:
        // assert_eq!(solve(TEST3, parse2), 0);
        assert_eq!(solve(TEST2, parse2), 9021);
        assert_eq!(solve(INPUT, parse2), 1535509);
    }
}
