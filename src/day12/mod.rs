use std::collections::HashMap;

use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("12");
    let input: Vec<Vec<char>> = input.map_while(Result::ok).map(|x| x.chars().collect()).collect();
    let mut vec = vec![vec!['#'; input[0].len() + 2]; input.len() + 2];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            vec[i + 1][j + 1] = input[i][j];
        }
    }
    let mut big_vec = vec![vec!['#'; vec[0].len() * 4]; vec.len() * 4];
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            for k in 0..4 {
                for l in 0..4 {
                    big_vec[i * 4 + k][j * 4 + l] = vec[i][j];
                }
            }
        }
    }

    let mut visited = vec![vec![false; big_vec[0].len()]; big_vec.len()];

    let mut total = 0;
    let mut total2 = 0;
    for i in 4..(big_vec.len() - 4) {
        for j in 4..(big_vec[i].len() - 4) {
            if visited[i][j] {
                continue;
            }

            let mut stack = vec![(i, j)];
            let mut walls = 0;
            let mut plants = 0;
            let mut sides = HashMap::new();
            let plant = big_vec[i][j];
            while let Some((x, y)) = stack.pop() {
                if big_vec[x][y] != plant {
                    walls += 1;
                    sides.insert((x, y), false);
                    continue;
                }
                if visited[x][y] {
                    continue;
                }

                plants += 1;
                visited[x][y] = true;

                stack.push((x - 1, y));
                stack.push((x + 1, y));
                stack.push((x, y - 1));
                stack.push((x, y + 1));
            }
            total += plants * walls / 16 / 4;

            let mut num_sides = 0;
            let keys = sides.keys().cloned().collect::<Vec<(usize, usize)>>();
            for side in keys {
                if sides.get(&side) == Some(&true) {
                    continue;
                }
                let (x, y) = side;
                for directions in [vec![(0, 1), (0, -1)], vec![(1, 0), (-1, 0)]] {
                    let mut worked = false;
                    for direction in directions {
                        let (dx, dy) = direction;
                        let mut next = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                        while sides.get(&next) == Some(&false) {
                            worked = true;
                            sides.insert(next, true);
                            next = ((next.0 as i32 + dx) as usize, (next.1 as i32 + dy) as usize);
                        }
                    }
                    if worked {
                        num_sides += 1;
                    }
                }
            }
            total2 += plants * num_sides / 16;
        }
    }

    (total, total2)
}
