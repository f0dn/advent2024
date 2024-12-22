use std::collections::HashMap;

use crate::get_input;

pub fn solve() -> (u128, u128) {
    let input = get_input!("22");
    let mut total = 0;
    let mut values: HashMap<Vec<i32>, (u128, bool)> = HashMap::new();
    for line in input.flatten() {
        let mut secret = line.parse::<u128>().unwrap();
        let mut diffs = Vec::new();
        for _ in 0..2000 {
            let next = calculate_next(secret);
            if diffs.len() < 4 {
                diffs.push((next % 10) as i32 - (secret % 10) as i32);
            } else {
                diffs.remove(0);
                diffs.push((next % 10) as i32 - (secret % 10) as i32);
            }
            if diffs.len() == 4 {
                if values.contains_key(&diffs) {
                    if !values[&diffs].1 {
                        values.insert(diffs.clone(), (values[&diffs].0 + next % 10, true));
                    }
                } else {
                    values.insert(diffs.clone(), (next % 10, true));
                }
            }
            secret = next;
        }
        for (_, value) in values.iter_mut() {
            if value.1 {
                value.1 = false;
            }
        }
        total += secret;
    }
    let mut max = 0;
    let mut best_diffs = Vec::new();
    for (diffs, (value, _)) in values {
        if value > max {
            max = value;
            best_diffs = diffs;
        }
    }
    println!("{:?}", best_diffs);
    (total, max)
}

fn calculate_next(mut secret: u128) -> u128 {
    let result = secret * 64;
    secret ^= result;
    secret %= 16777216;
    let result = secret / 32;
    secret ^= result;
    secret %= 16777216;
    let result = secret * 2048;
    secret ^= result;
    secret %= 16777216;
    secret
}
