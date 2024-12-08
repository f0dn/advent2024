use std::collections::{HashMap, HashSet};

use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("08");
    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    for (i, line) in input.map_while(Result::ok).enumerate() {
        width = line.chars().count();
        height = i + 1;
        for (j, char) in line.chars().enumerate() {
            if char != '.' {
                if let std::collections::hash_map::Entry::Vacant(e) = nodes.entry(char) {
                    e.insert(vec![(i, j)]);
                } else {
                    nodes.get_mut(&char).unwrap().push((i, j));
                }
            }
        }
    }


    let mut positions = HashSet::new();
    let mut positions2 = HashSet::new();
    for node in nodes.values() {
        for (x, y) in node {
            for (x1, y1) in node {
                if x == x1 && y == y1 {
                    continue;
                }
                let dx = *x as i32 - *x1 as i32;
                let dy = *y as i32 - *y1 as i32;
                let mut antinode = (*x as i32 + dx, *y as i32 + dy);
                if is_within_bounds(antinode.0, antinode.1, width, height) {
                    positions.insert(antinode);
                }

                positions2.insert((*x as i32, *y as i32));
                while is_within_bounds(antinode.0, antinode.1, width, height) {
                    positions2.insert(antinode);
                    antinode = (antinode.0 + dx, antinode.1 + dy);
                }
            }
        }
    }

    (positions.len() as u32, positions2.len() as u32)
}

fn is_within_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x < width as i32 && y < height as i32 && x >= 0 && y >= 0
}
