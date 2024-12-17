use crate::get_input;

pub fn solve() -> (String, u128) {
    let mut input = get_input!("17").map_while(Result::ok);
    let mut a = input.next().unwrap()[12..].parse::<u128>().unwrap();
    let mut b = input.next().unwrap()[12..].parse::<u128>().unwrap();
    let mut c = input.next().unwrap()[12..].parse::<u128>().unwrap();
    input.next();
    let program = input.next().unwrap()[9..]
        .split(",")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    let mut pc = 0;
    let mut out = Vec::new();
    while pc < program.len() {
        let opcode = program[pc];
        let operand = program[pc + 1];
        match opcode {
            0 => {
                a = a / 2_u128.pow(combo(a, b, c, operand) as u32);
            }
            1 => {
                b ^= operand;
            }
            2 => {
                b = combo(a, b, c, operand) % 8;
            }
            3 => {
                if a != 0 {
                    pc = operand as usize;
                    continue;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                out.push(combo(a, b, c, operand) % 8);
            }
            6 => {
                b = a / 2_u128.pow(combo(a, b, c, operand) as u32);
            }
            7 => {
                c = a / 2_u128.pow(combo(a, b, c, operand) as u32);
            }
            _ => {
                unreachable!();
            }
        }
        pc += 2;
    }

    let mut prev_a = vec![0];
    let mut new_pa = Vec::new();
    for num in program.iter().rev() {
        for pa in prev_a.iter() {
            for i in 0..8 {
                let mut a = *pa * 8 + i;
                let mut b = 0;
                let mut c = 0;
                let mut pc = 0;
                while pc < program.len() {
                    let opcode = program[pc];
                    let operand = program[pc + 1];
                    match opcode {
                        0 => {
                            a = a / 2_u128.pow(combo(a, b, c, operand) as u32);
                        }
                        1 => {
                            b ^= operand;
                        }
                        2 => {
                            b = combo(a, b, c, operand) % 8;
                        }
                        3 => {
                            if a != 0 {
                                pc = operand as usize;
                                continue;
                            }
                        }
                        4 => {
                            b ^= c;
                        }
                        5 => {
                            if combo(a, b, c, operand) % 8 == *num {
                                new_pa.push(*pa * 8 + i);
                            }
                            break;
                        }
                        6 => {
                            b = a / 2_u128.pow(combo(a, b, c, operand) as u32);
                        }
                        7 => {
                            c = a / 2_u128.pow(combo(a, b, c, operand) as u32);
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                    pc += 2;
                }
            }
        }
        prev_a = new_pa;
        new_pa = Vec::new();
    }
    (
        out.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(","),
        *prev_a.iter().min().unwrap(),
    )
}

fn combo(a: u128, b: u128, c: u128, op: u128) -> u128 {
    match op {
        n @ (0..=3) => n,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    }
}
