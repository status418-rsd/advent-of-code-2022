use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Peekable;
use std::str::Lines;

fn main() {
    let mut peekable: Peekable<Lines> = include_str!("input").lines().peekable();
    let mut characters:Vec<&str> = Vec::new();
    while let Some(line) = peekable.next() {
        characters = line.split_terminator("").collect();
        characters.remove(0);
    }
    println!("part 1: {}", result(&characters, 4));
    println!("part 2: {}", result(&characters, 14));
}

fn result(characters: &Vec<&str>, part_len: usize) -> usize {
    let mut buffer: Vec<&str> = Vec::new();
    let mut res: usize = 0;
    for (i, char) in characters.iter().enumerate() {
        if buffer.len() >= part_len && !has_duplicates(&buffer) {
            res = i;
            break;
        }
        if buffer.len() >= part_len {
            buffer.remove(0);
        }
        buffer.push(char);
    }
    res
}

fn has_duplicates<T: Eq + Hash>(vec: &Vec<T>) -> bool {
    let mut seen = HashSet::new();

    for item in vec {
        if seen.contains(item) {
            return true;
        }
        seen.insert(item);
    }
    false
}
