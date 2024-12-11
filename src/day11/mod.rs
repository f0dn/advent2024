use std::collections::HashMap;

use crate::get_input;

pub fn solve() -> (u32, u128) {
    let input = get_input!("11");
    let stones = input
        .map_while(Result::ok)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut cache = HashMap::new();
    let stones2 = stones.clone();
    let mut total = 0;
    for stone in stones {
        total += count(stone, 25, &mut cache);
    }
    let mut total2 = 0;
    for stone in stones2 {
        total2 += count(stone, 75, &mut cache);
    }
    (total as u32, total2)
}

fn count(stone: String, times: u32, cache: &mut HashMap<(String, u32), u128>) -> u128 {
    if times == 0 {
        return 1;
    }
    let key = (stone, times);
    if cache.get(&key).is_some() {
        return *cache.get(&key).unwrap();
    }
    let stone = key.0;
    if stone == "0" {
        let count = count("1".to_string(), times - 1, cache);
        cache.insert((stone.to_string(), times), count);
        count
    } else if stone.len() % 2 == 0 {
        let first_half = &stone[..stone.len() / 2];
        let mut second_half = &stone[stone.len() / 2..];
        second_half = &second_half[second_half
            .find(|c: char| c != '0')
            .unwrap_or(second_half.len() - 1)..];
        let count = count(first_half.to_string(), times - 1, cache)
            + count(second_half.to_string(), times - 1, cache);
        cache.insert((stone.to_string(), times), count);
        count
    } else {
        let parse = stone.parse::<u128>().unwrap();
        let var_name = (parse * 2024).to_string();
        let count = count(var_name, times - 1, cache);
        cache.insert((stone.to_string(), times), count);
        count
    }
}
