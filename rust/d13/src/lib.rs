#![allow(dead_code)]

#[derive(Debug)]
struct Claw {
    // 3 tokens
    a_x: u32,
    a_y: u32,

    // 1 tokens
    b_x: u32,
    b_y: u32,

    // where's the prize
    prize_x: u64,
    prize_y: u64,
}

fn line_parse(line: &str) -> (u32, u32) {
    let parts: Vec<&str> = line.split(": ").collect();
    let xy: Vec<&str> = parts[1].split(", ").collect();

    if xy[0].contains(&"=") {
        let x = xy[0].split('=').into_iter().nth(1).unwrap().parse::<u32>();
        let y = xy[1].split('=').into_iter().nth(1).unwrap().parse::<u32>();

        (x.unwrap(), y.unwrap())
    } else if xy[0].contains(&"+") {
        let x = xy[0].split('+').into_iter().nth(1).unwrap().parse::<u32>();
        let y = xy[1].split('+').into_iter().nth(1).unwrap().parse::<u32>();

        (x.unwrap(), y.unwrap())
    } else {
        panic!();
    }
}

impl Claw {
    fn from(line1: &str, line2: &str, line3: &str) -> Self {
        let b_a = line_parse(line1);
        let b_b = line_parse(line2);
        let prize = line_parse(line3);

        Claw {
            a_x: b_a.0,
            a_y: b_a.1,
            b_x: b_b.0,
            b_y: b_b.1,
            prize_x: prize.0 as u64,
            prize_y: prize.1 as u64,
        }
    }

    fn cramer(&self) -> Option<(i64, i64)> {
        // (am i dumb? this is so obvious lol)
        // det: ad - bc

        // kind of gross assignment and logic in this function, sorry

        let a = self.a_x as u64;
        let b = self.b_x as u64;
        let c = self.a_y as u64;
        let d = self.b_y as u64;

        let det_a = (a * d) as i64 - (b * c) as i64;

        let det_x = (self.prize_x * d) as i64 - (self.prize_y * b) as i64;
        let det_y = (a * self.prize_y) as i64 - (c * self.prize_x) as i64;

        let num_a = det_x / det_a;
        let num_b = det_y / det_a;

        if num_a < 0 || num_b < 0 { return None };

        if !(num_a as u64 * a as u64 + num_b as u64 * b as u64 == self.prize_x as u64) {
            if !(num_a as u64 * c as u64 + num_b as u64 * d as u64 == self.prize_y as u64) {
                return None
            }
        }

        Some((num_a, num_b))
    }

    fn cost(&self) -> u64 {
        if let Some((a, b)) = self.cramer() {
            (3*a + b).abs().try_into().unwrap()
        } else {
            0
        }
    }
}

fn parse_buttons(input: &str) -> Vec<Claw> {
    let mut lines = input.lines();
    let mut claws = Vec::new();

    loop {
        if let Some(line1) = lines.next() {
            if let Some(line2) = lines.next() {
                if let Some(line3) = lines.next() {
                    // each line should be valid
                    claws.push(Claw::from(line1, line2, line3));

                    // should be empty, until end of input
                    if lines.next() == None {
                        break
                    }
                }
            }
        }
    }

    claws
}

fn solve(input: &str, parse: fn(&str) -> u64) -> u64 {
    parse(input)
}

fn parse1(input: &str) -> u64 {
    parse_buttons(input)
        .iter()
        .map(|c| c.cost())
        .sum()
}

fn parse2(input: &str) -> u64 {
    let diff: u64 = 10_000_000_000_000;
    parse_buttons(input)
        .iter_mut()
        .map(|c| {
            c.prize_x += diff;
            c.prize_y += diff;
            c.cost()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 480);
        assert_eq!(solve(INPUT, parse1), 35082);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(INPUT, parse2), 82570698600470);
    }
}
