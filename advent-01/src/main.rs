fn main() {
    let mut current_calories: i32 = 0;
    let mut ranking_list = vec![0];
    let mut peekable = include_str!("input").lines().peekable();
    while let Some(line) = peekable.next() {
        if line.is_empty() {
            ranking_list.push(current_calories);
            current_calories = 0;
            continue;
        }
        current_calories += line.parse::<i32>().unwrap();
        if peekable.peek().is_none() {
            ranking_list.push(current_calories);
        }
    }
    ranking_list.sort_by(|a, b| b.cmp(a));
    let result_1: i32 = ranking_list[0];
    let result_2: i32 = ranking_list[ .. 3].iter().sum();
    println!("{}", result_1.to_string());
    println!("{}", result_2.to_string());
}