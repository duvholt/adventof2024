// safe if both of the following are true:

//     The levels are either all increasing or all decreasing.
//     Any two adjacent levels differ by at least one and at most three.

pub fn part1(contents: String) -> String {
    let reports = contents.lines();
    let mut sum = 0;
    for report in reports {
        let levels: Vec<_> = report
            .split_ascii_whitespace()
            .map(|l| l.parse::<i64>().unwrap())
            .collect();
        let safe = safe_report(&levels, None);
        if safe {
            sum += 1;
        }
    }

    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let reports = contents.lines();
    let mut sum = 0;
    for report in reports {
        let levels: Vec<_> = report
            .split_ascii_whitespace()
            .map(|l| l.parse::<i64>().unwrap())
            .collect();
        let safe = safe_report(&levels, None);
        if safe {
            sum += 1;
        } else {
            for skip_i in 0..levels.len() {
                let safe = safe_report(&levels, Some(skip_i));
                if safe {
                    sum += 1;
                    break;
                }
            }
        }
    }

    sum.to_string()
}

fn cond(delta: i64) -> bool {
    let abs = delta.abs();
    abs > 0 && abs <= 3
}

fn safe_report(levels: &[i64], skip_i: Option<usize>) -> bool {
    let mut i = 0;
    let sign = (levels.last().unwrap() - levels.first().unwrap()).signum();
    while i + 1 < (levels.len()) {
        if skip_i == Some(i) {
            i += 1;
            continue;
        }
        let prev = levels[i];
        let next = if skip_i == Some(i + 1) {
            if i + 2 >= levels.len() {
                i += 1;
                continue;
            }
            levels[i + 2]
        } else {
            levels[i + 1]
        };
        let delta = next - prev;
        if delta.signum() != sign || !cond(delta) {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/2/real.txt").unwrap()),
            "663"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/2/real.txt").unwrap()),
            "692"
        );
    }
}
