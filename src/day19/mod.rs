use crate::get_input;

pub fn solve() -> (u32, u128) {
    let mut input = get_input!("19").flatten();
    let towels = input
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    input.next();

    let mut possible = 0;
    let mut possibilities = 0;
    for line in input {
        let mut cache = vec![None; line.len()];
        let count = count_possible(0, &line, &towels, &mut cache);
        if count.is_some() && count.unwrap() > 0 {
            possible += 1;
            possibilities += count.unwrap();
        }
    }
    (possible as u32, possibilities)
}

fn count_possible(
    i: usize,
    input: &str,
    towels: &Vec<String>,
    cache: &mut Vec<Option<u128>>,
) -> Option<u128> {
    if i == input.len() {
        return Some(1);
    }
    if let Some(x) = cache[i] {
        return Some(x);
    }
    for towel in towels {
        if i + towel.len() <= input.len() && &input[i..i + towel.len()] == towel {
            if cache[i].is_none() {
                cache[i] = Some(0);
            }
            let count = count_possible(i + towel.len(), input, towels, cache);
            if count.is_some() {
                cache[i] = Some(cache[i].unwrap() + count.unwrap());
            }
        }
    }
    cache[i]
}
