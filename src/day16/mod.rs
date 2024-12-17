use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("16");
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in input.map_while(Result::ok).enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(true),
                '.' => row.push(false),
                'S' => {
                    row.push(false);
                    start = (i, j);
                }
                'E' => {
                    row.push(false);
                    end = (i, j);
                }
                _ => (),
            }
        }
        map.push(row);
    }

    let og_map = map.clone();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut dist = vec![vec![vec![usize::MAX; 4]; map[0].len()]; map.len()];
    let mut queue = std::collections::VecDeque::new();
    for i in 0..4 {
        dist[start.0][start.1][i] = 0;
    }
    queue.push_back((start.0, start.1, 0, 0));
    while let Some((x, y, d, dir)) = queue.pop_front() {
        if (x, y) == end {
            continue;
        }

        map[x][y] = true;

        for ndir in 0..4 {
            let nx = x as i32 + dirs[ndir].0;
            let ny = y as i32 + dirs[ndir].1;
            if og_map[nx as usize][ny as usize] {
                continue;
            }
            let nd = d + if dir == ndir { 1 } else { 1001 };
            if nd >= dist[nx as usize][ny as usize][ndir] {
                continue;
            }
            if ndir == dir {
                queue.push_front((nx as usize, ny as usize, nd, ndir));
            } else {
                queue.push_back((nx as usize, ny as usize, nd, ndir));
            }
            dist[nx as usize][ny as usize][ndir] = nd.min(dist[nx as usize][ny as usize][ndir]);
        }
    }

    let mut on_best = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
    let mut min = 0;
    for i in 0..4 {
        if dist[end.0][end.1][i] < dist[end.0][end.1][min] {
            min = i;
        }
    }
    let mut stack = vec![(end.0, end.1, min)];

    while let Some((x, y, dir)) = stack.pop() {
        if on_best[x][y][dir] {
            continue;
        }
        on_best[x][y][dir] = true;
        if (x, y) == start {
            continue;
        }
        let nx = x as i32 - dirs[dir].0;
        let ny = y as i32 - dirs[dir].1;
        if og_map[nx as usize][ny as usize] {
            continue;
        }
        for ndir in 0..4 {
            if dist[nx as usize][ny as usize][ndir] == usize::MAX {
                continue;
            }
            let nd = dist[x][y][dir] as isize - if dir == ndir { 1 } else { 1001 };
            if nd < 0 {
                continue;
            }
            if dist[nx as usize][ny as usize][ndir] == nd as usize {
                stack.push((nx as usize, ny as usize, ndir));
            }
        }
    }

    let num = on_best
        .iter()
        .flatten()
        .filter(|&x| x.contains(&true))
        .count();

    let ans = dist[end.0][end.1].iter().min().unwrap();

    (*ans as u32, num as u32)
}
