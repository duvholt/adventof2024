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
        let safe = safe_report(&levels);
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
        let safe = safe_report(&levels);
        if safe {
            sum += 1;
        } else {
            for skip_i in 0..levels.len() {
                let safe = safe_report_with_skip(skip_i, &levels);
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

fn safe_report(levels: &[i64]) -> bool {
    levels.windows(3).all(|w| {
        let prev = w[0];
        let current = w[1];
        let next = w[2];
        let delta_prev = current - prev;
        let delta_next = next - current;
        if !cond(delta_prev) || !cond(delta_next) {
            false
        } else {
            delta_prev.signum() == delta_next.signum()
        }
    })
}

fn safe_report_with_skip(skip_i: usize, levels: &[i64]) -> bool {
    let l = levels
        .iter()
        .enumerate()
        .filter_map(|(i, e)| if i != skip_i { Some(*e) } else { None })
        .collect::<Vec<_>>();
    safe_report(&l)
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
