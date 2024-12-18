use std::collections::BinaryHeap;

use rustc_hash::FxHashSet;

pub fn part1(contents: String) -> String {
    let (instructions, (register_a, register_b, register_c)) = parse(contents);

    let outputs = run_computer(&instructions, register_a, register_b, register_c);

    let a: Vec<_> = outputs.into_iter().map(|c| c.to_string()).collect();
    a.join(",")
}

#[derive(PartialEq, Eq)]
struct Node(usize);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

pub fn part2(contents: String) -> String {
    let (instructions, (_register_a, register_b, register_c)) = parse(contents);

    // min heap
    let mut heap = BinaryHeap::new();
    heap.push(Node(0));

    let mut visited = FxHashSet::default();
    while let Some(Node(current_register_a)) = heap.pop() {
        // solve by finding possible values for bit values left to right
        for byte in 0..8 {
            let new_register_a = (current_register_a << 3) + byte;
            if visited.contains(&new_register_a) {
                continue;
            }
            let calculated =
                run_computer(&instructions, new_register_a as u64, register_b, register_c);
            if instructions == calculated {
                return new_register_a.to_string();
            }
            let correct_instruction = instructions[instructions.len() - calculated.len()];
            let calculated_instruction = calculated[0];
            if correct_instruction == calculated_instruction {
                heap.push(Node(new_register_a));
            }
        }
        visited.insert(current_register_a);
    }
    panic!("unable to find a solution")
}

fn run_computer(
    instructions: &[u64],
    mut register_a: u64,
    mut register_b: u64,
    mut register_c: u64,
) -> Vec<u64> {
    let mut pc = 0;
    let mut outputs = Vec::new();
    while pc < instructions.len() {
        let instruction = instructions[pc];
        match instruction {
            // adv, combo
            0 => {
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);

                let result = division(register_a, operand);
                register_a = result;
                pc += 2;
            }
            // bxl, literal
            1 => {
                let operand = instructions[pc + 1];
                let result = register_b ^ operand;
                register_b = result;
                pc += 2;
            }
            // bst
            2 => {
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);
                let result = operand.rem_euclid(8);
                register_b = result;
                pc += 2;
            }
            // jnz
            3 => {
                if register_a != 0 {
                    let operand = instructions[pc + 1];
                    pc = operand as usize;
                } else {
                    pc += 2;
                }
            }
            // bxc
            4 => {
                register_b ^= register_c;
                pc += 2;
            }
            // out
            5 => {
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);
                let result = operand.rem_euclid(8);
                outputs.push(result);
                pc += 2;
            }
            // bdv
            6 => {
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);

                let result = division(register_a, operand);
                register_b = result;
                pc += 2;
            }
            // cdv
            7 => {
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);

                let result = division(register_a, operand);
                register_c = result;
                pc += 2;
            }
            _ => panic!("Unknown instruction"),
        }
    }

    // dbg!(register_a, register_b, register_c);

    outputs
}

fn division(register_a: u64, operand: u64) -> u64 {
    let num = register_a;

    let den = 2u64.pow(operand as u32);

    num / den
}

fn combo_operator(register_a: u64, register_b: u64, register_c: u64, operand: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => register_a,
        5 => register_b,
        6 => register_c,
        7 => panic!("reserved"),
        _ => unreachable!(),
    }
}

fn parse(contents: String) -> (Vec<u64>, (u64, u64, u64)) {
    let mut lines = contents.lines();
    let register_a = parse_register(&mut lines, "Register A: ");
    let register_b = parse_register(&mut lines, "Register B: ");
    let register_c = parse_register(&mut lines, "Register C: ");
    lines.next();
    let instructions: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();
    (instructions, (register_a, register_b, register_c))
}

fn parse_register(lines: &mut std::str::Lines<'_>, prefix: &str) -> u64 {
    lines
        .next()
        .unwrap()
        .strip_prefix(prefix)
        .unwrap()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/17/real.txt").unwrap()),
            "2,1,4,7,6,0,3,1,4"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/17/real.txt").unwrap()),
            "266932601404433"
        );
    }
}
