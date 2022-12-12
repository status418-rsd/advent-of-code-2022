use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
    let mut highest: i64 = 0;
    let mut current_calories: i64 = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    highest = cmp::max(current_calories, highest);
                    current_calories = 0;
                    continue;
                }
                current_calories += ip.parse::<i64>().unwrap();
            }
        }
    }
    println!("{}", highest.to_string());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}