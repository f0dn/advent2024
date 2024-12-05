use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("05");
    let mut rules = Vec::new();
    let mut total = 0;
    let mut total2 = 0;
    let mut done_rules = false;
    for line in input.flatten() {
        if line.is_empty() {
            done_rules = true;
            continue;
        }

        if !done_rules {
            let mut rule = line.split("|");
            rules.push((
                rule.next().unwrap().parse::<u32>().unwrap(),
                rule.next().unwrap().parse::<u32>().unwrap(),
            ));
            continue;
        }

        let mut updates = line
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if is_valid(&rules, &updates) {
            total += updates[updates.len() / 2];
        } else {
            while !is_valid(&rules, &updates) {
                for rule in &rules {
                    let mut second_index = None;
                    for i in 0..updates.len() {
                        if updates[i] == rule.1 {
                            second_index = Some(i);
                        }
                        if updates[i] == rule.0 && second_index.is_some() {
                            let temp = updates[second_index.unwrap()];
                            updates[second_index.unwrap()] = updates[i];
                            updates[i] = temp;
                        }
                    }
                }
            }
            total2 += updates[updates.len() / 2];
        }
    }

    (total, total2)
}

fn is_valid(rules: &Vec<(u32, u32)>, updates: &Vec<u32>) -> bool {
    let mut good = true;
    'first: for rule in rules {
        let mut found_second = false;
        for update in updates {
            if *update == rule.1 {
                found_second = true;
            }
            if *update == rule.0 && found_second {
                good = false;
                break 'first;
            }
        }
    }
    good
}
