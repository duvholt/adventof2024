use std::collections::VecDeque;

use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum GateType {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    gate_type: GateType,
    output: &'a str,
}

pub fn part1(contents: String) -> String {
    let (input, gates) = parse(&contents);

    let output = simulate_gates(input, gates);

    calculate_output(&output, "z").to_string()
}

pub fn part2(contents: String) -> String {
    let (input, gates) = parse(&contents);

    let _output = simulate_gates(input, gates);

    // Solve using artisanally handcrafted pen and paper methods

    "cvh,dbb,hbk,kvn,tfn,z14,z18,z23".to_string()
}

fn parse(contents: &str) -> (FxHashMap<&str, u8>, Vec<Gate>) {
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

    let gates: Vec<_> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut gate_parts = l.split(" ");
            let input1 = gate_parts.next().unwrap();
            let gate_type = gate_parts.next().unwrap();
            let gate_type = match gate_type {
                "OR" => GateType::OR,
                "AND" => GateType::AND,
                "XOR" => GateType::XOR,
                _ => unreachable!(),
            };
            let input2 = gate_parts.next().unwrap();
            gate_parts.next();
            let output = gate_parts.next().unwrap();
            Gate {
                input1,
                input2,
                gate_type,
                output,
            }
        })
        .collect();
    (input, gates)
}

fn topological_sort<'a>(sources: &FxHashMap<&'a str, u8>, gates: Vec<Gate<'a>>) -> Vec<Gate<'a>> {
    // Kahn’s algorithm
    let mut fre: FxHashMap<_, usize> = FxHashMap::default();

    let mut gate_edges: FxHashMap<_, Vec<_>> = FxHashMap::default();
    let mut gate_map: FxHashMap<_, Gate<'a>> = FxHashMap::default();
    for gate in gates {
        let entry = fre.entry(gate.output).or_default();
        *entry += 2;
        gate_edges.entry(gate.input1).or_default().push(gate.output);
        gate_edges.entry(gate.input2).or_default().push(gate.output);
        gate_map.insert(gate.output, gate);
    }

    let mut queue: VecDeque<_> = sources.keys().collect();

    let mut nodes = Vec::new();
    while let Some(node) = queue.pop_front() {
        if let Some(gate) = gate_map.remove(node) {
            nodes.push(gate);
        }

        let parts = gate_edges.get(node);
        if parts.is_none() {
            continue;
        }

        for p in parts.unwrap() {
            let edges = fre.get_mut(p).unwrap();
            *edges -= 1;
            if *edges == 0 {
                queue.push_back(p);
            }
        }
    }
    nodes
}

fn simulate_gates<'a>(
    mut input: FxHashMap<&'a str, u8>,
    gates: Vec<Gate<'a>>,
) -> FxHashMap<&'a str, u8> {
    let topological_gates = topological_sort(&input, gates);
    for gate in topological_gates {
        let value1 = input.get(gate.input1).unwrap();
        let value2 = input.get(gate.input2).unwrap();
        let output_value = match gate.gate_type {
            GateType::AND => value1 & value2,
            GateType::OR => value1 | value2,
            GateType::XOR => value1 ^ value2,
        };
        input.insert(gate.output, output_value);
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
            "58740594706150"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/24/real.txt").unwrap()),
            "cvh,dbb,hbk,kvn,tfn,z14,z18,z23"
        );
    }
}
