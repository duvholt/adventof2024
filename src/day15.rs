use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Debug, Clone)]
enum Block {
    Wall,
    Box,
    Empty,
    Robot,
}

#[derive(PartialEq, Debug, Clone)]
enum Block2 {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
    Robot,
}

type Position = (usize, usize);

pub fn part1(contents: String) -> String {
    let (mut map_grid, moves) = parse(contents);

    let mut robot_position = part1::find_start_position(&map_grid);

    for robot_move in moves {
        let (robot_x, robot_y) = robot_position;
        let (new_x, new_y) = next_position(&robot_move, robot_position);
        match map_grid[new_y][new_x] {
            Block::Wall => (),
            Block::Box => {
                let moved =
                    part1::move_boxes(&robot_move, (new_x, new_y), Block::Robot, &mut map_grid);
                if moved {
                    map_grid[robot_y][robot_x] = Block::Empty;
                    robot_position = (new_x, new_y);
                }
            }
            Block::Empty => {
                map_grid[new_y][new_x] = Block::Robot;
                map_grid[robot_y][robot_x] = Block::Empty;
                robot_position = (new_x, new_y);
            }
            Block::Robot => unreachable!("robot moved into robot"),
        }
        // Part1::print_map(&map_grid, &robot_move);
    }
    let sum = part1::sum_boxes(map_grid);

    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let (map_grid, moves) = parse(contents);

    let mut map_grid = part2::expand_map(map_grid);

    let mut robot_position = part2::find_start_position(&map_grid);

    for robot_move in moves {
        let (robot_x, robot_y) = robot_position;
        let (new_x, new_y) = next_position(&robot_move, robot_position);
        match map_grid[new_y][new_x] {
            Block2::Wall => (),
            Block2::BoxLeft | Block2::BoxRight => {
                let boxes_to_from =
                    part2::find_boxes_to_move((new_x, new_y), &map_grid, &robot_move);

                if !boxes_to_from.is_empty() {
                    part2::move_boxes(boxes_to_from, &robot_move, &mut map_grid);
                    map_grid[robot_y][robot_x] = Block2::Empty;
                    map_grid[new_y][new_x] = Block2::Robot;
                    robot_position = (new_x, new_y);
                }
            }
            Block2::Empty => {
                map_grid[new_y][new_x] = Block2::Robot;
                map_grid[robot_y][robot_x] = Block2::Empty;
                robot_position = (new_x, new_y);
            }
            Block2::Robot => unreachable!("robot moved into robot"),
        }
        part2::print_map(&map_grid, &robot_move);
    }
    let sum = part2::sum_boxes(map_grid);

    sum.to_string()
}

mod part1 {
    use super::{Block, Direction};
    use crate::day15::next_position;

    pub fn find_start_position(map_grid: &[Vec<Block>]) -> (usize, usize) {
        let mut robot_position = (0, 0);

        for y in 0..map_grid.len() {
            for x in 0..map_grid[0].len() {
                if map_grid[y][x] == Block::Robot {
                    robot_position = (x, y);
                    break;
                }
            }
        }
        robot_position
    }

    pub fn move_boxes(
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

    pub fn sum_boxes(map_grid: Vec<Vec<Block>>) -> usize {
        let mut sum = 0;
        for y in 0..map_grid.len() {
            for x in 0..map_grid[0].len() {
                if map_grid[y][x] == Block::Box {
                    sum += 100 * y + x;
                }
            }
        }
        sum
    }

    #[allow(dead_code)]
    pub fn print_map(map_grid: &[Vec<Block>], robot_move: &Direction) {
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
}

mod part2 {
    use crate::day15::next_position;
    use std::collections::HashSet;

    use super::{Block, Block2, Direction, Position};

    pub fn expand_map(map_grid: Vec<Vec<Block>>) -> Vec<Vec<Block2>> {
        let mut new = Vec::new();
        for line in map_grid {
            let mut new_line = Vec::new();
            for l in line {
                match l {
                    Block::Wall => {
                        new_line.push(Block2::Wall);
                        new_line.push(Block2::Wall);
                    }
                    Block::Box => {
                        new_line.push(Block2::BoxLeft);
                        new_line.push(Block2::BoxRight);
                    }
                    Block::Empty => {
                        new_line.push(Block2::Empty);
                        new_line.push(Block2::Empty);
                    }
                    Block::Robot => {
                        new_line.push(Block2::Robot);
                        new_line.push(Block2::Empty);
                    }
                }
            }
            new.push(new_line);
        }
        new
    }

    pub fn find_start_position(map_grid: &[Vec<Block2>]) -> (usize, usize) {
        let mut robot_position = (0, 0);

        for y in 0..map_grid.len() {
            for x in 0..map_grid[0].len() {
                if map_grid[y][x] == Block2::Robot {
                    robot_position = (x, y);
                    break;
                }
            }
        }
        robot_position
    }

    pub fn find_boxes_to_move(
        start_box: Position,
        map_grid: &[Vec<Block2>],
        robot_move: &Direction,
    ) -> Vec<(Position, Position, Block2)> {
        let mut stack = vec![start_box];
        let mut visited = HashSet::new();
        let mut boxes_to_from = Vec::new();
        let mut can_move = true;
        while let Some(block) = stack.pop() {
            if visited.contains(&block) {
                continue;
            }
            let block_type = map_grid[block.1][block.0].clone();
            visited.insert(block);
            match block_type {
                Block2::Wall => {
                    can_move = false;
                    break;
                }
                Block2::BoxLeft => {
                    let new_box_position = next_position(robot_move, block);
                    // next in queue
                    stack.push(new_box_position);

                    let right_box = (block.0 + 1, block.1);

                    stack.push(right_box);

                    boxes_to_from.push((block, new_box_position, Block2::BoxLeft));
                }
                Block2::BoxRight => {
                    let new_box_position = next_position(robot_move, block);
                    // next in queue
                    stack.push(new_box_position);

                    let left_box = (block.0 - 1, block.1);

                    stack.push(left_box);

                    boxes_to_from.push((block, new_box_position, Block2::BoxRight));
                }
                Block2::Empty => {}
                Block2::Robot => unreachable!("wtf how"),
            };
        }
        if can_move {
            boxes_to_from
        } else {
            Vec::new()
        }
    }

    pub fn move_boxes(
        boxes_to_from: Vec<(Position, Position, Block2)>,
        robot_move: &Direction,
        map_grid: &mut [Vec<Block2>],
    ) {
        let boxes_to_move: HashSet<_> = boxes_to_from
            .iter()
            .map(|(from, _, _)| from)
            .cloned()
            .collect();
        for (from, to, block_type) in boxes_to_from {
            // if box is outside of boxes to move we need to empty the old position
            if *robot_move == Direction::Up || *robot_move == Direction::Down {
                let opposite_direction = if *robot_move == Direction::Up {
                    Direction::Down
                } else {
                    Direction::Up
                };
                let opposite_position = next_position(&opposite_direction, from);
                if !boxes_to_move.contains(&opposite_position) {
                    map_grid[from.1][from.0] = Block2::Empty;
                }
            }
            map_grid[to.1][to.0] = block_type;
        }
    }

    pub fn sum_boxes(map_grid: Vec<Vec<Block2>>) -> usize {
        let mut sum = 0;
        for y in 0..map_grid.len() {
            for x in 0..map_grid[0].len() {
                if map_grid[y][x] == Block2::BoxLeft {
                    sum += 100 * y + x;
                }
            }
        }
        sum
    }

    #[allow(dead_code)]
    pub fn print_map(map_grid: &[Vec<Block2>], robot_move: &Direction) {
        println!("Direction: {:#?}", robot_move);
        let mut s = String::new();
        for y in 0..map_grid.len() {
            for x in 0..map_grid[0].len() {
                let letter = match map_grid[y][x] {
                    Block2::Wall => '#',
                    Block2::BoxLeft => '[',
                    Block2::BoxRight => ']',
                    Block2::Empty => '.',
                    Block2::Robot => '@',
                };
                s.push(letter);
            }
            s.push('\n');
        }
        println!("{}", s);
    }
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/15/real.txt").unwrap()),
            "1457740"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/15/real.txt").unwrap()),
            "1467145"
        );
    }
}
