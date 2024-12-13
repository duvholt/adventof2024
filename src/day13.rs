#[derive(Debug)]
struct Input {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

pub fn part1(contents: String) -> String {
    let machines = parse(contents);
    let sum = claw_inputs_sum(machines, 0);
    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let machines = parse(contents);
    let sum = claw_inputs_sum(machines, 10000000000000);
    sum.to_string()
}

fn claw_inputs_sum(machines: Vec<Input>, prize_offset: i64) -> i64 {
    let a_cost = 3;
    let b_cost = 1;

    let mut sum = 0;

    for Input { a, b, prize } in machines.into_iter() {
        let prize = (prize.0 + prize_offset, prize.1 + prize_offset);

        let (a_presses, b_presses, calculated_position) = machine_presses(a, b, prize);

        if calculated_position == prize {
            sum += a_presses * a_cost + b_presses * b_cost;
        }
    }
    sum
}

fn machine_presses(a: (i64, i64), b: (i64, i64), prize: (i64, i64)) -> (i64, i64, (i64, i64)) {
    let a1 = a.0;
    let a2 = a.1;

    let b1 = b.0;
    let b2 = b.1;

    let c1 = prize.0;
    let c2 = prize.1;

    // Cramer's rule
    let a_presses = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);
    let position = (a.0 * a_presses, a.1 * a_presses);

    let b_presses = (prize.0 - position.0) / b.0;

    let calculated_position = (position.0 + (b_presses * b.0), position.1 + b_presses * b.1);
    (a_presses, b_presses, calculated_position)
}

fn parse(contents: String) -> Vec<Input> {
    let input: Vec<_> = contents
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.lines();
            let a = lines.next().unwrap().strip_prefix("Button A: X+").unwrap();
            let a: Vec<i64> = a
                .split(", Y+")
                .map(|a1| a1.parse::<i64>().unwrap())
                .collect();
            let b = lines.next().unwrap().strip_prefix("Button B: X+").unwrap();
            let b: Vec<i64> = b
                .split(", Y+")
                .map(|a1| a1.parse::<i64>().unwrap())
                .collect();
            let prize = lines.next().unwrap().strip_prefix("Prize: X=").unwrap();
            let prize: Vec<i64> = prize
                .split(", Y=")
                .map(|a1| a1.parse::<i64>().unwrap())
                .collect();
            Input {
                a: (a[0], a[1]),
                b: (b[0], b[1]),
                prize: (prize[0], prize[1]),
            }
        })
        .collect();
    input
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/13/real.txt").unwrap()),
            "34393"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/13/real.txt").unwrap()),
            "83551068361379"
        );
    }
}
