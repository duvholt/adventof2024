use regex::Regex;

pub fn part1(contents: String) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = vec![];
    for (_, [first, second]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results.push((
            first.parse::<u64>().unwrap(),
            second.parse::<u64>().unwrap(),
        ));
    }
    results
        .into_iter()
        .map(|(f, s)| f * s)
        .sum::<u64>()
        .to_string()
}

pub fn part2(contents: String) -> String {
    let re = Regex::new(r"((mul)\(\d+,\d+\)|(don\'t)\(\)|(do)\(\))").unwrap();
    let mut results = vec![];
    let mut enabled = true;
    for (instr, [_, command]) in re.captures_iter(&contents).map(|c| c.extract()) {
        match command {
            "mul" => {
                if enabled {
                    results.push(instr);
                }
            }
            "don't" => {
                enabled = false;
            }
            "do" => {
                enabled = true;
            }
            _ => {}
        }
    }

    part1(results.join(" "))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/3/real.txt").unwrap()),
            "192767529"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/3/real.txt").unwrap()),
            "104083373"
        );
    }
}
