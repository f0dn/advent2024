use std::collections::HashSet;

use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("10");
    let map = input
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut total = 0;
    let mut total2 = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                let count = count_paths(&map, i, j);
                total += count.0;
                total2 += count.1;
            }
        }
    }
    (total, total2)
}

fn count_paths(map: &[Vec<u32>], i: usize, j: usize) -> (u32, u32) {
    let mut stack = vec![(i, j)];
    let mut found = HashSet::new();
    let mut total = 0;
    while let Some((i, j)) = stack.pop() {
        if map[i][j] == 9 {
            found.insert((i, j));
            total += 1;
            continue;
        }
        if i > 0 && map[i - 1][j] == map[i][j] + 1 {
            stack.push((i - 1, j));
        }
        if i < map.len() - 1 && map[i + 1][j] == map[i][j] + 1{
            stack.push((i + 1, j));
        }
        if j > 0 && map[i][j - 1] == map[i][j] + 1{
            stack.push((i, j - 1));
        }
        if j < map[i].len() - 1 && map[i][j + 1] == map[i][j] + 1{
            stack.push((i, j + 1));
        }
    }
    (found.len() as u32, total)
}
