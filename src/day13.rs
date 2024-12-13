#[derive(Debug)]
struct Input {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

pub fn part1(contents: String) -> String {
    let a_cost = 3;
    let b_cost = 1;
    let machines = parse(contents);

    let mut sum = 0;

    for Input { a, b, prize } in machines {
        let mut solution = None;
        for a_presses in 0..100 {
            let position = (a.0 * a_presses, a.1 * a_presses);
            // if prize.0 % b.0 != 0 || prize.1 % b.1 != 0 {
            //     continue;
            // }
            let b_presses = (prize.0 - position.0) / b.0;

            if b_presses > 100 {
                continue;
            }
            let calced = (position.0 + (b_presses * b.0), position.1 + b_presses * b.1);

            // dbg!(position);
            if calced != prize {
                continue;
            }
            solution = Some((a_presses, b_presses));
            break;
        }
        if let Some((a_presses, b_presses)) = solution {
            sum += a_presses * a_cost + b_presses * b_cost;
        }
        // let solved = solve(&machine, (0, 0), 0, 0, 0);
        // dbg!(solved);
    }

    sum.to_string()
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

pub fn part2(contents: String) -> String {
    "example2".to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/13/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/13/real.txt").unwrap()),
            "example2"
        );
    }
}
