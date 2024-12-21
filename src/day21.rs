use rustc_hash::FxHashMap;

pub fn part1(contents: String) -> String {
    let rooms: Vec<_> = contents.lines().collect();

    let (door_map, robot_map) = keypad_maps();

    let mut sum = 0;
    for door_number in rooms {
        sum += count_button_presses_for_door(door_number, &door_map, &robot_map, 2);
    }

    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let rooms: Vec<_> = contents.lines().collect();

    let (door_map, robot_map) = keypad_maps();

    let mut sum = 0;
    for door_number in rooms {
        sum += count_button_presses_for_door(door_number, &door_map, &robot_map, 25);
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

fn count_button_presses_for_door(
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
        // Door robot
        let buttons_to_press = {
            let next_position = door_map[&next_key];
            let door_move = euclidean_move(&current_position, &next_position);

            let start_button_presses =
                find_possible_start_buttons_door(current_position, next_position, door_move);

            current_position = next_position;

            find_robot_keys(robot_map, next_position, door_move, start_button_presses)
        };

        // Middle robots
        let mut memoized = FxHashMap::default();

        let count = expand_keys(
            robot_map,
            start_robot,
            &buttons_to_press,
            &mut memoized,
            times_to_expand,
        );
        supercount += count;
    }

    let num: usize = door_number[0..3].parse().unwrap();

    num * supercount
}

fn find_possible_start_buttons_door(
    current_position: (isize, isize),
    next_position: (isize, isize),
    door_move: (isize, isize),
) -> Vec<char> {
    let mut buttons = vec![];

    if door_move.0 > 0 {
        buttons.push('>');
    }
    if door_move.0 < 0 && !(next_position.0 == 0 && current_position.1 == 3) {
        buttons.push('<');
    }
    if door_move.1 > 0 && !(current_position.0 == 0 && next_position.1 == 3) {
        buttons.push('v');
    }
    if door_move.1 < 0 {
        buttons.push('^');
    }

    buttons
}

fn find_possible_start_buttons_door_robot(
    current_position: &(isize, isize),
    next_position: (isize, isize),
    robot_move: (isize, isize),
) -> Vec<char> {
    let mut buttons = vec![];

    if robot_move.0 > 0 {
        buttons.push('>');
    }
    if robot_move.0 < 0 && !(next_position.0 == 0 && current_position.1 == 0) {
        buttons.push('<');
    }

    // why does order matter here? if vertical is selected first the result differs

    if robot_move.1 > 0 {
        buttons.push('v');
    }
    if robot_move.1 < 0 && !(current_position.0 == 0 && next_position.1 == 0) {
        buttons.push('^');
    }

    buttons
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

    let mut current_robot_position = start_robot;

    let mut s = 0;
    for key in start_keys {
        let (robot_move, start_buttons) = robot_move(robot_map, &mut current_robot_position, *key);
        let key_group =
            find_robot_keys(robot_map, current_robot_position, robot_move, start_buttons);
        s += expand_keys(
            robot_map,
            start_robot,
            &key_group,
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
) -> ((isize, isize), Vec<char>) {
    let next_position = robot_map[&robot_key];
    let robot_move = euclidean_move(current_robot_position, &next_position);

    let start_buttons =
        find_possible_start_buttons_door_robot(current_robot_position, next_position, robot_move);
    *current_robot_position = next_position;
    (robot_move, start_buttons)
}

fn find_robot_keys(
    robot_map: &FxHashMap<char, (isize, isize)>,
    current_position: (isize, isize),
    position_move: (isize, isize),
    start_buttons: Vec<char>,
) -> Vec<char> {
    let next_key = start_key(robot_map, current_position, start_buttons);
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
    possible_start_keys: Vec<char>,
) -> char {
    match possible_start_keys.as_slice() {
        // already at correct key?
        [] => 'A',
        [key] => *key,
        // pick key with shortest euclidean distance
        [key1, key2] => {
            let position1 = robot_map[key1];
            let position2 = robot_map[key2];
            let robot_move1 = euclidean_move(&current_position, &position1);
            let robot_move2 = euclidean_move(&current_position, &position2);

            let cost1 = robot_move1.0 + robot_move1.1;
            let cost2 = robot_move2.0 + robot_move2.1;

            if cost2 >= cost1 {
                *key1
            } else {
                *key2
            }
        }
        _ => unreachable!(),
    }
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

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/21/real.txt").unwrap()),
            "167538833832712"
        );
    }
}
