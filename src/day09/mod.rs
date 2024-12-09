use crate::get_input;

pub fn solve() -> (u128, u128) {
    let input = get_input!("09");
    let vec = input
        .map_while(Result::ok)
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u128)
        .collect::<Vec<u128>>();
    let mut disks = Vec::new();
    let mut space = Vec::new();
    let mut checksum = 0;
    let mut befores = Vec::new();
    let mut before = 0;
    for i in (1..vec.len()).step_by(2) {
        disks.push(vec[i - 1]);
        space.push(vec[i]);
        befores.push(before);
        before += vec[i];
        before += vec[i - 1];
    }
    befores.push(before);
    disks.push(vec[vec.len() - 1]);
    let befores2 = befores.clone();

    let disks2 = disks.clone();
    let mut space2 = space.clone();

    let mut before = disks[0];
    let mut disks_start = 1;
    let mut space_start = 0;

    let mut i = disks.len() - 1;
    loop {
        if disks_start > i {
            break;
        }
        let num = disks[i].min(space[space_start]);
        checksum += i as u128 * num * (before + before + num - 1) / 2;
        disks[i] -= num;
        space[space_start] -= num;
        before += num;

        if disks[i] == 0 {
            i -= 1;
        }
        if space[space_start] == 0 {
            space_start += 1;
            checksum +=
                disks_start as u128 * disks[disks_start] * (2 * before + disks[disks_start] - 1)
                    / 2;
            before += disks[disks_start];
            disks_start += 1;
        }
    }

    let mut checksum2 = 0;

    for i in (1..disks2.len()).rev() {
        let mut fit = false;
        for j in 0..space2.len() {
            if befores[j] > befores[i] {
                break;
            }
            if space2[j] >= disks2[i] {
                checksum2 += i as u128 * disks2[i] * (2 * (befores[j] + disks2[j]) + disks2[i] - 1) / 2;
                before += disks2[i];
                fit = true;
                space2[j] -= disks2[i];
                befores[j] += disks2[i];
                break;
            }
        }
        if !fit {
            checksum2 += i as u128 * disks2[i] * (2 * befores2[i] + disks2[i] - 1) / 2;
        }
    }

    (checksum, checksum2)
}
