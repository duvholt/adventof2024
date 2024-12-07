pub fn part1(contents: String) -> String {
    let parsed = parse(contents);

    let mut sum = 0;
    for (result, numbers) in parsed {
        if solve(result, numbers[0], &numbers[1..]) {
            sum += result;
        }
    }

    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let parsed = parse(contents);

    let mut sum = 0;
    for (result, numbers) in parsed {
        if solve2(result, numbers[0], &numbers[1..]) {
            sum += result;
        }
    }

    sum.to_string()
}

fn solve(result: u64, sofar: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        if sofar == result {
            return true;
        }
        return false;
    }
    let current = numbers[0];
    if solve(result, sofar + current, &numbers[1..]) {
        return true;
    }
    if solve(result, sofar * current, &numbers[1..]) {
        return true;
    }
    false
}

fn solve2(result: u64, sofar: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        if sofar == result {
            return true;
        }
        return false;
    }
    let current = numbers[0];
    let concat = concat(sofar, current);
    if solve2(result, concat, &numbers[1..]) {
        return true;
    }
    let mult = sofar * current;
    if solve2(result, mult, &numbers[1..]) {
        return true;
    }
    let plus = sofar + current;
    if solve2(result, plus, &numbers[1..]) {
        return true;
    }

    false
}

fn concat(sofar: u64, current: u64) -> u64 {
    let len = ((current + 1) as f64).log10().ceil() as u32;
    sofar * (10u64.pow(len)) + current
}

fn parse(contents: String) -> Vec<(u64, Vec<u64>)> {
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
    parsed
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/7/real.txt").unwrap()),
            "1430271835320"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/7/real.txt").unwrap()),
            "456565678667482"
        );
    }
}
