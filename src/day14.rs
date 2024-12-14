use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

pub fn part1(contents: String) -> String {
    let mut robots = parse(contents);

    let width = 101;
    let height = 103;

    for _second in 0..100 {
        for robot in robots.iter_mut() {
            let (x, y) = robot.position;
            let (vx, vy) = robot.velocity;
            let new_position = ((x + vx).rem_euclid(width), (y + vy).rem_euclid(height));
            robot.position = new_position;
        }
    }

    let mut quadrants = [0; 4];
    for robot in robots {
        let (x, y) = robot.position;
        if y < (height / 2) {
            if x < (width / 2) {
                // 1
                quadrants[0] += 1;
            } else if x > (width / 2) {
                // 2
                quadrants[1] += 1;
            }
        } else if y > (height / 2) {
            if x < (width / 2) {
                // 3
                quadrants[2] += 1;
            } else if x > (width / 2) {
                // 4
                quadrants[3] += 1;
            }
        }
    }

    quadrants.into_iter().product::<u64>().to_string()
}

fn parse(contents: String) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots: Vec<_> = contents
        .lines()
        .map(|line| {
            let (_, [x, y, vx, vy]) = re.captures(line).map(|c| c.extract()).unwrap();

            Robot {
                position: (x.parse().unwrap(), y.parse().unwrap()),
                velocity: (vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect();
    robots
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
            part1(fs::read_to_string("./input/14/real.txt").unwrap()),
            "222208000"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/14/real.txt").unwrap()),
            "example2"
        );
    }
}
