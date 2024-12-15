#![allow(dead_code)]

use core::fmt;
use std::collections::HashSet;

struct Map {
    walls: HashSet<(usize, usize)>,
    boxes: HashSet<(usize, usize)>,
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
                } else if self.boxes.contains(&(x, y)) {
                    s.push('O');
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
    fn from(lines: Vec<&str>) -> Map {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut bot = (0, 0);
        let mut width = 0;

        for (y, line) in lines.iter().enumerate() {
            width = line.len();

            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => _ = walls.insert((x, y)),
                    'O' => _ = boxes.insert((x, y)),
                    '@' => bot = (x, y),
                    _ => {},
                };
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

    fn find_next_empty(&self, direction: char) -> Option<(Option<(usize, usize)>, (usize, usize))> {

        let movement = self.get_movement(direction);
        let mut boxes_found = false;

        let mut cx = self.bot.0 as isize;
        let mut cy = self.bot.1 as isize;

        let adj = (
            (self.bot.0 as isize + movement.0) as usize,
            (self.bot.1 as isize + movement.1) as usize,
        );

        loop {
            let next = ((cx as isize + movement.0) as usize, (cy as isize + movement.1) as usize);

            if (cx + movement.0) < 0 || next.0 as usize > self.width || (cy + movement.1) < 0 || next.1 as usize > self.height {
                return None;
            } else if self.walls.contains(&next) {
                return None;
            } else if self.boxes.contains(&next) {
                boxes_found = true;
                cx = next.0 as isize;
                cy = next.1 as isize;
            } else {
                return Some((if boxes_found { Some(adj) } else { None }, next));
            }
        }
    }

    fn move_(&mut self, direction: char) {
        if let Some((next_empty, last_empty)) = self.find_next_empty(direction) {
            match next_empty {
                Some(adj) => {
                    self.bot = adj;
                    self.boxes.remove(&adj);
                    self.boxes.insert(last_empty);
                },
                None => { self.bot = last_empty },
            }
        }
    }
}

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse_input(input: &str) -> (Map, String) {
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
        Map::from(map_lines),
        directions,
    )
}

fn parse1(input: &str) -> u32 {
    let (mut map, directions) = parse_input(input);

    for c in directions.chars() {
        map.move_(c);
    }

    map.boxes
        .iter()
        .map(|(x, y)| x + 100*y)
        .sum::<usize>() as u32
}

fn parse2(input: &str) -> u32 {
    0
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

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 2028);
        assert_eq!(solve(TEST2, parse1), 10092);
        assert_eq!(solve(INPUT, parse1), 1526673);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(TEST2, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
