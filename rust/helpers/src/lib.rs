use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
/// "Read lines from a file and return an iterator over the lines."
///
/// The first line is the function signature. It's a function named read_lines that takes a single
/// parameter named filename. The parameter is a generic type P that implements the AsRef trait on Path.
/// The function returns a Result that contains an iterator over the lines of the file
///
/// Arguments:
///
/// * `filename`: The path to the file to read the lines from.
///
/// Returns:
///
/// A Result<io::Lines<io::BufReader<File>>>
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}