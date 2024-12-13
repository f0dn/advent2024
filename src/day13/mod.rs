use crate::get_input;

pub fn solve() -> (u32, u128) {
    let input = get_input!("13").map_while(Result::ok).collect::<Vec<String>>();
    let mut total = 0;
    let mut total2 = 0;
    for group in input.split(|x| x.is_empty()) {
        let a = group[0].clone();
        let b = group[1].clone();
        let prize = group[2].clone();
        let mut prize = prize.split(", Y=");
        let prize = (
            prize.next().unwrap()[9..].parse::<u32>().unwrap(),
            prize.next().unwrap().parse::<u32>().unwrap(),
        );
        let mut a = a.split(", Y+");
        let a = (
            a.next().unwrap()[12..].parse::<u32>().unwrap(),
            a.next().unwrap().parse::<u32>().unwrap(),
        );
        let mut b = b.split(", Y+");
        let b = (
            b.next().unwrap()[12..].parse::<u32>().unwrap(),
            b.next().unwrap().parse::<u32>().unwrap(),
        );

        let mut num_b = (prize.0 / b.0).min(100);
        let mut num_a = (prize.0 - num_b * b.0) / a.0;
        loop {
            if num_a > 100 {
                break;
            }
            let x_total = num_a * a.0 + num_b * b.0;
            if x_total == prize.0 && num_a * a.1 + num_b * b.1 == prize.1 {
                total += num_a * 3 + num_b;
                break;
            }

            if x_total > prize.0 {
                if num_b == 0 {
                    break;
                }
                num_b -= 1;
                num_a = (prize.0 - num_b * b.0) / a.0;
            } else {
                num_a += 1;
            }
        }

        let prize = (prize.0 as u128 + 10000000000000, prize.1 as u128 + 10000000000000);
        let a = (a.0 as u128, a.1 as u128);
        let b = (b.0 as u128, b.1 as u128);
        let num_b = (prize.1 as f64 - a.1 as f64 * prize.0 as f64 / a.0 as f64)
            / (b.1 as f64 - a.1 as f64 * b.0 as f64 / a.0 as f64);
        let num_a = (prize.0 as f64 - num_b * b.0 as f64) / a.0 as f64;
        let num_b = num_b.round() as u128;
        let num_a = num_a.round() as u128;
        if num_a * a.0 + num_b * b.0 == prize.0 && num_a * a.1 + num_b * b.1 == prize.1 {
            total2 += num_a * 3 + num_b;
        }
    }
    (total, total2)
}
