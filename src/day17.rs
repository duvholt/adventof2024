pub fn part1(contents: String) -> String {
    let (instructions, (mut register_a, mut register_b, mut register_c)) = parse(contents);

    let mut pc = 0;
    let mut outputs = Vec::new();
    while pc < instructions.len() {
        let instruction = instructions[pc];
        // dbg!(pc, register_a, register_b, register_c, instruction);
        match instruction {
            // adv, combo
            0 => {
                println!("adv");
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);

                let result = division(register_a, operand);
                register_a = result;
                pc += 2;
            }
            // bxl, literal
            1 => {
                println!("bxl");
                let operand = instructions[pc + 1];
                let result = register_b ^ operand;
                register_b = result;
                pc += 2;
            }
            // bst
            2 => {
                println!("bst");
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);
                // todo: check this
                let result = operand.rem_euclid(8);
                // dbg!(operand, 8, result, register_b);
                register_b = result;
                pc += 2;
            }
            // jnz
            3 => {
                println!("jnz");
                if register_a != 0 {
                    let operand = instructions[pc + 1];
                    pc = operand as usize;
                } else {
                    pc += 2;
                }
            }
            // bxc
            4 => {
                println!("bxc");
                // todo: check
                register_b = register_b ^ register_c;
                pc += 2;
            }
            // out
            5 => {
                println!("out");
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);
                let result = operand.rem_euclid(8);
                outputs.push(result);
                pc += 2;
            }
            // bdv
            6 => {
                println!("bdv");
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);

                let result = division(register_a, operand);
                register_b = result;
                pc += 2;
            }
            // cdv
            7 => {
                println!("cdv");
                let operand = instructions[pc + 1];
                let operand = combo_operator(register_a, register_b, register_c, operand);

                let result = division(register_a, operand);
                register_c = result;
                pc += 2;
            }
            _ => panic!("Unknown instruction"),
        }
    }

    dbg!(register_a, register_b, register_c);

    // for output in
    let a: Vec<_> = outputs.into_iter().map(|c| c.to_string()).collect();
    a.join(",")
}

fn division(register_a: u64, operand: u64) -> u64 {
    let num = register_a;

    let den = 2u64.pow(operand as u32);
    let result = num / den;
    result
}

fn combo_operator(register_a: u64, register_b: u64, register_c: u64, operand: u64) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand,
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
            part1(fs::read_to_string("./input/17/real.txt").unwrap()),
            "2,1,4,7,6,0,3,1,4"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/17/real.txt").unwrap()),
            "example2"
        );
    }
}
