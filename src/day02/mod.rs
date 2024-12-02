use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("02");
    let mut num_safe = 0;
    let mut num_safe2 = 0;
    input.map_while(Result::ok).for_each(|line| {
        let mut rev = false;
        let mut nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if nums[0] > nums[1] {
            rev = true;
        }
        let mut safe = true;
        for i in 1..nums.len() {
            let diff: i32 = nums[i - 1] - nums[i];
            if rev {
                if !(1..=3).contains(&diff) {
                    safe = false;
                    break;
                }
            } else if !(-3..=-1).contains(&diff) {
                safe = false;
                break;
            }
        }
        if safe {
            num_safe += 1;
            num_safe2 += 1;
        } else {
            for i in 0..nums.len() {
                let num = nums.remove(i);
                let mut rev = false;
                if nums[0] > nums[1] {
                    rev = true;
                }
                let mut safe2 = true;
                for j in 1..nums.len() {
                    let diff: i32 = nums[j - 1] - nums[j];
                    if rev {
                        if !(1..=3).contains(&diff) {
                            safe2 = false;
                            break;
                        }
                    } else if !(-3..=-1).contains(&diff) {
                        safe2 = false;
                        break;
                    }
                }
                if safe2 {
                    num_safe2 += 1;
                    break;
                }
                nums.insert(i, num);
            }
        }
    });
    (num_safe, num_safe2)
}
