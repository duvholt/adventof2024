use rustc_hash::FxHashMap;

#[derive(Debug)]
enum MoveDirection {
    Vertical,
    Horizontal,
}

pub fn part1(contents: String) -> String {
    let rooms: Vec<_> = contents.lines().collect();

    let (door_map, robot_map) = keypad_maps();

    let mut sum = 0;
    for door_number in rooms {
        sum += expand_input(door_number, &door_map, &robot_map, 2);
    }

    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let rooms: Vec<_> = contents.lines().collect();

    let (door_map, robot_map) = keypad_maps();

    let mut sum = 0;
    for door_number in rooms {
        sum += expand_input(door_number, &door_map, &robot_map, 25);
    }

    sum.to_string()
}

fn keypad_maps() -> (
    FxHashMap<char, (isize, isize)>,
    FxHashMap<char, (isize, isize)>,
) {
    let door_keypad = [
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['-', '0', 'A'],
    ];
    let robot_keypad = [vec!['-', '^', 'A'], vec!['<', 'v', '>']];
    // - is illegal

    let mut door_map: FxHashMap<char, Position> = FxHashMap::default();
    for y in 0..door_keypad.len() {
        for x in 0..door_keypad[0].len() {
            door_map.insert(door_keypad[y][x], (x as isize, y as isize));
        }
    }

    let mut robot_map: FxHashMap<char, Position> = FxHashMap::default();
    for y in 0..robot_keypad.len() {
        for x in 0..robot_keypad[0].len() {
            robot_map.insert(robot_keypad[y][x], (x as isize, y as isize));
        }
    }
    (door_map, robot_map)
}

fn expand_input(
    door_number: &str,
    door_map: &FxHashMap<char, (isize, isize)>,
    robot_map: &FxHashMap<char, (isize, isize)>,
    times_to_expand: usize,
) -> usize {
    let start_door = (2, 3);
    let start_robot = (2, 0);
    let seq: Vec<_> = door_number.chars().collect();

    let mut current_position = start_door;

    let mut supercount = 0;

    for next_key in seq {
        // find door keypad move

        let (door_move, first_door_moves) = {
            let next_position = door_map[&next_key];
            let door_move = euclidean_move(&current_position, &next_position);

            let first_door_moves =
                find_first_moves_door(&current_position, next_position, door_move);

            current_position = next_position;
            (door_move, first_door_moves)
        };

        {
            // Robot 1
            let keys_to_press1 =
                find_robot_keys(robot_map, current_position, door_move, first_door_moves);

            // Middle robots
            let expanded = keys_to_press1;
            let mut memoized = FxHashMap::default();

            let count = expand_keys(
                robot_map,
                start_robot,
                &expanded,
                &mut memoized,
                times_to_expand,
            );
            supercount += count;
        }
    }

    let num: usize = door_number[0..3].parse().unwrap();

    // dbg!(num, s.len());
    println!("{} {}", door_number, supercount);
    num * supercount
}

fn expand_keys(
    robot_map: &FxHashMap<char, (isize, isize)>,
    start_robot: (isize, isize),
    start_keys: &Vec<char>,
    memoized: &mut FxHashMap<(Vec<char>, usize), usize>,
    times_to_expand: usize,
) -> usize {
    if times_to_expand == 0 {
        return start_keys.len();
    }
    if let Some(count) = memoized.get(&(start_keys.clone(), times_to_expand)) {
        return *count;
    }

    let mut current_robot1_position = start_robot;
    let mut key_group = vec![];
    for key in start_keys {
        let (robot_move, robot_first_moves) =
            robot_move(robot_map, &mut current_robot1_position, *key);
        key_group.push(find_robot_keys(
            robot_map,
            current_robot1_position,
            robot_move,
            robot_first_moves,
        ));
    }

    let mut s = 0;
    for key_group in key_group.iter() {
        s += expand_keys(
            robot_map,
            start_robot,
            key_group,
            memoized,
            times_to_expand - 1,
        );
    }
    memoized.insert((start_keys.clone(), times_to_expand), s);
    s
}

fn robot_move(
    robot_map: &FxHashMap<char, (isize, isize)>,
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
        // why does order matter here? if vertical is selected first the result differs
        if robot_move.0 != 0 {
            first_moves.push(MoveDirection::Horizontal);
        }
        if robot_move.1 != 0 {
            first_moves.push(MoveDirection::Vertical);
        }
    };
    first_moves
}

fn find_robot_keys(
    robot_map: &FxHashMap<char, (isize, isize)>,
    current_position: (isize, isize),
    position_move: (isize, isize),
    first_moves: Vec<MoveDirection>,
) -> Vec<char> {
    let next_key = start_key(robot_map, current_position, position_move, first_moves);
    let single_keys = find_key_order(position_move, next_key);

    expand_single_keys(position_move, single_keys)
}

fn expand_single_keys(position_move: (isize, isize), single_keys: Vec<char>) -> Vec<char> {
    let mut keys_to_press =
        Vec::with_capacity(position_move.0.unsigned_abs() + position_move.1.unsigned_abs());
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

fn find_key_order(position_move: (isize, isize), next_key: char) -> Vec<char> {
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
    single_keys
}

fn start_key(
    robot_map: &std::collections::HashMap<char, (isize, isize), rustc_hash::FxBuildHasher>,
    current_position: (isize, isize),
    position_move: (isize, isize),
    first_moves: Vec<MoveDirection>,
) -> char {
    if first_moves.is_empty() {
        // already at correct key?
        return 'A';
    }
    let possible_start_keys =
        first_moves
            .into_iter()
            .map(|first_door_move| match first_door_move {
                MoveDirection::Vertical if position_move.1 > 0 => 'v',
                MoveDirection::Vertical if position_move.1 < 0 => '^',
                MoveDirection::Horizontal if position_move.0 > 0 => '>',
                MoveDirection::Horizontal if position_move.0 < 0 => '<',
                _ => unreachable!(),
            });
    // pick key with shortest euclidean distance

    possible_start_keys
        .min_by_key(|key| {
            let position = robot_map[key];
            let robot_move = euclidean_move(&current_position, &position);
            robot_move.0 + robot_move.1
        })
        .unwrap()
}

type Position = (isize, isize);
type Move = (isize, isize);

fn euclidean_move(current_position: &Position, next_position: &Position) -> Move {
    let (x1, y1) = current_position;
    let (x2, y2) = next_position;
    (x2 - x1, y2 - y1)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/21/real.txt").unwrap()),
            "136780"
        );
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/21/real.txt").unwrap()),
            "example2"
        );
    }
}
