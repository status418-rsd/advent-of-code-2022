use std::iter::Peekable;
use std::str::Lines;


fn main() {
    let mut peekable: Peekable<Lines> = include_str!("input").lines().peekable();
    let mut commands: Vec<Vec<usize>> = Vec::new();
    let mut stacks: Vec<Vec<&str>> = Vec::new();

    stacks.push(vec!["G", "F", "V", "H", "P", "S"]);
    stacks.push(vec!["G", "J", "F", "B", "V", "D", "Z", "M"]);
    stacks.push(vec!["G", "M", "L", "J", "N"]);
    stacks.push(vec!["N", "G", "Z", "V", "D", "W", "P"]);
    stacks.push(vec!["V", "R", "C", "B"]);
    stacks.push(vec!["V", "R", "S", "M", "P", "W", "L", "Z"]);
    stacks.push(vec!["T", "H", "P"]);
    stacks.push(vec!["Q", "R", "S", "N", "C", "H", "Z", "V"]);
    stacks.push(vec!["F", "L", "G", "P", "V", "Q", "J"]);

    while let Some(mut line) = peekable.next() {

        let parts: Vec<&str> = line.split_whitespace().collect();
        let command = parts[1].parse::<usize>().expect("Failed to parse command");
        let from_stack = parts[3].parse::<usize>().expect("Failed to parse from_stack");
        let to_stack = parts[5].parse::<usize>().expect("Failed to parse to_stack");

        let result: Vec<usize> = vec![command, from_stack, to_stack];

        commands.push(result);
    }
    for command in &commands {
        let from_stack: usize = command[1] - 1;
        let to_stack: usize = command[2] - 1;
        let start = (stacks[from_stack].len()) - command[0];
        let end = stacks[from_stack].len();

        let mut drained_elements: Vec<_> = stacks[from_stack].drain(start..end).collect();
        drained_elements.reverse();
        stacks[to_stack].extend(drained_elements);

    }
    let res_1: String = stacks.iter().map(|row| row.last().unwrap().to_string()).collect();
    println!("res_1: {}", res_1);
}
