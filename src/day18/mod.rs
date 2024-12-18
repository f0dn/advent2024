use crate::get_input;

pub fn solve() -> (u32, String) {
    let input = get_input!("18");
    let size = 71;
    let mut map = vec![vec![false; size]; size];
    let mut bytes = Vec::new();
    for line in input.map_while(Result::ok) {
        let mut split = line.split(",");
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        bytes.push((x, y));
    }

    for (x, y) in bytes.iter().take(1024) {
        map[*y][*x] = true;
    }

    let mut stack = vec![(0, 0)];
    let mut dists = vec![vec![usize::MAX; size]; size];
    dists[0][0] = 0;
    while let Some((x, y)) = stack.pop() {
        if x == size - 1 && y == size - 1 {
            continue;
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= size as i32 || ny >= size as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if map[ny][nx] {
                continue;
            }
            if dists[ny][nx] > dists[y][x] + 1 {
                dists[ny][nx] = dists[y][x] + 1;
                stack.push((nx, ny));
            }
        }
    }

    let mut i = 1024;
    let mut x;
    let mut y;
    loop {
        (x, y) = bytes[i];
        map[y][x] = true;
        let mut found = false;
        let mut visited = vec![vec![false; size]; size];
        stack = vec![(0, 0)];
        while let Some((x, y)) = stack.pop() {
            if x == size - 1 && y == size - 1 {
                found = true;
                break;
            }
            visited[y][x] = true;
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || ny < 0 || nx >= size as i32 || ny >= size as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if map[ny][nx] || visited[ny][nx] {
                    continue;
                }
                stack.push((nx, ny));
            }
        }
        if !found {
            break;
        }
        i += 1;
    }
    (
        dists[size - 1][size - 1] as u32,
        x.to_string() + "," + &y.to_string(),
    )
}
