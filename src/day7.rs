pub fn part1(contents: String) -> String {
    let parsed: Vec<(u64, Vec<u64>)> = contents
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result: u64 = parts.next().unwrap().parse().unwrap();
            let numbers: Vec<u64> = parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (result, numbers)
        })
        .collect();

    let mut sum = 0;
    for (result, numbers) in parsed {
        let local_solutions = solve(result, 0, &numbers);
        if (local_solutions > 0) {
            sum += result;
        }
    }

    sum.to_string()
}

fn solve(result: u64, sofar: u64, numbers: &[u64]) -> u64 {
    let mut sum = 0;
    if sofar == result {
        sum += 1;
    }
    if numbers.is_empty() {
        return sum;
    }
    let current = numbers[0];
    sum += solve(result, sofar + current, &numbers[1..]);
    sum += solve(result, sofar * current, &numbers[1..]);
    sum += solve(result, sofar, &numbers[1..]);
    sum
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
            part1(fs::read_to_string("./input/7/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/7/real.txt").unwrap()),
            "example2"
        );
    }
}
