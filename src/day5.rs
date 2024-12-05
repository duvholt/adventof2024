use std::collections::{HashMap, HashSet};

pub fn part1(contents: String) -> String {
    let mut parts = contents.split("\n\n");
    let mut before_order_map: HashMap<u64, HashSet<u64>> = HashMap::new();
    for order in parts.next().unwrap().lines() {
        dbg!(order);
        let mut o = order.split("|");
        let before = o.next().unwrap().parse().unwrap();
        let after = o.next().unwrap().parse().unwrap();
        let a = before_order_map.entry(after).or_default();
        a.insert(before);
    }
    let mut sum = 0;
    for update in parts.next().unwrap().lines() {
        let update: Vec<u64> = update.split(",").map(|u| u.parse().unwrap()).collect();
        let mut failed = false;
        for (i, u) in update.iter().enumerate() {
            if let Some(before) = before_order_map.get(u) {
                let not_before = update.iter().skip(i + 1).all(|u| !before.contains(u));
                if !not_before {
                    println!("Fail!");
                    failed = true;
                    break;
                }
            }
        }
        if !failed {
            let middle = update[update.len() / 2];
            dbg!(update, middle);
            sum += middle;
        }
    }
    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let mut parts = contents.split("\n\n");
    let mut before_order_map: HashMap<u64, HashSet<u64>> = HashMap::new();
    for order in parts.next().unwrap().lines() {
        dbg!(order);
        let mut o = order.split("|");
        let before = o.next().unwrap().parse().unwrap();
        let after = o.next().unwrap().parse().unwrap();
        let a = before_order_map.entry(after).or_default();
        a.insert(before);
    }
    let mut sum = 0;
    for update in parts.next().unwrap().lines() {
        let update: Vec<u64> = update.split(",").map(|u| u.parse().unwrap()).collect();
        let mut failed = false;
        for (i, u) in update.iter().enumerate() {
            if let Some(before) = before_order_map.get(u) {
                let not_before = update.iter().skip(i + 1).all(|u| !before.contains(u));
                if !not_before {
                    println!("Fail!");
                    failed = true;
                    break;
                }
            }
        }
        if failed {
            dbg!(&update);
            let new = fix_order(&update, &before_order_map);

            let middle = new[new.len() / 2];
            sum += middle;
        }
    }
    sum.to_string()
}

fn fix_order(update: &[u64], before_order_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    let mut new = update.to_vec();
    let mut i = 0;
    while i < update.len() {
        // dbg!(&new);
        let u = new[i];
        match before_order_map.get(&u) {
            Some(before) => {
                let first = new
                    .iter()
                    .enumerate()
                    .filter_map(|(ui, u)| {
                        if before.contains(u) && ui > i {
                            Some(ui)
                        } else {
                            None
                        }
                    })
                    .max();
                if let Some(f) = first {
                    // println!("Moving!");
                    let n = new.remove(i);
                    // dbg!(f, n, i);
                    new.insert(f, n);
                } else {
                    i += 1;
                }
            }
            None => {
                i += 1;
            }
        }
    }
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
