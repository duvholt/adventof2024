use rustc_hash::FxHashMap;

#[derive(Debug)]
enum Gate {
    AND,
    OR,
    XOR,
}

pub fn part1(contents: String) -> String {
    let mut parts = contents.split("\n\n");
    let mut input: FxHashMap<&str, u8> = parts
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

    let mut gates: Vec<(&str, Gate, &str, &str)> = parts
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

    dbg!(&input);

    while let Some(gate) = gates.pop() {
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
            // bad
            gates.insert(0, gate);
        }
    }

    let mut output: Vec<_> = input.iter().filter(|(k, v)| k.starts_with("z")).collect();
    output.sort_by_key(|v| v.0);

    let s: u64 = output
        .into_iter()
        .enumerate()
        .map(|(i, v)| (*v.1 as u64) << i)
        .sum();

    s.to_string()
}

pub fn part2(_contents: String) -> String {
    "example2".to_string()
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
