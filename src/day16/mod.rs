use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("16");
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in input.flatten().enumerate() {
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

    let mut cache = vec![vec![None; map[0].len()]; map.len()];

    let best = find_min_score(&mut map, start, end, &mut cache, (0, 1)).unwrap();
    (best as u32, 0)
}

fn find_min_score(
    map: &mut Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
    cache: &mut Vec<Vec<Option<((i32, i32), usize)>>>,
    prev_dir: (i32, i32),
) -> Option<usize> {
    if start == end {
        return Some(0);
    }
    if let Some((dir, score)) = &cache[start.0][start.1] {
        if *dir == prev_dir {
            //return Some(*score);
        }
        //return Some(*score + 1000);
    }
    map[start.0][start.1] = true;
    let mut turn = prev_dir;
    let mut cache_best = usize::MAX;
    let mut best = usize::MAX;
    for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let new_pos = ((start.0 as i32 + i) as usize, (start.1 as i32 + j) as usize);
        if map[new_pos.0][new_pos.1] {
            continue;
        }
        let new_score = if (*i, *j) == prev_dir { 1 } else { 1001 };
        let next_best = find_min_score(map, new_pos, end, cache, (*i, *j));
        if next_best.is_none() {
            continue;
        }
        if new_score + next_best.unwrap() < best {
            best = new_score + next_best.unwrap();
            if new_score == 1001 {
                turn = (*i, *j);
                cache_best = best - 1000;
            } else {
                turn = prev_dir;
                cache_best = best;
            }
        }
    }
    map[start.0][start.1] = false;
    if best == usize::MAX {
        return None;
    }
    cache[start.0][start.1] = Some((turn, cache_best));
    Some(best)
}
