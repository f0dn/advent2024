use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("20");
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (x, line) in input.map_while(Result::ok).enumerate() {
        let mut row = Vec::new();
        for (y, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(false),
                '#' => row.push(true),
                'S' => {
                    row.push(false);
                    start = (x as i32, y as i32);
                }
                'E' => {
                    row.push(false);
                    end = (x as i32, y as i32);
                }
                _ => unreachable!(),
            }
        }
        map.push(row);
    }

    let mut dists = vec![vec![u32::MAX; map[0].len()]; map.len()];
    search(&map, start, end, &mut dists);
    let possible1 = find_num_poss(&map, &dists, 2);
    let possible2 = find_num_poss(&map, &dists, 20);

    (possible1, possible2)
}

fn search(
    map: &[Vec<bool>],
    start: (i32, i32),
    end: (i32, i32),
    dists: &mut [Vec<u32>],
) -> u32 {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start, 0));
    dists[start.0 as usize][start.1 as usize] = 0;

    while let Some(((x, y), dist)) = queue.pop_front() {
        if (x, y) == end {
            return dist;
        }

        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= map.len() as i32 || ny >= map[0].len() as i32 {
                continue;
            }
            if map[nx as usize][ny as usize] {
                continue;
            }
            if dists[nx as usize][ny as usize] <= dist + 1 {
                continue;
            }
            dists[nx as usize][ny as usize] = dist + 1;
            queue.push_back(((nx, ny), dist + 1));
        }
    }

    unreachable!()
}

fn find_num_poss(map: &[Vec<bool>], dists: &[Vec<u32>], num_picos: i32) -> u32 {
    let mut possible = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] {
                continue;
            }
            let init_dist = dists[i][j];
            let min_x = 0.max(i as i32 - num_picos);
            let max_x = (map.len() as i32 - 1).min(i as i32 + num_picos);
            let min_y = 0.max(j as i32 - num_picos);
            let max_y = (map[0].len() as i32 - 1).min(j as i32 + num_picos);
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    if !map[x as usize][y as usize] {
                        let d = (x - i as i32).abs() + (y - j as i32).abs();
                        if d > num_picos {
                            continue;
                        }
                        if dists[x as usize][y as usize] >= d as u32 + init_dist + 100 {
                            possible += 1;
                        }
                    }
                }
            }
        }
    }
    possible
}
