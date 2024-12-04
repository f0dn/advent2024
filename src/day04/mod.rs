use crate::get_input;

pub fn solve() -> (u32, u32) {
    let input = get_input!("04");
    let mut grid = Vec::new();
    input.map_while(Result::ok).for_each(|line| {
        grid.push(line.chars().collect::<Vec<char>>());
    });

    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] != 'X' {
                continue;
            }
            if x >= 3 {
                if grid[x - 1][y] == 'M' && grid[x - 2][y] == 'A' && grid[x - 3][y] == 'S' {
                    count += 1;
                }
                if y >= 3
                    && grid[x - 1][y - 1] == 'M'
                    && grid[x - 2][y - 2] == 'A'
                    && grid[x - 3][y - 3] == 'S'
                {
                    count += 1;
                }
                if y < grid[x].len() - 3
                    && grid[x - 1][y + 1] == 'M'
                    && grid[x - 2][y + 2] == 'A'
                    && grid[x - 3][y + 3] == 'S'
                {
                    count += 1;
                }
            }
            if y >= 3 {
                if grid[x][y - 1] == 'M' && grid[x][y - 2] == 'A' && grid[x][y - 3] == 'S' {
                    count += 1;
                }
                if x < grid.len() - 3
                    && grid[x + 1][y - 1] == 'M'
                    && grid[x + 2][y - 2] == 'A'
                    && grid[x + 3][y - 3] == 'S'
                {
                    count += 1;
                }
            }
            if x < grid.len() - 3 {
                if grid[x + 1][y] == 'M' && grid[x + 2][y] == 'A' && grid[x + 3][y] == 'S' {
                    count += 1;
                }
                if y < grid[x].len() - 3
                    && grid[x + 1][y + 1] == 'M'
                    && grid[x + 2][y + 2] == 'A'
                    && grid[x + 3][y + 3] == 'S'
                {
                    count += 1;
                }
            }
            if y < grid[x].len() - 3
                && grid[x][y + 1] == 'M'
                && grid[x][y + 2] == 'A'
                && grid[x][y + 3] == 'S'
            {
                count += 1;
            }
        }
    }
    let mut count2 = 0;
    for x in 0..(grid.len() - 2) {
        for y in 0..(grid[x].len() - 2) {
            if grid[x + 1][y + 1] != 'A' {
                continue;
            }
            if grid[x][y] == grid[x + 2][y + 2] || grid[x + 2][y] == grid[x][y + 2] {
                continue;
            }
            let num_m = if grid[x][y] == 'M' { 1 } else { 0 }
                + if grid[x + 2][y] == 'M' { 1 } else { 0 }
                + if grid[x][y + 2] == 'M' { 1 } else { 0 }
                + if grid[x + 2][y + 2] == 'M' { 1 } else { 0 };
            let num_s = if grid[x][y] == 'S' { 1 } else { 0 }
                + if grid[x + 2][y] == 'S' { 1 } else { 0 }
                + if grid[x][y + 2] == 'S' { 1 } else { 0 }
                + if grid[x + 2][y + 2] == 'S' { 1 } else { 0 };
            if num_m == 2 && num_s == 2 {
                count2 += 1;
            }
        }
    }
    (count, count2)
}
