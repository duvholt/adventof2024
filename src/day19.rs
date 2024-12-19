pub fn part1(contents: String) -> String {
    let mut parts = contents.split("\n\n");
    let patterns: Vec<&str> = parts.next().unwrap().split(", ").collect();

    let towel_designs: Vec<_> = parts.next().unwrap().lines().collect();

    let mut possible = 0;
    for towel_design in towel_designs {
        let mut stack = Vec::new();
        stack.push(towel_design.to_string());
        while let Some(wip_design) = stack.pop() {
            if wip_design.is_empty() {
                possible += 1;
                break;
            }
            for pattern in patterns.iter() {
                if let Some(stripped) = wip_design.strip_prefix(pattern) {
                    stack.push(stripped.to_string());
                }
            }
        }
    }

    possible.to_string()
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
            part1(fs::read_to_string("./input/19/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/19/real.txt").unwrap()),
            "example2"
        );
    }
}
