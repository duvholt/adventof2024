use std::collections::HashSet;

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
        move_robots(&mut robots, width, height);
    }

    let quadrants = find_quadrants(robots.iter().map(|r| r.position), height, width);
    quadrants.into_iter().product::<u64>().to_string()
}

fn move_robots(robots: &mut [Robot], width: i64, height: i64) {
    for robot in robots.iter_mut() {
        let (x, y) = robot.position;
        let (vx, vy) = robot.velocity;
        let new_position = ((x + vx).rem_euclid(width), (y + vy).rem_euclid(height));
        robot.position = new_position;
    }
}

fn find_quadrants(
    positions: impl Iterator<Item = (i64, i64)>,
    height: i64,
    width: i64,
) -> [u64; 4] {
    let mut quadrants = [0; 4];
    for position in positions {
        let (x, y) = position;
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
    quadrants
}

pub fn part2(contents: String) -> String {
    let mut robots = parse(contents);

    let width = 101;
    let height = 103;

    let mut second = 0;
    loop {
        second += 1;
        move_robots(&mut robots, width, height);

        let has_christmas_tree_line = print_map(second, &robots, width, height);
        if has_christmas_tree_line {
            break;
        }
    }
    second.to_string()
}

fn print_map(second: i32, robots: &[Robot], width: i64, height: i64) -> bool {
    let mut s = String::new();
    let positions: HashSet<_> = robots.iter().map(|r| r.position).collect();
    for y in 0..height {
        for x in 0..width {
            if positions.contains(&(x, y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    // println!("############# Second: {}\n{}", second, s);
    s.contains("#########################")
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
            "7623"
        );
    }
}
