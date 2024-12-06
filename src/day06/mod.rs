use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("06");
    let mut map = Vec::new();
    let mut visited = Vec::new();
    let mut guard = (0, 0);
    let mut dir: (i32, i32) = (-1, 0);
    for line in input.map_while(Result::ok) {
        let mut row = Vec::new();
        for c in line.chars() {
            if c == '#' {
                row.push(true);
            } else {
                row.push(false);
            }
            if c == '^' {
                guard = (map.len(), row.len() - 1);
            }
        }
        visited.push(vec![false; row.len()]);
        map.push(row);
    }

    let original_dir = dir;
    let original_guard = guard;

    visited[guard.0][guard.1] = true;

    loop {
        if guard.0 as i32 + dir.0 >= map.len() as i32
            || guard.1 as i32 + dir.1 >= map[0].len() as i32
            || (guard.0 as i32 + dir.0) < 0
            || (guard.1 as i32 + dir.1) < 0
        {
            break;
        }
        if map[(guard.0 as i32 + dir.0) as usize][(guard.1 as i32 + dir.1) as usize] {
            match dir {
                (-1, 0) => {
                    dir = (0, 1);
                }
                (1, 0) => {
                    dir = (0, -1);
                }
                (0, 1) => {
                    dir = (1, 0);
                }
                (0, -1) => {
                    dir = (-1, 0);
                }
                _ => panic!("Invalid direction"),
            }
        }
        guard = (
            (guard.0 as i32 + dir.0) as usize,
            (guard.1 as i32 + dir.1) as usize,
        );
        visited[guard.0][guard.1] = true;
    }

    let mut count = 0;
    for row in &visited {
        for cell in row {
            if *cell {
                count += 1;
            }
        }
    }

    let mut count2 = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] {
                continue;
            }
            if (i, j) == original_guard {
                continue;
            }
            map[i][j] = true;
            guard = original_guard;
            dir = original_dir;
            let mut count_visited = vec![vec![0; visited[0].len()]; visited.len()];
            
            loop {
                if guard.0 as i32 + dir.0 >= map.len() as i32
                    || guard.1 as i32 + dir.1 >= map[0].len() as i32
                    || (guard.0 as i32 + dir.0) < 0
                    || (guard.1 as i32 + dir.1) < 0
                {
                    break;
                }
                if count_visited[guard.0][guard.1] >= 4 {
                    count2 += 1;
                    break;
                }
                if map[(guard.0 as i32 + dir.0) as usize][(guard.1 as i32 + dir.1) as usize] {
                    match dir {
                        (-1, 0) => {
                            dir = (0, 1);
                        }
                        (1, 0) => {
                            dir = (0, -1);
                        }
                        (0, 1) => {
                            dir = (1, 0);
                        }
                        (0, -1) => {
                            dir = (-1, 0);
                        }
                        _ => panic!("Invalid direction"),
                    }
                }
                guard = (
                    (guard.0 as i32 + dir.0) as usize,
                    (guard.1 as i32 + dir.1) as usize,
                );
                count_visited[guard.0][guard.1] += 1;
            }
            map[i][j] = false;
        }
    }

    (count, count2)
}
