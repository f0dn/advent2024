use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("03");
    let mut count = 0;
    let mut count2 = 0;
    let mut do2 = true;
    input.map_while(Result::ok).for_each(|line| {
        for i in 0..(line.len() - 4) {
            if &line[i..i + 4] == "do()" {
                do2 = true;
            }
            if i < (line.len() - 7) && &line[i..i + 7] == "don't()" {
                do2 = false;
            }
            if &line[i..i + 4] == "mul(" {
                let first = line[i + 4..].find(',');
                if first.is_none() {
                    continue;
                }
                let first_num = line[i + 4..i + 4 + first.unwrap()].parse::<u32>();
                let second = line[i + 4 + first.unwrap() + 1..].find(')');
                if second.is_none() {
                    continue;
                }
                let second_num = line
                    [i + 4 + first.unwrap() + 1..i + 4 + first.unwrap() + 1 + second.unwrap()]
                    .parse::<u32>();
                if first_num.is_ok() && second_num.is_ok() {
                    let mul = first_num.unwrap() * second_num.unwrap();
                    count += mul;
                    if do2 {
                        count2 += mul;
                    }
                }
            }
        }
    });
    (count, count2)
}
