use std::collections::HashMap;

use crate::get_input;

pub fn solve() -> (u128, u128) {
    let input = get_input!("21");
    let mut input_map = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];
    let mut first_dists =
        vec![
            vec![vec![vec![u128::MAX; input_map[0].len()]; input_map.len()]; input_map[0].len()];
            input_map.len()
        ];
    for i in 0..input_map.len() {
        for j in 0..input_map[i].len() {
            if input_map[i][j] == ' ' {
                continue;
            }
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((i, j));
            first_dists[i][j][i][j] = 1;
            while let Some((x, y)) = queue.pop_front() {
                let dist = first_dists[i][j][x][y];
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0
                        || ny < 0
                        || ny >= input_map[0].len() as i32
                        || nx >= input_map.len() as i32
                    {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if input_map[nx][ny] == ' ' {
                        continue;
                    }
                    if first_dists[i][j][nx][ny] <= dist + 1 {
                        continue;
                    }
                    first_dists[i][j][nx][ny] = dist + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    let mut dir_to_coord = HashMap::new();
    dir_to_coord.insert((-1, 0), (0, 1));
    dir_to_coord.insert((1, 0), (1, 1));
    dir_to_coord.insert((0, -1), (1, 0));
    dir_to_coord.insert((0, 1), (1, 2));
    let mut prev_dists = first_dists.clone();
    let mut second_dists =
        vec![
            vec![vec![vec![u128::MAX; input_map[0].len()]; input_map.len()]; input_map[0].len()];
            input_map.len()
        ];
    let mut second_dists1 = Vec::new();
    for n in 0..24 {
        second_dists = vec![
            vec![
                vec![vec![u128::MAX; input_map[0].len()]; input_map.len()];
                input_map[0].len()
            ];
            input_map.len()
        ];
        for i in 0..input_map.len() {
            for j in 0..input_map[i].len() {
                if input_map[i][j] == ' ' {
                    continue;
                }

                for x in 0..input_map.len() {
                    for y in 0..input_map[x].len() {
                        if input_map[x][y] == ' ' {
                            continue;
                        }
                        let c = explore(
                            i,
                            j,
                            0,
                            2,
                            x,
                            y,
                            &input_map,
                            &mut vec![vec![false; input_map[0].len()]; input_map.len()],
                            &prev_dists,
                            &dir_to_coord,
                        );
                        second_dists[i][j][x][y] = c;
                    }
                }
            }
        }
        prev_dists = second_dists.clone();
        if n == 0 {
            second_dists1 = second_dists.clone();
        }
    }
    let keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
    let mut digit_to_coord = HashMap::new();
    for i in 0..keypad.len() {
        for j in 0..keypad[i].len() {
            if keypad[i][j] == ' ' {
                continue;
            }
            digit_to_coord.insert(keypad[i][j], (i, j));
        }
    }
    let mut keypad_dists1 =
        vec![
            vec![vec![vec![u128::MAX; keypad[0].len()]; keypad.len()]; keypad[0].len()];
            keypad.len()
        ];
    let mut keypad_dists2 =
        vec![
            vec![vec![vec![u128::MAX; keypad[0].len()]; keypad.len()]; keypad[0].len()];
            keypad.len()
        ];
    for i in 0..keypad.len() {
        for j in 0..keypad[i].len() {
            if keypad[i][j] == ' ' {
                continue;
            }
            for xf in 0..keypad.len() {
                for yf in 0..keypad[xf].len() {
                    if keypad[xf][yf] == ' ' {
                        continue;
                    }
                    let mut visited = vec![vec![false; keypad[0].len()]; keypad.len()];
                    keypad_dists1[i][j][xf][yf] = explore(
                        i,
                        j,
                        0,
                        2,
                        xf,
                        yf,
                        &keypad,
                        &mut visited,
                        &second_dists1,
                        &dir_to_coord,
                    );
                    keypad_dists2[i][j][xf][yf] = explore(
                        i,
                        j,
                        0,
                        2,
                        xf,
                        yf,
                        &keypad,
                        &mut visited,
                        &second_dists,
                        &dir_to_coord,
                    );
                }
            }
        }
    }

    let mut total1 = 0;
    let mut total2 = 0;
    for line in input.map_while(Result::ok) {
        let mut min_dist1 = 0;
        let mut min_dist2 = 0;
        let mut prev_x = 3;
        let mut prev_y = 2;
        let num = line[0..3].parse::<u128>().unwrap();
        for c in line.chars() {
            let (x, y) = digit_to_coord[&c];
            min_dist1 += keypad_dists1[prev_x][prev_y][x][y];
            min_dist2 += keypad_dists2[prev_x][prev_y][x][y];
            prev_x = x;
            prev_y = y;
        }
        total1 += min_dist1 * num;
        total2 += min_dist2 * num;
    }
    (total1, total2)
}

fn explore(
    x: usize,
    y: usize,
    x1: usize,
    y1: usize,
    tx: usize,
    ty: usize,
    input_map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    second_dists: &Vec<Vec<Vec<Vec<u128>>>>,
    dir_to_coord: &HashMap<(i32, i32), (usize, usize)>,
) -> u128 {
    if x == tx && y == ty {
        return second_dists[x1][y1][0][2];
    }

    if visited[x][y] {
        return u128::MAX;
    }
    visited[x][y] = true;

    let mut min_dist = u128::MAX;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || ny >= input_map[0].len() as i32 || nx >= input_map.len() as i32 {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        let (cx, cy) = dir_to_coord[&(*dx, *dy)];
        if input_map[nx][ny] == ' ' {
            continue;
        }
        let dist = explore(
            nx,
            ny,
            cx,
            cy,
            tx,
            ty,
            input_map,
            visited,
            second_dists,
            dir_to_coord,
        );
        if dist == u128::MAX {
            continue;
        }
        min_dist = min_dist.min(dist + second_dists[x1][y1][cx][cy]);
    }

    visited[x][y] = false;
    min_dist
}
