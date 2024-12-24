#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

fn solve(input: &str, parse: fn(&str) -> u64) -> u64 {
    parse(input)
}

fn parse1(input: &str) -> u64 {
    let mut values: HashMap<&str, Option<bool>> = HashMap::new();
    let mut rules = HashSet::new();
    let mut z_gates = HashSet::new();

    let mut iter = input.lines();
    for line in iter.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let (lhs, rhs) = line.split_once(": ").unwrap();
        values.insert(lhs, Some(if rhs.parse::<usize>().unwrap() == 1 { true } else { false }));
    }

    for line in iter {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();
        let lhs: Vec<&str> = lhs.split(' ').collect();
        rules.insert((lhs, rhs));

        if !values.contains_key(rhs) {
            values.insert(rhs, None);
        }

        if rhs.starts_with("z") && !z_gates.contains(rhs) {
            z_gates.insert(rhs);
        }
    }

    println!("{z_gates:?}");
    // assumes none are set at beginning?
    let mut z_gates = vec![None; z_gates.len()];

    loop {
        for (lhs, rhs) in &rules {
            match (lhs[0], lhs[1], lhs[2]) {
                (op1, "AND", op2) => {
                    if let Some(&Some(v1)) = values.get(op1) {
                        if let Some(&Some(v2)) = values.get(op2) {
                            let result = Some(v1 & v2);
                            values.get_mut(rhs).map(|val| { *val = result; });

                            if rhs.starts_with("z") {
                                z_gates[rhs[1..].parse::<usize>().unwrap()] = result;
                            }
                        }
                    }
                },

                (op1, "OR", op2) => {
                    if let Some(&Some(v1)) = values.get(op1) {
                        if let Some(&Some(v2)) = values.get(op2) {
                            let result = Some(v1 | v2);
                            values.get_mut(rhs).map(|val| { *val = result; });

                            if rhs.starts_with("z") {
                                z_gates[rhs[1..].parse::<usize>().unwrap()] = result;
                            }
                        }
                    }
                },

                (op1, "XOR", op2) => {
                    if let Some(&Some(v1)) = values.get(op1) {
                        if let Some(&Some(v2)) = values.get(op2) {
                            let result = Some(v1 ^ v2);
                            values.get_mut(rhs).map(|val| { *val = result; });

                            if rhs.starts_with("z") {
                                z_gates[rhs[1..].parse::<usize>().unwrap()] = result;
                            }
                        }
                    }
                },

                _ => panic!(),
            }
        }

        for (gate, value) in &values {
            println!("{gate}: {value:?}");
        }
        println!();

        if z_gates.iter().all(|b| b.is_some()) { break; }
    }

    isize::from_str_radix(
        &z_gates
            .iter()
            .rev()
            .map(|b| (b.unwrap() as usize).to_string())
            .collect::<Vec<_>>()
            .join(""),
        2)
        .unwrap() as u64
}

fn parse2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    const TEST2: &'static str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 4);
        assert_eq!(solve(TEST2, parse1), 2024);
        assert_eq!(solve(INPUT, parse1), 52956035802096);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST1, parse2), 0);
        assert_eq!(solve(INPUT, parse2), 0);
    }
}
