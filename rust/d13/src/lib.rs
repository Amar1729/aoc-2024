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
    prize_x: u32,
    prize_y: u32,
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
            prize_x: prize.0,
            prize_y: prize.1,
        }
    }

    // start from fewest to most button presses of A (the more expensive one)
    // see if we can find any integral solution by combining A and B presses
    // if not, fail
    fn solve(&self) -> Result<(u32, u32), bool> {
        // i assume p2 will increase our search space.
        // wonder if rust will be BLAZINGLY FAST enough.
        for mult_a in 0..101 {
            let x = mult_a * self.a_x;
            let y = mult_a * self.a_y;

            if self.prize_x > x && self.prize_y > y {
                if (self.prize_x - x) % self.b_x == 0 && (self.prize_y - y) % self.b_y == 0 {
                    let mult_b = (self.prize_x - x) / self.b_x;
                    if mult_b < 101 && mult_b == (self.prize_y - y) / self.b_y {
                        println!("{x} {y} {mult_a} {mult_b}");
                        return Ok((mult_a, mult_b));
                    }
                }
            }

        }

        Err(false)
    }

    fn cost(&self) -> u32 {
        if let Ok((a, b)) = self.solve() {
            3*a + b
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

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    parse(input)
}

fn parse1(input: &str) -> u32 {
    let claws = parse_buttons(input);
    
    claws.iter().map(|c| {c.cost()}).sum()
}

fn parse2(input: &str) -> u32 {
    0
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
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
