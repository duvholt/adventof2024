use std::collections::VecDeque;

use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    let (_input, gates) = parse(&contents);

    let mut output_gates: Vec<_> = gates
        .iter()
        .filter(|g| g.output.starts_with("z"))
        .cloned()
        .collect();
    output_gates.sort_by_key(|g| g.output);

    let output_count = output_gates.len();

    let mut gate_map: FxHashMap<_, _> = FxHashMap::default();
    for gate in gates {
        gate_map.insert(gate.output, gate);
    }

    let mut prev_input_map = FxHashMap::<(&str, &str), usize>::default();

    let mut wrong = Vec::new();

    for (i, output_gate) in output_gates.into_iter().enumerate() {
        prev_input_map.insert((output_gate.input1, output_gate.input2), i);
        prev_input_map.insert((output_gate.input2, output_gate.input1), i);
        let invalid_output = match i {
            0 => valid_output_gate0(&output_gate),
            1 => valid_output_gate1(&output_gate, &gate_map),
            i if i == output_count - 1 => {
                valid_output_gate_last(&output_gate, &gate_map, i, &prev_input_map)
            }
            i => valid_output_gate(&output_gate, &gate_map, i, &prev_input_map),
        };
        if let Some(invalid_output) = invalid_output {
            wrong.push(invalid_output.0);
        }
    }
    wrong.sort();

    wrong.join(",")
}

fn valid_output_gate0<'a>(output_gate: &Gate<'a>) -> Option<(&'a str, &'static str)> {
    let valid_gate_type = output_gate.gate_type == GateType::XOR;
    let valid_gates = valid_gate_values(output_gate, "x00", "y00");
    if !valid_gate_type || !valid_gates {
        return Some((output_gate.output, "wrong values"));
    }
    None
}

fn valid_gate_values(output_gate: &Gate<'_>, value1: &str, value2: &str) -> bool {
    (output_gate.input1 == value1 && output_gate.input2 == value2)
        || (output_gate.input1 == value2 && output_gate.input2 == value1)
}

fn valid_output_gate1<'a>(
    output_gate: &Gate<'a>,
    gate_map: &'a FxHashMap<&'a str, Gate<'a>>,
) -> Option<(&'a str, &'static str)> {
    if output_gate.gate_type != GateType::XOR {
        return Some((output_gate.output, "gate type"));
    }

    let (left_gate, right_gate) =
        match match_gates(output_gate, GateType::AND, GateType::XOR, gate_map) {
            Ok(value) => value,
            Err(value) => return value,
        };
    if valid_gate_values(left_gate, "x00", "y00") {
        if valid_gate_values(right_gate, "x01", "y01") {
            return None;
        }
        return Some((right_gate.output, "right gate wrong values"));
    }
    if valid_gate_values(right_gate, "x00", "y00") {
        if valid_gate_values(left_gate, "x01", "y01") {
            return None;
        }
        return Some((left_gate.output, "left gate wrong values"));
    }

    Some((left_gate.output, "wrong values"))
}

fn valid_output_gate<'a>(
    output_gate: &Gate<'a>,
    gate_map: &'a FxHashMap<&'a str, Gate<'a>>,
    gate_index: usize,
    prev_input_map: &FxHashMap<(&str, &str), usize>,
) -> Option<(&'a str, &'static str)> {
    if output_gate.gate_type != GateType::XOR {
        return Some((output_gate.output, "gate type"));
    }

    let gate_output = format!("{:0>2}", gate_index);

    let (left_gate, right_gate) =
        match match_gates(output_gate, GateType::XOR, GateType::OR, gate_map) {
            Ok(value) => value,
            Err(_value) => {
                // check if one input is correct
                let left = gate_map.get(output_gate.input1).unwrap();
                let right = gate_map.get(output_gate.input2).unwrap();
                if left.gate_type == GateType::XOR
                    && valid_gate_values(
                        left,
                        &format!("x{}", gate_output),
                        &format!("y{}", gate_output),
                    )
                {
                    return Some((output_gate.input2, "left was correct"));
                } else if right.gate_type == GateType::XOR
                    && valid_gate_values(
                        right,
                        &format!("x{}", gate_output),
                        &format!("y{}", gate_output),
                    )
                {
                    return Some((output_gate.input1, "right was correct"));
                }
                return Some((output_gate.input2, "both were wrong"));
            }
        };
    // left: XOR sum
    // right: OR carry

    if !valid_gate_values(
        left_gate,
        &format!("x{}", gate_output),
        &format!("y{}", gate_output),
    ) {
        return Some((left_gate.output, "right gate wrong values"));
    }

    check_carry(gate_map, prev_input_map, gate_index, right_gate)
}

fn check_carry<'a>(
    gate_map: &'a std::collections::HashMap<&str, Gate<'a>, rustc_hash::FxBuildHasher>,
    prev_input_map: &std::collections::HashMap<(&str, &str), usize, rustc_hash::FxBuildHasher>,
    gate_index: usize,
    gate: &'_ Gate<'a>,
) -> Option<(&'a str, &'static str)> {
    let prev_gate_output = format!("{:0>2}", gate_index - 1);

    let (left_gate, right_gate) = match match_gates(gate, GateType::AND, GateType::AND, gate_map) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let prev_x = format!("x{}", prev_gate_output);
    let prev_y = format!("y{}", prev_gate_output);

    if valid_gate_values(left_gate, &prev_x, &prev_y) {
        if prev_input_map.contains_key(&(right_gate.input1, right_gate.input2)) {
            return None;
        }
        return Some((right_gate.output, "right gate carry wrong values"));
    } else if valid_gate_values(right_gate, &prev_x, &prev_y) {
        if prev_input_map.contains_key(&(left_gate.input1, left_gate.input2)) {
            return None;
        }
        return Some((right_gate.output, "left gate carry wrong values"));
    }

    Some((left_gate.output, "wrong carry values"))
}

fn valid_output_gate_last<'a>(
    output_gate: &Gate<'a>,
    gate_map: &'a FxHashMap<&'a str, Gate<'a>>,
    gate_index: usize,
    prev_input_map: &FxHashMap<(&str, &str), usize>,
) -> Option<(&'a str, &'static str)> {
    if output_gate.gate_type != GateType::OR {
        return Some((output_gate.output, "gate type"));
    }

    check_carry(gate_map, prev_input_map, gate_index, output_gate)
}

fn match_gates<'a>(
    output_gate: &Gate<'a>,
    gate1_type: GateType,
    gate2_type: GateType,
    gate_map: &'a std::collections::HashMap<&'a str, Gate<'a>, rustc_hash::FxBuildHasher>,
) -> Result<(&'a Gate<'a>, &'a Gate<'a>), Option<(&'a str, &'static str)>> {
    Ok(
        match get_gate_with_type(output_gate.input1, gate1_type, gate_map) {
            Ok(value) => {
                let left_gate = match get_gate_with_type(output_gate.input2, gate2_type, gate_map) {
                    Ok(value) => value,
                    Err(value) => return Err(value),
                };
                (value, left_gate)
            }
            Err(_) => match get_gate_with_type(output_gate.input2, gate1_type, gate_map) {
                Ok(value) => {
                    let left_gate =
                        match get_gate_with_type(output_gate.input1, gate2_type, gate_map) {
                            Ok(value) => value,
                            Err(value) => return Err(value),
                        };
                    (value, left_gate)
                }
                Err(_) => {
                    match get_gate_with_type(output_gate.input2, gate2_type, gate_map) {
                        Ok(_) => {
                            return Err(Some((output_gate.input1, "input 2 has correct type")))
                        }
                        Err(_value) => {
                            return Err(Some((output_gate.input2, "both have wrong type")))
                        }
                    };
                }
            },
        },
    )
}

fn get_gate_with_type<'a>(
    output: &'a str,
    gate_type: GateType,
    gate_map: &'a FxHashMap<&str, Gate<'a>>,
) -> Result<&'a Gate<'a>, Option<(&'a str, &'static str)>> {
    Ok(match gate_map.get(output) {
        Some(g) => {
            if g.gate_type != gate_type {
                return Err(Some((output, "gate type")));
            }
            g
        }
        None => {
            return Err(Some((output, "gate not found")));
        }
    })
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
