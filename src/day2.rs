// safe if both of the following are true:

//     The levels are either all increasing or all decreasing.
//     Any two adjacent levels differ by at least one and at most three.

#[derive(PartialEq, Eq)]
enum Dir {
    None,
    Up,
    Down,
}

pub fn part1(contents: String) -> String {
    let reports = contents.lines();
    let mut sum = 0;
    for report in reports {
        let levels: Vec<_> = report
            .split_ascii_whitespace()
            .map(|l| l.parse::<i64>().unwrap())
            .collect();
        let safe = safe_report(None, &levels);
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
        let safe = safe_report(None, &levels);
        if safe {
            sum += 1;
        } else {
            for skip_i in 0..levels.len() {
                let safe = safe_report(Some(skip_i), &levels);
                if safe {
                    sum += 1;
                    break;
                }
            }
        }
    }

    sum.to_string()
}

fn safe_report(skip_i: Option<usize>, levels: &[i64]) -> bool {
    let mut prev_option: Option<(Dir, i64)> = None;
    let mut safe = true;
    for (i, &level) in levels.iter().enumerate() {
        if let Some(skip_i) = skip_i {
            if skip_i == i {
                continue;
            }
        }
        if let Some((dir, prev)) = prev_option {
            let diff = level - prev;
            if diff.abs() > 3 || diff.abs() < 1 {
                safe = false;
                break;
            }
            if diff > 0 && (dir == Dir::Up || dir == Dir::None) {
                prev_option = Some((Dir::Up, level));
            } else if diff < 0 && (dir == Dir::Down || dir == Dir::None) {
                prev_option = Some((Dir::Down, level));
            } else {
                safe = false;
                break;
            }
        } else {
            prev_option = Some((Dir::None, level));
        }
    }
    safe
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
