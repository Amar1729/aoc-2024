#![allow(dead_code)]

fn solve(input: &str, parse: fn(&str) -> String) -> String {
    parse(input)
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<usize>) {
    let mut lines = input.lines();

    let registers = lines.clone()
        .take(3)
        .map(|reg| {
            reg.split(": ").nth(1).unwrap().parse::<u32>().unwrap()
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

fn run_program(registers: &mut [u32], program: &[usize]) -> String {

    let mut outputs = Vec::new();

    // fn combo(operand: usize) -> usize {
    // i don't actually want registers to be borrowed mutably here!
    let combo = |operand, registers: &&mut [u32]| {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            7 => panic!(),
            _ => panic!(),
        }.try_into().unwrap()
    };

    println!("Starting:\n{:?}\n{:?}", registers, program);

    // for slice in program.chunks(2) {
    let mut curr = 0;
    while curr < program.len() {

        // assumed to work
        // let opcode = slice[0];
        // let operand = slice[1];

        let opcode = program[curr];
        let operand = program[curr + 1];

        println!("{opcode}, {operand}");

        match opcode {

            // adv (division)
            0 => {
                let base: u32 = 2;
                let exp: u32 = combo(operand, &registers);
                registers[0] = registers[0] / (base.pow(exp));
            },

            // bxl (bitwise xor)
            1 => registers[1] ^= operand as u32,

            // bst (mod 8)
            2 => registers[1] = combo(operand, &registers) % 8,

            // jnz (jump)
            3 => {
                if registers[0] != 0 {
                    curr = operand;
                    continue;
                }
            },

            // bxc (bitwise xor)
            4 => registers[1] = registers[1] ^ registers[2],

            // out
            5 => outputs.push(combo(operand, &registers) % 8),

            // bdv (like adv, division)
            6 => {
                let base: u32 = 2;
                let exp: u32 = combo(operand, &registers);
                registers[1] = registers[0] / (base.pow(exp));
            },

            // cdv (like adv and bdv, division)
            7 => {
                let base: u32 = 2;
                let exp: u32 = combo(operand, &registers);
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

    println!("{:?}", registers);
    println!("{:?}", program);

    let result = run_program(&mut registers, &program);
    println!("{:?}", result);

    result
}

fn parse2(input: &str) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    // const TEST2: &'static str = r#""#;

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
        assert_eq!(solve(INPUT, parse1), "");
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), "");
        assert_eq!(solve(INPUT, parse2), "");
    }
}
