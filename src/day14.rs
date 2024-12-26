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

    move_robots(&mut robots, width, height, 100);

    let quadrants = find_quadrants(robots.iter().map(|r| r.position), height, width);
    quadrants.into_iter().product::<u64>().to_string()
}

fn move_robots(robots: &mut [Robot], width: i64, height: i64, times: i64) {
    for robot in robots.iter_mut() {
        let (x, y) = robot.position;
        let (vx, vy) = robot.velocity;
        let new_position = (
            (x + vx * times).rem_euclid(width),
            (y + vy * times).rem_euclid(height),
        );
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
        let up = y < (height / 2);
        let down = y > (height / 2);
        let left = x < (width / 2);
        let right = x > (width / 2);
        if up && left {
            // 1
            quadrants[0] += 1;
        } else if up && right {
            // 2
            quadrants[1] += 1;
        } else if down && left {
            // 3
            quadrants[2] += 1;
        } else if down && right {
            // 4
            quadrants[3] += 1;
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
        move_robots(&mut robots, width, height, 1);

        let has_christmas_tree_line = has_line(&robots, width, height, 10);
        if has_christmas_tree_line {
            // print_map(second, &robots, width, height);
            break;
        }
    }
    second.to_string()
}

fn has_line(robots: &[Robot], width: i64, height: i64, min_length: i32) -> bool {
    let mut y_robots: Vec<Vec<_>> = vec![vec![]; height as usize];
    for robot in robots {
        y_robots[robot.position.1 as usize].push(robot.position.0);
    }
    for y in 0..height {
        y_robots[y as usize].sort();
        let robot_line = &y_robots[y as usize];
        if robot_line.len() < (min_length as usize) {
            continue;
        }
        let mut continous = 0;
        for w in robot_line.windows(2) {
            if w[0] == w[1] - 1 {
                continous += 1;
            } else {
                continous = 0;
            }

            if continous > min_length {
                return true;
            }
        }
    }
    false
}

#[allow(dead_code)]
fn print_map(second: i32, robots: &[Robot], width: i64, height: i64) {
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
    println!("############# Second: {}\n{}", second, s);
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
