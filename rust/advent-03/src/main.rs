use regex::Regex;
use std::fmt::Debug;
use std::iter::Peekable;
use std::str::Lines;



static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
fn main() {
    let mut round_1: i32 = 0;
    // Vec1 = Groups, Vec2 = Backpacks, Vec3 = Letters
    let mut groups: Vec<Vec<Vec<&str>>> = Vec::new();
    let mut peekable: Peekable<Lines> = include_str!("input").lines().peekable();
    let mut index: usize = 0;
    while let Some(mut line) = peekable.next() {
        line = line.trim();
        round_1 += get_priority_score_for_backpack(&line);

        // runs only on first iteration
        if groups.last().is_none() {
            let mut split: Vec<&str> = line.split("").collect();
            split.retain(|&x| x != "");
            let value: Vec<Vec<&str>> = Vec::from([split]);
            groups.push(value);
            continue;
        };
        if groups.last().unwrap().len() >= 3 {
            index += 1;
            let mut split: Vec<&str> = line.split("").collect();
            split.retain(|&x| x != "");
            groups.push(Vec::from([split]));
        } else {
            let mut split: Vec<&str> = line.split("").collect();
            split.retain(|&x| x != "");
            groups[index].push(split);
        }
    }
    let round_2 = get_score_for_groups(&groups);
    println!("{}", round_1);
    println!("{}", round_2);
}

// 01
fn get_priority_score_for_backpack (line: &str) -> i32 {
    let test_lowercase: Regex = Regex::new(r"^[a-z]+$").unwrap();
    let mut compartment_2: Vec<&str> = line.split("").collect();
    let compartment_1: Vec<&str> = compartment_2.drain(1 .. compartment_2.len() / 2).collect();
    let mut duplicate_letter: Vec<&str> = Vec::new();
    let mut res = 0;
    for letter in compartment_1 { let deduplicate_index = compartment_2.iter().position(|&r| r == letter); if deduplicate_index.is_some() { duplicate_letter.push(compartment_2[deduplicate_index.unwrap()]); break; } }
    for i in duplicate_letter { res += ASCII_LOWER.iter().position(|&r| r.to_string() == i.to_ascii_lowercase()).unwrap() as i32 + if test_lowercase.is_match(i) { 1 } else { 27 } }
    res
}

// 02
fn get_score_for_groups(groups: &Vec<Vec<Vec<&str>>>) -> i32 {
    let mut badges: Vec<&str> = Vec::new();
    let mut score: i32 = 0;
    for group in groups {
        if group.len() < 3 { continue; }
        let mut backpack_1: Vec<&str> = group[0].clone();
        let mut backpack_2: Vec<&str> = group[1].clone();
        let mut backpack_3: Vec<&str> = group[2].clone();
        let mut badge: &str = "";
        for letter in backpack_1 {
            let temp: &str = letter;
            let exists_in_backpack_2: bool = backpack_2.iter().position(|&r| r == temp).is_some();
            let exists_in_backpack_3: bool = backpack_3.iter().position(|&r| r == temp).is_some();
            if exists_in_backpack_2 && exists_in_backpack_3 {
                badges.push(&letter);
                break;
            }
        }
        // println!("badge {}", badge);

    }
    let test_lowercase: Regex = Regex::new(r"^[a-z]+$").unwrap();
    for badge in badges {
        score += ASCII_LOWER.iter().position(|&r| r.to_string() == badge.to_ascii_lowercase()).unwrap() as i32 + if test_lowercase.is_match(badge) { 1 } else { 27 }
    }

    score
}
