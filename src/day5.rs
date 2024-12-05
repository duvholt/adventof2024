use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn part1(contents: String) -> String {
    let mut parts = contents.split("\n\n");
    let before_order_map = parse_ordering(&mut parts);
    let mut sum = 0;
    for update in parts.next().unwrap().lines() {
        let update = parse_update(update);
        let failed = is_ordered(&update, &before_order_map);
        if !failed {
            let middle = update[update.len() / 2];
            sum += middle;
        }
    }
    sum.to_string()
}

fn is_ordered(update: &[u64], before_order_map: &HashMap<u64, HashSet<u64>>) -> bool {
    let mut failed = false;
    for (i, u) in update.iter().enumerate() {
        if let Some(before) = before_order_map.get(u) {
            let not_before = update.iter().skip(i + 1).all(|u| !before.contains(u));
            if !not_before {
                failed = true;
                break;
            }
        }
    }
    failed
}

fn parse_update(update: &str) -> Vec<u64> {
    let update: Vec<u64> = update.split(",").map(|u| u.parse().unwrap()).collect();
    update
}

fn parse_ordering(parts: &mut std::str::Split<'_, &str>) -> HashMap<u64, HashSet<u64>> {
    let mut before_order_map: HashMap<u64, HashSet<u64>> = HashMap::new();
    for order in parts.next().unwrap().lines() {
        let mut o = order.split("|");
        let before = o.next().unwrap().parse().unwrap();
        let after = o.next().unwrap().parse().unwrap();
        let a = before_order_map.entry(after).or_default();
        a.insert(before);
    }
    before_order_map
}

pub fn part2(contents: String) -> String {
    let mut parts = contents.split("\n\n");
    let before_order_map = parse_ordering(&mut parts);
    let mut sum = 0;
    for update in parts.next().unwrap().lines() {
        let update = parse_update(update);
        let failed = is_ordered(&update, &before_order_map);
        if failed {
            let new = sort_update(&update, &before_order_map);

            let middle = new[new.len() / 2];
            sum += middle;
        }
    }
    sum.to_string()
}

fn sort_update(update: &[u64], before_order_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    let mut new = update.to_vec();
    new.sort_by(|a, b| {
        if a == b {
            return Ordering::Equal;
        }
        match before_order_map.get(a) {
            Some(before) => {
                if before.contains(b) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            None => Ordering::Less,
        }
    });
    new
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/5/real.txt").unwrap()),
            "4281"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/5/real.txt").unwrap()),
            "5466"
        );
    }
}
