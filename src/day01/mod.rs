use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("01");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    input.map_while(Result::ok).for_each(|line| {
        let nums = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        list1.push(nums[0]);
        list2.push(nums[1]);
    });
    let mut sum = 0;
    for num in list1.iter() {
        let mut count = 0;
        for num2 in list2.iter() {
            if num == num2 {
                count += 1;
            }
        }
        sum += count * num;
    }
    list1.sort();
    list2.sort();
    let sum_diff: u32 = list1.iter().zip(list2.iter()).map(|(x, y)| x.abs_diff(*y)).sum();
    (sum_diff, sum)
}
