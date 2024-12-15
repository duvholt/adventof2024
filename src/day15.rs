#[derive(PartialEq, Debug, Clone)]
enum Block {
    Wall,
    Box,
    Empty,
    Robot,
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn part1(contents: String) -> String {
    let (mut map_grid, moves) = parse(contents);

    let mut robot_position = (0, 0);

    for y in 0..map_grid.len() {
        for x in 0..map_grid[0].len() {
            if map_grid[y][x] == Block::Robot {
                robot_position = (x, y);
                break;
            }
        }
    }

    for robot_move in moves {
        let (robot_x, robot_y) = robot_position;
        let (new_x, new_y) = next_position(&robot_move, robot_position);
        match map_grid[new_y][new_x] {
            Block::Wall => (),
            Block::Box => {
                let moved = move_boxes(&robot_move, (new_x, new_y), Block::Robot, &mut map_grid);
                if moved {
                    map_grid[robot_y][robot_x] = Block::Empty;
                    robot_position = (new_x, new_y);
                }
            }
            Block::Empty => {
                // move robot
                map_grid[new_y][new_x] = Block::Robot;
                map_grid[robot_y][robot_x] = Block::Empty;
                robot_position = (new_x, new_y);
            }
            Block::Robot => unreachable!("robot moved into robot"),
        }
        // print_map(&map_grid, &robot_move);
    }
    let mut sum = 0;
    for y in 0..map_grid.len() {
        for x in 0..map_grid[y].len() {
            if map_grid[y][x] == Block::Box {
                sum += 100 * y + x;
            }
        }
    }

    sum.to_string()
}

fn move_boxes(
    robot_move: &Direction,
    (x, y): (usize, usize),
    prev_block: Block,
    map_grid: &mut Vec<Vec<Block>>,
) -> bool {
    let new_box_position = next_position(robot_move, (x, y));
    let moved = match map_grid[new_box_position.1][new_box_position.0] {
        Block::Wall => false,
        Block::Box => move_boxes(robot_move, new_box_position, Block::Box, map_grid),
        Block::Empty => true,
        Block::Robot => unreachable!("robot into box into robot?"),
    };
    if moved {
        map_grid[new_box_position.1][new_box_position.0] = Block::Box;
        map_grid[y][x] = prev_block;
    }
    moved
}

fn next_position(robot_move: &Direction, position: (usize, usize)) -> (usize, usize) {
    let (new_x, new_y) = match robot_move {
        Direction::Up => (position.0, position.1 - 1),
        Direction::Right => (position.0 + 1, position.1),
        Direction::Down => (position.0, position.1 + 1),
        Direction::Left => (position.0 - 1, position.1),
    };
    (new_x, new_y)
}

fn parse(contents: String) -> (Vec<Vec<Block>>, Vec<Direction>) {
    let mut parts = contents.split("\n\n");
    let map_part = parts.next().unwrap();
    let moves_part = parts.next().unwrap();
    let mut map_grid = Vec::new();
    for line in map_part.lines() {
        let map_line: Vec<_> = line
            .chars()
            .map(|c| match c {
                '#' => Block::Wall,
                '.' => Block::Empty,
                '@' => Block::Robot,
                'O' => Block::Box,
                _ => panic!("Unknown block"),
            })
            .collect();
        map_grid.push(map_line);
    }

    let moves: Vec<_> = moves_part
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            d => panic!("Unknown direction: {}", d),
        })
        .collect();
    (map_grid, moves)
}

#[allow(dead_code)]
fn print_map(map_grid: &[Vec<Block>], robot_move: &Direction) {
    println!("Direction: {:#?}", robot_move);
    let mut s = String::new();
    for y in 0..map_grid.len() {
        for x in 0..map_grid[0].len() {
            let letter = match map_grid[y][x] {
                Block::Wall => '#',
                Block::Box => 'O',
                Block::Empty => '.',
                Block::Robot => '@',
            };
            s.push(letter);
        }
        s.push('\n');
    }
    println!("{}", s);
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
            part1(fs::read_to_string("./input/15/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/15/real.txt").unwrap()),
            "example2"
        );
    }
}
