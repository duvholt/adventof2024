pub fn part1(contents: String) -> String {
    let lines: Vec<_> = contents
        .lines()
        .map(|s| {
            let mut a = s.split_whitespace();
            (a.next().unwrap(), a.next().unwrap())
        })
        .collect();
    let mut first: Vec<u64> = lines.iter().map(|s| s.0.parse::<u64>().unwrap()).collect();
    first.sort();
    let mut second: Vec<u64> = lines.iter().map(|s| s.1.parse::<u64>().unwrap()).collect();
    second.sort();

    let mut s = 0;
    for (i, f) in first.into_iter().enumerate() {
        s += f.abs_diff(second[i])
    }

    s.to_string()
}
pub fn part2(contents: String) -> String {
    let lines: Vec<_> = contents
        .lines()
        .map(|s| {
            let mut a = s.split_whitespace();
            (a.next().unwrap(), a.next().unwrap())
        })
        .collect();
    let mut first: Vec<u64> = lines.iter().map(|s| s.0.parse::<u64>().unwrap()).collect();
    first.sort();
    let mut second: Vec<u64> = lines.iter().map(|s| s.1.parse::<u64>().unwrap()).collect();
    second.sort();

    let mut s = 0;
    for (i, f) in first.into_iter().enumerate() {
        s += second.iter().filter(|c| **c == f).sum::<u64>();
    }

    s.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/1/real.txt").unwrap()),
            "2580760"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/1/real.txt").unwrap()),
            "25358365"
        );
    }
}
