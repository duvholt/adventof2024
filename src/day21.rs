use std::collections::HashMap;

#[derive(Debug)]
enum MoveDirection {
    Vertical,
    Horizontal,
}

pub fn part1(contents: String) -> String {
    let rooms: Vec<_> = contents.lines().collect();

    let door_keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['-', '0', 'A'],
    ];
    let robot_keypad = vec![vec!['-', '^', 'A'], vec!['<', 'v', '>']];
    // - is illegal

    let mut door_map: HashMap<char, Position> = HashMap::default();
    for y in 0..door_keypad.len() {
        for x in 0..door_keypad[0].len() {
            door_map.insert(door_keypad[y][x], (x as isize, y as isize));
        }
    }

    let mut robot_map: HashMap<char, Position> = HashMap::default();
    for y in 0..robot_keypad.len() {
        for x in 0..robot_keypad[0].len() {
            robot_map.insert(robot_keypad[y][x], (x as isize, y as isize));
        }
    }

    let mut sum = 0;
    for door_number in rooms {
        sum += expand_input(door_number, &door_map, &robot_map);
    }

    sum.to_string()
}

fn expand_input(
    door_number: &str,
    door_map: &HashMap<char, (isize, isize)>,
    robot_map: &HashMap<char, (isize, isize)>,
) -> usize {
    let start_door = (2, 3);
    let start_robot = (2, 0);
    let seq: Vec<_> = door_number.chars().collect();

    let mut current_position = start_door;
    let mut current_robot1_position = start_robot;
    let mut current_robot2_position = start_robot;

    let mut s = String::new();

    for next_key in seq {
        // dbg!(next_key);
        // find door keypad move

        let (door_move, first_door_moves) = {
            let next_position = door_map[&next_key];
            let door_move = euclidean_move(&current_position, &next_position);

            let first_door_moves =
                find_first_moves_door(&current_position, next_position, door_move);

            current_position = next_position;
            (door_move, first_door_moves)
        };

        // possible optimization: is robot always at A after finishing previous robots seq?

        // dbg!(&door_move, &first_door_moves);
        {
            // Robot 1
            let keys_to_press1 =
                find_robot_keys(robot_map, current_position, door_move, first_door_moves);

            // println!("{:?}", &keys_to_press1);

            // Robot 2
            let mut keys_to_press2 = vec![];
            for next_robot_key1 in keys_to_press1 {
                let (robot1_move, robot1_first_moves) =
                    robot_move(robot_map, &mut current_robot1_position, next_robot_key1);
                keys_to_press2.extend(find_robot_keys(
                    robot_map,
                    current_robot1_position,
                    robot1_move,
                    robot1_first_moves,
                ));
            }

            // println!("{:?}", &keys_to_press2);

            // Robot 3
            let mut keys_to_press3 = vec![];
            for next_robot_key2 in keys_to_press2 {
                let (robot2_move, robot2_first_moves) =
                    robot_move(robot_map, &mut current_robot2_position, next_robot_key2);
                keys_to_press3.extend(find_robot_keys(
                    robot_map,
                    current_robot2_position,
                    robot2_move,
                    robot2_first_moves,
                ));
            }

            // println!("{:?}", &keys_to_press3);

            for k in keys_to_press3 {
                s.push(k);
            }
        }
    }

    let num: usize = door_number[0..3].parse().unwrap();

    // dbg!(num, s.len());
    println!("{} {}", door_number, s);
    num * s.len()
}

fn robot_move(
    robot_map: &HashMap<char, (isize, isize)>,
    current_robot_position: &mut (isize, isize),
    robot_key: char,
) -> ((isize, isize), Vec<MoveDirection>) {
    let next_position = robot_map[&robot_key];
    let robot_move = euclidean_move(current_robot_position, &next_position);

    let first_moves = find_first_moves_robot(current_robot_position, next_position, robot_move);
    *current_robot_position = next_position;
    (robot_move, first_moves)
}

fn find_first_moves_door(
    current_position: &(isize, isize),
    next_position: (isize, isize),
    door_move: (isize, isize),
) -> Vec<MoveDirection> {
    let mut first_door_moves = vec![];
    if next_position.0 == 0 && current_position.1 == 3 {
        // avoid empty spot
        first_door_moves.push(MoveDirection::Vertical);
    } else if current_position.0 == 0 && next_position.1 == 3 {
        // avoid empty spot
        first_door_moves.push(MoveDirection::Horizontal);
    } else {
        if door_move.0 != 0 {
            first_door_moves.push(MoveDirection::Horizontal);
        }
        if door_move.1 != 0 {
            first_door_moves.push(MoveDirection::Vertical);
        }
    };
    first_door_moves
}

fn find_first_moves_robot(
    current_position: &(isize, isize),
    next_position: (isize, isize),
    robot_move: (isize, isize),
) -> Vec<MoveDirection> {
    let mut first_moves = vec![];
    if next_position.0 == 0 && current_position.1 == 0 {
        // avoid empty spot
        first_moves.push(MoveDirection::Vertical);
    } else if current_position.0 == 0 && next_position.1 == 0 {
        // avoid empty spot
        first_moves.push(MoveDirection::Horizontal);
    } else {
        if robot_move.1 != 0 {
            first_moves.push(MoveDirection::Vertical);
        }
        if robot_move.0 != 0 {
            first_moves.push(MoveDirection::Horizontal);
        }
    };
    first_moves
}

fn find_robot_keys(
    robot_map: &HashMap<char, (isize, isize)>,
    current_position: (isize, isize),
    position_move: (isize, isize),
    first_moves: Vec<MoveDirection>,
) -> Vec<char> {
    let mut possible_start_keys = vec![];
    if first_moves.is_empty() {
        // already at correct key?
        possible_start_keys.push('A');
    }
    for first_door_move in first_moves {
        let key = match first_door_move {
            MoveDirection::Vertical if position_move.1 > 0 => 'v',
            MoveDirection::Vertical if position_move.1 < 0 => '^',
            MoveDirection::Horizontal if position_move.0 > 0 => '>',
            MoveDirection::Horizontal if position_move.0 < 0 => '<',
            _ => panic!("wtf"),
        };
        possible_start_keys.push(key);
    }
    // pick key with shortest euclidean distance
    let next_key = possible_start_keys
        .into_iter()
        .min_by_key(|key| {
            let position = robot_map[key];
            let robot_move = euclidean_move(&current_position, &position);
            // penalize changing directions
            robot_move.0 + robot_move.1
        })
        .unwrap();
    // calculate key presses
    let mut single_keys = vec![next_key];
    match next_key {
        '^' | 'v' => {
            if position_move.0 > 0 {
                single_keys.push('>');
            }
            if position_move.0 < 0 {
                single_keys.push('<');
            }
        }
        '<' | '>' => {
            if position_move.1 > 0 {
                single_keys.push('v');
            }
            if position_move.1 < 0 {
                single_keys.push('^');
            }
        }
        _ => {}
    }
    let mut keys_to_press = vec![];
    for single_key in single_keys {
        match single_key {
            '^' | 'v' => {
                for _ in 0..position_move.1.abs() {
                    keys_to_press.push(single_key);
                }
            }
            '<' | '>' => {
                for _ in 0..position_move.0.abs() {
                    keys_to_press.push(single_key);
                }
            }
            _ => {}
        }
    }
    // always ends with A
    keys_to_press.push('A');
    keys_to_press
}

type Position = (isize, isize);
type Move = (isize, isize);

fn euclidean_move(current_position: &Position, next_position: &Position) -> Move {
    let (x1, y1) = current_position;
    let (x2, y2) = next_position;
    (x2 - x1, y2 - y1)
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
            part1(fs::read_to_string("./input/21/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/21/real.txt").unwrap()),
            "example2"
        );
    }
}
