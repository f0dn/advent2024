use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("25");
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for segment in input.map_while(Result::ok).collect::<Vec<String>>().split(|x| x.is_empty()) {
        let first = &segment[0];
        let mut v = vec![0; 5];
        for s in segment.iter() {
            for (j, c) in s.chars().enumerate() {
                if c == '#' {
                    v[j] += 1;
                }
            }
        }
        if first == "#####" {
            locks.push(v);
        } else {
            keys.push(v);
        }
    }

    let mut unique = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            let mut works = true;
            for i in 0..5 {
                if key[i] + lock[i] > 7 {
                    works = false;
                    break;
                }
            }
            if works {
                unique += 1;
            }
        }
    }

    (unique, 0)
}
