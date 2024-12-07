pub fn part1(contents: String) -> String {
    let parsed = parse(contents);

    let mut sum = 0;
    let mut stack = Vec::new();
    for (result, numbers) in parsed {
        stack.push((numbers[0], 1));
        while let Some((sofar, i)) = stack.pop() {
            if numbers.len() == i {
                if sofar == result {
                    sum += result;
                    break;
                }
                continue;
            }
            if sofar > result {
                continue;
            }
            let next = numbers[i];
            stack.push((sofar + next, i + 1));
            stack.push((sofar * next, i + 1));
        }
        stack.clear();
    }

    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let parsed = parse(contents);

    let mut sum = 0;
    let mut stack = Vec::new();
    for (result, numbers) in parsed {
        stack.push((numbers[0], 1));
        while let Some((sofar, i)) = stack.pop() {
            if numbers.len() == i {
                if sofar == result {
                    sum += result;
                    break;
                }
                continue;
            }
            if sofar > result {
                continue;
            }
            let next = numbers[i];
            stack.push((sofar + next, i + 1));
            stack.push((sofar * next, i + 1));
            stack.push((concat(sofar, next), i + 1));
        }
        stack.clear();
    }

    sum.to_string()
}

fn concat(sofar: u64, current: u64) -> u64 {
    sofar * (10u64.pow(current.ilog10() + 1)) + current
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
