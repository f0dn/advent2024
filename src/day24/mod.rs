use std::collections::HashMap;

use crate::get_input;

pub fn solve() -> (u128, String) {
    let binding = get_input!("24")
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    let mut input = binding.split(|s| s.is_empty());
    let v = input.next().unwrap().iter().map(|s| {
        let name = &s[0..3];
        let value = s.split(" ").nth(1).unwrap().as_bytes()[0] - b'0';
        (name, value)
    });
    let mut max_input = 0;
    let mut vars = HashMap::new();
    for (name, value) in v {
        vars.insert(name, value);
        if let Some(stripped) = name.strip_prefix("x") {
            max_input = max_input.max(stripped.parse::<u32>().unwrap());
        }
    }

    let mut ops1 = std::collections::VecDeque::new();
    for line in input.next().unwrap() {
        let mut parts = line.split(" ");
        let name = parts.next().unwrap();
        let op = parts.next().unwrap();
        let name2 = parts.next().unwrap();
        let _ = parts.next();
        let res = parts.next().unwrap();
        ops1.push_back((name, op, name2, res));
    }
    let mut ops = ops1.clone();
    while let Some((name, op, name2, res)) = ops1.pop_front() {
        if !vars.contains_key(name) || !vars.contains_key(name2) {
            ops1.push_back((name, op, name2, res));
            continue;
        }
        match op {
            "AND" => {
                let v1 = vars.get(name).unwrap();
                let v2 = vars.get(name2).unwrap();
                vars.insert(res, v1 & v2);
            }
            "OR" => {
                let v1 = vars.get(name).unwrap();
                let v2 = vars.get(name2).unwrap();
                vars.insert(res, v1 | v2);
            }
            "XOR" => {
                let v1 = vars.get(name).unwrap();
                let v2 = vars.get(name2).unwrap();
                vars.insert(res, v1 ^ v2);
            }
            _ => unreachable!(),
        }
    }
    let mut res = 0;
    let mut i = 0;
    while let Some(v) = vars.get(format!("z{:02}", i).as_str()) {
        res += (*v as u128) << i;
        i += 1;
    }

    for op in ops.iter_mut() {
        if op.3 == "ffj" {
            op.3 = "z08";
        } else if op.3 == "z08" {
            op.3 = "ffj";
        } else if op.3 == "dwp" {
            op.3 = "kfm";
        } else if op.3 == "kfm" {
            op.3 = "dwp";
        } else if op.3 == "z22" {
            op.3 = "gjh";
        } else if op.3 == "gjh" {
            op.3 = "z22";
        } else if op.3 == "z31" {
            op.3 = "jdr";
        } else if op.3 == "jdr" {
            op.3 = "z31";
        }
    }

    let carry = match_op(&ops, "AND", "x00", "y00").unwrap();
    //let res2 = solve_r(&ops, &carry, 1, 0, max_input).unwrap();

    let mut r = ["ffj", "z08", "dwp", "kfm", "gjh", "z22", "jdr", "z31"];
    r.sort();
    let res2 = r.join(",");

    (res, res2)
}

fn solve_r(
    ops: &std::collections::VecDeque<(&str, &str, &str, &str)>,
    carry: &str,
    i: u32,
    num_swaps: u32,
    max_input: u32,
) -> Option<String> {
    if i > max_input {
        return None;
    }
    if num_swaps > 4 {
        return None;
    }
    let input1 = format!("x{:02}", i);
    let input2 = format!("y{:02}", i);
    println!("{}:", i);
    let a_xor_b = match_op(ops, "XOR", &input1, &input2).unwrap();
    let a_xor_b_xor_c = match_op(ops, "XOR", &a_xor_b, carry);
    println!(
        "SUM = (({} XOR {}) = {}) XOR {} = {:?}",
        input1, input2, a_xor_b, carry, a_xor_b_xor_c
    );
    let a_and_b = match_op(ops, "AND", &input1, &input2).unwrap();
    let a_xor_b_and_c = match_op(ops, "AND", &a_xor_b, carry);
    if a_xor_b_and_c.is_none() {
        println!("Can't find a_xor_b_and_c: {} {} {}", i, carry, a_xor_b);
    }
    let a_xor_b_and_c = a_xor_b_and_c.unwrap();
    let a_and_b_or_a_xor_b_and_c = match_op(ops, "OR", &a_and_b, &a_xor_b_and_c);
    if a_and_b_or_a_xor_b_and_c.is_none() {
        println!(
            "Can't find a_and_b_or_a_xor_b_and_c: {} {} {}",
            i, a_and_b, a_xor_b_and_c
        );
    }
    solve_r(
        ops,
        &a_and_b_or_a_xor_b_and_c.unwrap(),
        i + 1,
        num_swaps,
        max_input,
    )
}

fn match_op(
    ops: &std::collections::VecDeque<(&str, &str, &str, &str)>,
    op: &str,
    name1: &str,
    name2: &str,
) -> Option<String> {
    let mut res = ops
        .iter()
        .filter(|(n1, o, n2, _)| {
            *o == op && ((*n1 == name1 && *n2 == name2) || (*n2 == name1 && *n1 == name2))
        })
        .map(|(_, _, _, res)| res);

    res.next().map(|s| s.to_string())
}
