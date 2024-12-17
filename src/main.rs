#![recursion_limit = "1024"]
mod day17;

fn main() {
    solve!(day17);
}

#[macro_export]
macro_rules! get_input {
    ($day:literal) => {
        {
            use std::fs::File;
            use std::io::{self, BufRead};
            let file = File::open(concat!("src/day", $day, "/input")).unwrap();
            io::BufReader::new(file).lines()
        }
    };
}

#[macro_export]
macro_rules! solve {
    ($day:ident) => {
        {
            use std::time::Instant;
            let now = Instant::now();
            let solution = $day::solve();
            let elapsed = now.elapsed();
            println!("{}: {}ms", stringify!($day), elapsed.as_millis());
            println!("  Part 1: {}", solution.0);
            println!("  Part 2: {}", solution.1);
        }
    };
}
