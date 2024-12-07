use crate::get_input;

pub fn solve() -> (u128, u128) {
    let input = get_input!("07");
    let mut total = 0;
    let mut total2 = 0;
    for line in input.map_while(Result::ok) {
        let mut parts = line.split(": ");
        let target = parts.next().unwrap().parse::<u128>().unwrap();
        let nums = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        if is_target(&nums, target, nums[0], 1) {
            total += target;
        }
        if is_target2(&nums, target, nums[0], 1) {
            total2 += target;
        }
    }
    (total, total2)
}

fn is_target(nums: &Vec<u128>, target: u128, prev: u128, start: usize) -> bool {
    if prev == target && start == nums.len() {
        return true;
    }

    if start >= nums.len() || prev > target {
        return false;
    }

    is_target(nums, target, prev + nums[start], start + 1)
        || is_target(nums, target, prev * nums[start], start + 1)
}

fn concat(num1: u128, num2: u128) -> u128 {
    format!("{}{}", num1, num2).parse::<u128>().unwrap()
}

fn is_target2(nums: &Vec<u128>, target: u128, prev: u128, start: usize) -> bool {
    if prev == target && start == nums.len() {
        return true;
    }

    if start >= nums.len() || prev > target {
        return false;
    }

    is_target2(nums, target, prev + nums[start], start + 1)
        || is_target2(nums, target, prev * nums[start], start + 1)
        || is_target2(nums, target, concat(prev, nums[start]), start + 1)
}
