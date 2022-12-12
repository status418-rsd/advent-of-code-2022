use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_calories: i32 = 0;
    let mut ranking_list =vec![0];
    if let Ok(lines) = read_lines("./input") {
        let mut peekable = lines.peekable();
        while let Some(line) = peekable.next() {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    ranking_list.push(current_calories);
                    current_calories = 0;
                    continue;
                }
                current_calories += ip.parse::<i32>().unwrap();
                if peekable.peek().is_none() {
                    ranking_list.push(current_calories);
                }
            }
        }
    }
    ranking_list.sort_by(|a, b| b.cmp(a));
    let result_1: i32 = ranking_list[0];
    let result_2: i32 = ranking_list[ .. 3].iter().sum();
    println!("{}", result_1.to_string());
    println!("{}", result_2.to_string());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}