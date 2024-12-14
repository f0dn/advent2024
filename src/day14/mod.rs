use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("14");
    let mut robots = Vec::new();
    let width = 101;
    let height = 103;
    let mut map = vec![vec![0; width as usize]; height as usize];
    for line in input.flatten() {
        let mut split = line.split(" ");
        let pos = split.next().unwrap();
        let vel = split.next().unwrap();
        let comma = pos.find(",").unwrap();
        let x = pos[2..comma].parse::<u32>().unwrap();
        let y = pos[comma + 1..].parse::<u32>().unwrap();
        let comma = vel.find(",").unwrap();
        let dx = vel[2..comma].parse::<i32>().unwrap();
        let dy = vel[comma + 1..].parse::<i32>().unwrap();
        robots.push(((x, y), (dx, dy)));
        map[y as usize][x as usize] += 1;
    }

    let mut total = 0;
    let mut n = 0;
    'outer: while n < 10000 {
        for i in 0..robots.len() {
            let ((x, y), (dx, dy)) = robots[i];
            robots[i] = (
                (
                    ((x as i32 + dx + width as i32) % width as i32) as u32,
                    ((y as i32 + dy + height as i32) % height as i32) as u32,
                ),
                (dx, dy),
            );
            map[y as usize][x as usize] -= 1;
            map[robots[i].0 .1 as usize][robots[i].0 .0 as usize] += 1;
        }

        n += 1;

        if n == 100 {
            let mut q1 = 0;
            let mut q2 = 0;
            let mut q3 = 0;
            let mut q4 = 0;
            for i in 0..robots.len() {
                let ((x, y), _) = robots[i];
                if x < width / 2 && y < height / 2 {
                    q1 += 1;
                } else if x < width / 2 && y > height / 2 {
                    q2 += 1;
                } else if x > width / 2 && y < height / 2 {
                    q3 += 1;
                } else if x > width / 2 && y > height / 2 {
                    q4 += 1;
                }
            }
            total = q1 * q2 * q3 * q4;
        }

        for x in 5..(width as usize - 5) {
            for y in 5..(height as usize - 5) {
                if map[y][x] > 0
                    && map[y - 1][x] > 0
                    && map[y + 1][x] > 0
                    && map[y][x - 1] > 0
                    && map[y][x + 1] > 0
                    && map[y - 1][x - 1] > 0
                    && map[y - 1][x + 1] > 0
                    && map[y + 1][x - 1] > 0
                    && map[y + 1][x + 1] > 0
                {
                    break 'outer;
                }
            }
        }
    }

    (total, n)
}
