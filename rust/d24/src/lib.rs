#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn parse_rules_and_values<'a>(input: &'a str) -> (HashSet<(Vec<&'a str>, &'a str)>, HashMap<&'a str, Option<bool>>) {
    let mut values: HashMap<&str, Option<bool>> = HashMap::new();
    let mut rules = HashSet::new();

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
    }

    (rules, values)
}

fn collect_gates(prefix: &str, values: &HashMap<&str, Option<bool>>) -> u64 {
    gates_to_num(
        &values
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            // kill me, this doesn't do lexical sorting???
            // .sorted_by(|a, b| Ord::cmp(a.1, b.1))
            .sorted_by(|(k1, _), (k2, _)| {
                let label1 = k1[1..].parse::<usize>().unwrap();
                let label2 = k2[1..].parse::<usize>().unwrap();
                Ord::cmp(&label1, &label2)
            })
            .map(|(_, v)| *v)
            .collect::<Vec<Option<bool>>>()
    )
}

fn gates_to_num(gates: &[Option<bool>] ) -> u64 {
    usize::from_str_radix(
        &gates
            .iter()
            .rev()
            .map(|b| (b.unwrap() as usize).to_string())
            .collect::<Vec<_>>()
            .join(""),
        2)
        .unwrap() as u64
}

fn simulate<'a>(rules: &HashSet<(Vec<&str>, &'a str)>, values: &mut HashMap<&'a str, Option<bool>>) -> u64 {
    let z_gates = rules
        .iter()
        .filter_map(|(_, rhs)| {
            if rhs.starts_with("z") {
                Some(rhs)
            } else { None }
        })
        .collect::<HashSet<_>>();

    // assumes none are set at beginning?
    let mut z_gates = vec![None; z_gates.len()];

    loop {
        for (lhs, rhs) in rules {

            if let Some(&Some(v1)) = values.get(lhs[0]) {
                if let Some(&Some(v2)) = values.get(lhs[2]) {
                    let result = match lhs[1] {
                        "AND" => Some(v1 & v2),
                        "OR" => Some(v1 | v2),
                        "XOR" => Some(v1 ^ v2),
                        _ => panic!(),
                    };
                    values.get_mut(rhs).map(|val| { *val = result; });

                    if rhs.starts_with("z") {
                        z_gates[rhs[1..].parse::<usize>().unwrap()] = result;
                    }
                }
            }
        }

        // for (gate, value) in &values {
        //     println!("{gate}: {value:?}");
        // }
        // println!();

        if z_gates.iter().all(|b| b.is_some()) { return gates_to_num(&z_gates); }
    }
}

fn parse1(input: &str) -> u64 {
    let (rules, mut values) = parse_rules_and_values(input);
    simulate(&rules, &mut values)
}

fn parse2(input: &str) -> String {
    let (mut rules, values) = parse_rules_and_values(input);

    // just print out the actual graph
    print_digraph(input);

    // visually determined by inspecting graphviz output
    let swaps = &[
        ("z07", "vmv"),
        ("z20", "kfm"),
        ("hnv", "z28"),
        ("hth", "tqr"),
    ];

    // i can't figure out how to allow this to be borrowed mutably and immutably?
    // i guess the problem is that it uses &str which have a lifetime too short? not sure?
    let orig_rules = rules.clone();
    for (out1, out2) in swaps {
        let swap1 = orig_rules.iter().find(|(_, r)| r == out1).unwrap();
        let swap2 = orig_rules.iter().find(|(_, r)| r == out2).unwrap();

        rules.remove(&swap1);
        rules.remove(&swap2);
        rules.insert((swap1.0.clone(), swap2.1));
        rules.insert((swap2.0.clone(), swap1.1));
    }

    let x = collect_gates("x", &values);
    let y = collect_gates("y", &values);

    let result = simulate(&rules, &mut values.clone());

    println!("   {x}\n+  {y}\n=  {result}");
    println!("   {x:b}\n+  {y:b}\n= {result:b}");

    assert_eq!(x+y, result);

    swaps
        .iter()
        .flat_map(|(out1, out2)| vec![out1, out2])
        // ??? apparently this works, while sorted_by (above) does not
        .sorted()
        .join(",")
}

fn print_digraph(input: &str) {
    let mut wires = HashMap::new();
    println!("digraph G {{");

    for i in 0 .. 46 {
        if i < 45 {
            let key = format!("x{i:02}");
            println!("  {} [pos=\"{},{}!\"]", key, i*2, 5);
            let value = key.clone();
            wires.insert(key, vec![value]);

            let key = format!("y{i:02}");
            println!("  {} [pos=\"{},{}!\"]", key, i*2+1, 5);
            let value = key.clone();
            wires.insert(key, vec![value]);
        }

        let key = format!("z{i:02}");
        println!("  {} [pos=\"{},{}!\"]", key, i*2, 0);
    }

    println!();
    let (_, suffix) = input.split_once("\n\n").unwrap();

    for (name, line) in suffix.lines().enumerate() {
        let tokens: Vec<_> = line.split(' ').collect();
        let [_, _, _, _, to] = tokens[..] else { unreachable!() };
        wires.entry(String::from(to)).or_insert_with(Vec::new).push(format!("{name}"));
    }

    let mut second = HashMap::new();

    for (name, line) in suffix.lines().enumerate() {
        let tokens: Vec<_> = line.split(' ').collect();
        let [left, op, right, _, to] = tokens[..] else { unreachable!() };

        let shape = match op {
            "AND" => "square",
            "OR" => "hexagon",
            "XOR" => "triangle",
            _ => unreachable!(),
        };

        if left.starts_with('x') || right.starts_with('x') {
            // let i: usize = left.unsigned();
            let i: usize = left[1..].parse().unwrap();
            if op == "AND" {
                println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 4);
                second.insert(to, i);
            }
            if op == "XOR" {
                println!("{} [pos=\"{},{}!\"]", name, i * 2, 4);
                second.insert(to, i);
            }
        }
        if to.starts_with('z') {
            // let i: usize = to.unsigned();
            let i: usize = to[1..].parse().unwrap();
            println!("{} [pos=\"{},{}!\"]", name, i * 2, 1);
        }

        println!("  {name} [shape={shape}]");
        for edge in &wires[&String::from(left)] {
            println!("  {edge} -> {name} [label=\"{left}\"]");
        }
        for edge in &wires[&String::from(right)] {
            println!("  {edge} -> {name} [label=\"{right}\"]");
        }
    }

    for (name, line) in suffix.lines().enumerate() {
        let tokens: Vec<_> = line.split(' ').collect();
        let [left, op, right, _, _] = tokens[..] else { unreachable!() };

        if op == "AND" {
            if let Some(i) = second.get(left) {
                println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 3);
            }
            if let Some(i) = second.get(right) {
                println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 3);
            }
        }
        if op == "OR" {
            if let Some(i) = second.get(left) {
                println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 2);
            }
            if let Some(i) = second.get(right) {
                println!("{} [pos=\"{},{}!\"]", name, i * 2 + 1, 2);
            }
        }
    }

    for i in 0..46 {
        let key = format!("z{i:02}");
        for edge in &wires[&key] {
            println!("  {edge} -> {key}");
        }
    }

    println!("}}");
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
        assert_eq!(parse1(TEST1), 4);
        assert_eq!(parse1(TEST2), 2024);
        assert_eq!(parse1(INPUT), 52956035802096);
    }

    #[test]
    fn test2() {
        assert_eq!(parse2(INPUT), "hnv,hth,kfm,tqr,vmv,z07,z20,z28");
    }
}
