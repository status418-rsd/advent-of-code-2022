use std::iter::Peekable;
use std::str::Lines;
use regex::{Regex};


fn main() {
    let mut peekable: Peekable<Lines> = include_str!("input").lines().peekable();
    let mut movements: bool = false;
    // Vec1: the index of stacks represents the <stack>.
    // Vec2: crates are filled in like the crane would to - so we can pop and splice.
    // &str: represents the aktual supply item.
    // e.g.: [[E, D, F], [A, C], [X, Y]]
    // problem: as i read line by line - how to determinate in which stack the create needs to go?

    let mut stacks: Vec<Vec<&str>> = Vec::new();
    let mut stack: Vec<&str> = Vec::from(["Z", "N"]);
    stacks.push(stack);
    stack = Vec::from(["M", "C", "D"]);
    stacks.push(stack);
    stack = Vec::from(["P"]);
    stacks.push(stack);
    while let Some(mut line) = peekable.next() {



    }
    for row in &stacks {
        println!("{:?}", row);
    }
}
