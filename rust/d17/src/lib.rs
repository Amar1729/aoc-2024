#![allow(dead_code)]

fn solve(input: &str, parse: fn(&str) -> String) -> String {
    parse(input)
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<usize>) {
    let mut lines = input.lines();

    let registers = lines.clone()
        .take(3)
        .map(|reg| {
            reg.split(": ").nth(1).unwrap().parse::<u64>().unwrap()
        })
        .collect();

    // _ = lines.next();

    let program = lines
        // stupid, it's cause i'm cloning earlier
        .nth(4)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<usize>>();

    (registers, program)
}

fn run_program(registers: &mut [u64], program: &[usize]) -> String {

    let mut outputs = Vec::new();

    // fn combo(operand: usize) -> usize {
    // i don't actually want registers to be borrowed mutably here!
    let combo = |operand, registers: &&mut [u64]| {
        (match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            7 => panic!(),
            _ => panic!(),
        } as u64)
    };

    let mut curr = 0;
    while curr < program.len() {

        let opcode = program[curr];
        let operand = program[curr + 1];

        match opcode {

            // adv (division)
            0 => {
                let base: u64 = 2;
                let exp: u32 = combo(operand, &registers) as u32;
                registers[0] = registers[0] / (base.pow(exp));
            },

            // bxl (bitwise xor)
            1 => registers[1] ^= operand as u64,

            // bst (mod 8)
            2 => registers[1] = (combo(operand, &registers) % 8) as u64,

            // jnz (jump)
            3 => {
                if registers[0] != 0 {
                    curr = operand;
                    continue;
                }
            },

            // bxc (bitwise xor)
            4 => registers[1] ^= registers[2],

            // out
            5 => outputs.push(combo(operand, &registers) % 8),

            // bdv (like adv, division)
            6 => {
                let base: u64 = 2;
                let exp: u32 = combo(operand, &registers) as u32;
                registers[1] = registers[0] / (base.pow(exp));
            },

            // cdv (like adv and bdv, division)
            7 => {
                let base: u64 = 2;
                let exp: u32 = combo(operand, &registers) as u32;
                registers[2] = registers[0] / (base.pow(exp));
            },

            _ => panic!(),
        };

        curr += 2;
    }

    outputs
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn parse1(input: &str) -> String {
    let (mut registers, program) = parse_input(input);

    // println!("{:?}", registers);
    // println!("{:?}", program);

    let result = run_program(&mut registers, &program);
    // println!("{:?}", result);

    result
}

fn naive_register_search(registers: &[u64], program: &[usize]) -> String {
    // brute force search?
    // assume we start from original value of a
    let start = registers[0];
    for a in start .. {
        let mut reg_copy = vec![0; 3];
        reg_copy[..3].clone_from_slice(registers);
        reg_copy[0] = a;

        let out = run_program(&mut reg_copy, &program)
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if out == program {
            return a.to_string();
        }
    }

    panic!()
}

/// custom-built solution based on reading the input
fn dfs(registers: &[u64], program: &[usize], curr: u64) -> Option<u64> {
    for i in 0 .. 8 {
        let mut nr = vec![0; 3];
        nr[..3].clone_from_slice(registers);
        let candidate = (curr << 3) | i;
        nr[0] = candidate;

        let out = run_program(&mut nr, &program)
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if out.iter().rev().zip(program.iter().rev())
            .any(|(a, b)| a != b) {
                if out.len() > program.len() { return None; }
                continue;
            }

        if out.len() == program.len() {
            println!("Found soln {candidate}");
            return Some(candidate);
        }

        if let Some(soln) = dfs(registers, program, candidate) {
            println!("Found acceptable next byte, recursing: {candidate} {out:?}");
            return Some(soln)
        }
    }

    None
}

fn parse2(input: &str) -> String {
    let (registers, program) = parse_input(input);
    dfs(&registers, &program, 0)
        .expect("Could not find a solution.")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const TEST2: &'static str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test_run_program() {
        let mut registers = [0, 0, 9];
        run_program(&mut registers, &[2, 6]);
        assert_eq!(registers[1], 1);

        let mut registers = [10, 0, 0];
        let out = run_program(&mut registers, &[5,0,5,1,5,4]);
        assert_eq!(out, "0,1,2");

        let mut registers = [2024, 0, 0];
        let out = run_program(&mut registers, &[0,1,5,4,3,0]);
        assert_eq!(out, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(registers[0], 0);

        let mut registers = [0, 29, 0];
        run_program(&mut registers, &[1,7]);
        assert_eq!(registers[1], 26);

        let mut registers = [0, 2024, 43690];
        run_program(&mut registers, &[4,0]);
        assert_eq!(registers[1], 44354);
    }

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(solve(INPUT, parse1), "1,3,5,1,7,2,5,1,6");
    }

    #[test]
    fn test2() {
        // assert_eq!(solve(TEST2, parse2), "117440");
        assert_eq!(solve(INPUT, parse2), "236555997372013");
    }
}
