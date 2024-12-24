use std::collections::VecDeque;

use rustc_hash::FxHashMap;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Gate {
    AND,
    OR,
    XOR,
}

pub fn part1(contents: String) -> String {
    let (mut input, gates) = parse(&contents);

    let output = simulate_gates(&mut input, gates);

    calculate_output(output, "z").to_string()
}

pub fn part2(contents: String) -> String {
    let (mut input, gates) = parse(&contents);

    let output = simulate_gates(&mut input, gates);
    let x = calculate_output(output, "x");
    let y = calculate_output(output, "y");
    let z = calculate_output(output, "z");

    dbg!(x, y, z);

    todo!()
}

fn parse(contents: &str) -> (FxHashMap<&str, u8>, VecDeque<(&str, Gate, &str, &str)>) {
    let mut parts = contents.split("\n\n");
    let input: FxHashMap<&str, u8> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut input_parts = l.split(" ");
            (
                input_parts.next().unwrap().strip_suffix(":").unwrap(),
                input_parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let gates: VecDeque<(&str, Gate, &str, &str)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut gate_parts = l.split(" ");
            let input1 = gate_parts.next().unwrap();
            let gate_type = gate_parts.next().unwrap();
            let gate_type = match gate_type {
                "OR" => Gate::OR,
                "AND" => Gate::AND,
                "XOR" => Gate::XOR,
                _ => unreachable!(),
            };
            let input2 = gate_parts.next().unwrap();
            gate_parts.next();
            let output = gate_parts.next().unwrap();
            (input1, gate_type, input2, output)
        })
        .collect();
    (input, gates)
}

fn simulate_gates<'a>(
    input: &'a mut FxHashMap<&'a str, u8>,
    mut gates: VecDeque<(&'a str, Gate, &'a str, &'a str)>,
) -> &'a mut FxHashMap<&'a str, u8> {
    while let Some(gate) = gates.pop_back() {
        let (input1, gate_type, input2, output) = &gate;
        if input.contains_key(input1) && input.contains_key(input2) {
            let value1 = input.get(*input1).unwrap();
            let value2 = input.get(*input2).unwrap();
            let output_value = match gate_type {
                Gate::AND => value1 & value2,
                Gate::OR => value1 | value2,
                Gate::XOR => value1 ^ value2,
            };
            input.insert(output, output_value);
        } else {
            gates.push_front(gate);
        }
    }

    input
}

fn calculate_output(input: &FxHashMap<&str, u8>, starts_with: &str) -> u64 {
    let mut output: Vec<_> = input
        .iter()
        .filter(|(k, _v)| k.starts_with(starts_with))
        .collect();
    output.sort_by_key(|v| v.0);

    let s: u64 = output
        .into_iter()
        .enumerate()
        .map(|(i, v)| (*v.1 as u64) << i)
        .sum();

    s
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/24/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/24/real.txt").unwrap()),
            "example2"
        );
    }
}
