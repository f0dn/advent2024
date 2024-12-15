use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("15");
    let mut map = Vec::new();
    let binding = input.map_while(Result::ok).collect::<Vec<String>>();
    let mut parts = binding.split(|x| x.is_empty());
    let mut robot = (0, 0);
    for (i, line) in parts.next().unwrap().iter().enumerate() {
        map.push(line.chars().collect::<Vec<char>>());
        if let Some(j) = line.chars().position(|x| x == '@') {
            robot = (i as i32, j as i32);
            map[i][j] = '.';
        }
    }

    let mut map2 = vec![vec!['#'; map[0].len() * 2]; map.len()];
    let mut robot2 = (robot.0, robot.1 * 2);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                c @ ('#' | '.') => {
                    map2[i][j * 2] = c;
                    map2[i][j * 2 + 1] = c;
                }
                'O' => {
                    map2[i][j * 2] = '[';
                    map2[i][j * 2 + 1] = ']';
                }
                _ => unreachable!(),
            }
        }
    }

    let instructions = parts.next().unwrap().iter().flat_map(|x| x.chars());
    'outer: for instruction in instructions.clone() {
        if instruction == '\n' {
            continue;
        }
        let dir = match instruction {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        let mut curr = (robot.0 + dir.0, robot.1 + dir.1);
        loop {
            if map[curr.0 as usize][curr.1 as usize] == '#' {
                continue 'outer;
            }
            if map[curr.0 as usize][curr.1 as usize] == '.' {
                break;
            }
            curr = (curr.0 + dir.0, curr.1 + dir.1);
        }
        map[curr.0 as usize][curr.1 as usize] = 'O';
        map[(robot.0 + dir.0) as usize][(robot.1 + dir.1) as usize] = '.';
        robot = (robot.0 + dir.0, robot.1 + dir.1);
    }

    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                total += i * 100 + j
            }
        }
    }

    'outer: for instruction in instructions {
        if instruction == '\n' {
            continue;
        }
        let dir = match instruction {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        let mut curr = (robot2.0 + dir.0, robot2.1 + dir.1);
        loop {
            if map2[curr.0 as usize][curr.1 as usize] == '#' {
                continue 'outer;
            }
            if map2[curr.0 as usize][curr.1 as usize] == '.' {
                break;
            }
            curr = (curr.0 + dir.0, curr.1 + dir.1);
        }

        if dir == (0, -1) || dir == (0, 1) {
            while curr != robot2 {
                map2[curr.0 as usize][curr.1 as usize] =
                    map2[(curr.0 - dir.0) as usize][(curr.1 - dir.1) as usize];
                curr = (curr.0 - dir.0, curr.1 - dir.1);
            }
            robot2 = (robot2.0 + dir.0, robot2.1 + dir.1);
        } else {
            let first_box = (robot2.0 + dir.0, robot2.1 + dir.1);
            if can_move(&map2, first_box, dir) {
                move_map(&mut map2, first_box, dir);
                robot2 = first_box;
            }
        }
    }

    let mut total2 = 0;
    for i in 0..map2.len() {
        for j in 0..map2[i].len() {
            if map2[i][j] == '[' {
                total2 += i * 100 + j
            }
        }
    }

    (total as u32, total2 as u32)
}

fn can_move(map: &Vec<Vec<char>>, curr: (i32, i32), dir: (i32, i32)) -> bool {
    if map[curr.0 as usize][curr.1 as usize] == '#' {
        return false;
    }
    if map[curr.0 as usize][curr.1 as usize] == '.' {
        return true;
    }

    let head = if map[curr.0 as usize][curr.1 as usize] == ']' {
        (curr.0, curr.1 - 1)
    } else {
        curr
    };

    can_move(map, (head.0 + dir.0, head.1), dir)
        && can_move(map, (head.0 + dir.0, head.1 + 1), dir)
}

fn move_map(map: &mut Vec<Vec<char>>, curr: (i32, i32), dir: (i32, i32)) {
    if map[curr.0 as usize][curr.1 as usize] == '#' {
        unreachable!();
    }
    if map[curr.0 as usize][curr.1 as usize] == '.' {
        return;
    }

    let head = if map[curr.0 as usize][curr.1 as usize] == ']' {
        (curr.0, curr.1 - 1)
    } else {
        curr
    };

    move_map(map, (head.0 + dir.0, head.1), dir);
    if map[(head.0 + dir.0) as usize][(head.1 + 1) as usize] != ']' {
        move_map(map, (head.0 + dir.0, head.1 + 1), dir);
    }
    map[(head.0 + dir.0) as usize][(head.1) as usize] = map[head.0 as usize][head.1 as usize];
    map[(head.0 + dir.0) as usize][(head.1 + 1) as usize] =
        map[head.0 as usize][head.1 as usize + 1];
    map[head.0 as usize][head.1 as usize] = '.';
    map[head.0 as usize][head.1 as usize + 1] = '.';
}
