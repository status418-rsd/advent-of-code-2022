use std::collections::HashMap;

fn main() {
    let strategy_map = HashMap::from([
        ("A", "rock"),
        ("X", "rock"),
        ("B", "paper"),
        ("Y", "paper"),
        ("C", "scissors"),
        ("Z", "scissors"),
    ]);
    let mut player_points: i32 = 0;
    let mut cout: i32 = 0;
    let mut peekable = include_str!("input").lines().peekable();
    while let Some(line) = peekable.next() {
        let mut iter = line.split_whitespace();
        player_points += get_points_for_round(strategy_map[iter.next().unwrap()], strategy_map[iter.next().unwrap()]);
        cout += 1;
    }
    println!("{}", player_points);
}

fn get_points_for_round(opponent: &str, player: &str) -> i32 {
    let points = HashMap::from([
        ("rock", 1),
        ("paper", 2),
        ("scissors", 3),
    ]);
    let map = HashMap::from([
        ("rock", HashMap::from([
            ("paper", 0),
            ("scissors", 6),
            ("rock", 3)
        ])),
        ("paper", HashMap::from([
            ("paper", 3),
            ("scissors", 0),
            ("rock", 6)
        ])),
        ("scissors", HashMap::from([
            ("paper", 6),
            ("scissors", 3),
            ("rock", 0)
        ]))
    ]);
    return map[player][opponent] + points[player];
}