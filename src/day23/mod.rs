use std::collections::{HashMap, HashSet};

use crate::get_input;

pub fn solve() -> (u32, String) {
    let input = get_input!("23");
    let mut connections: HashMap<(char, char), HashSet<(char, char)>> = HashMap::new();
    for line in input.map_while(Result::ok) {
        let bytes = line.as_bytes();
        let c1 = bytes[0] as char;
        let c2 = bytes[1] as char;
        let c3 = bytes[3] as char;
        let c4 = bytes[4] as char;
        let first = (c1, c2);
        let second = (c3, c4);

        if let std::collections::hash_map::Entry::Vacant(e) = connections.entry(first) {
            e.insert(HashSet::from([second]));
        } else {
            connections.get_mut(&first).unwrap().insert(second);
        }
        if let std::collections::hash_map::Entry::Vacant(e) = connections.entry(second) {
            e.insert(HashSet::from([first]));
        } else {
            connections.get_mut(&second).unwrap().insert(first);
        }
    }

    let mut triples_found = HashSet::new();
    for (k, v) in connections.iter() {
        if k.0 != 't' {
            continue;
        }
        for c in v.iter() {
            for c2 in connections.get(c).unwrap().iter() {
                if c2 == k {
                    continue;
                }
                if connections.get(c2).unwrap().contains(k) {
                    let mut v = vec![k, c, c2];
                    v.sort();
                    triples_found.insert(v);
                }
            }
        }
    }

    let mut fully_connected = HashSet::new();
    let mut stack = connections
        .keys()
        .map(|k| (HashSet::from([k]), k))
        .collect::<Vec<_>>();
    while let Some((group, last)) = stack.pop() {
        let mut v = group.iter().collect::<Vec<_>>();
        v.sort();
        let key = v
            .iter()
            .map(|k| format!("{}{}", k.0, k.1))
            .collect::<Vec<_>>()
            .join(",");
        if fully_connected.contains(&key) {
            continue;
        }
        fully_connected.insert(key);
        'outer: for c in connections.get(last).unwrap() {
            if !group.contains(c) {
                for c2 in group.iter() {
                    if !connections.get(c).unwrap().contains(c2) {
                        continue 'outer;
                    }
                }
                let mut new_group = group.clone();
                new_group.insert(c);
                stack.push((new_group, c));
            }
        }
    }

    let mut max_size = 0;
    let mut max_group = String::new();
    for group in fully_connected.iter() {
        if group.len() > max_size {
            max_size = group.len();
            max_group = group.to_string();
        }
    }

    let total = triples_found.len() as u32;

    (total, max_group)
}
