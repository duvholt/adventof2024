fn parse_input(contents: String) -> (Vec<u64>, Vec<u64>) {
    let lines: Vec<_> = contents
        .lines()
        .map(|s| {
            let mut a = s.split_whitespace();
            let first = a.next().unwrap().parse::<u64>().unwrap();
            let second = a.next().unwrap().parse::<u64>().unwrap();
            (first, second)
        })
        .collect();
    let mut first: Vec<u64> = lines.iter().map(|(l, _)| *l).collect();
    first.sort();
    let mut second: Vec<u64> = lines.into_iter().map(|(_, l)| l).collect();
    second.sort();
    (first, second)
}

pub fn part1(contents: String) -> String {
    let (first, second) = parse_input(contents);

    let s: u64 = first
        .into_iter()
        .zip(second)
        .map(|(f, s)| f.abs_diff(s))
        .sum();
    s.to_string()
}

pub fn part2(contents: String) -> String {
    let (first, second) = parse_input(contents);

    let s: u64 = first
        .into_iter()
        .map(|f| second.iter().filter(|&&c| c == f).sum::<u64>())
        .sum();
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
