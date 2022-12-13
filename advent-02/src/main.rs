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
    let mut player_points_round_1: i32 = 0;
    let mut player_points_round_2: i32 = 0;
    let mut peekable = include_str!("input").lines().peekable();
    while let Some(line) = peekable.next() {
        let mut iter = line.split_whitespace();
        let opponent = strategy_map[iter.next().unwrap()];
        let col_2 = iter.next().unwrap();
        let player = strategy_map[col_2];
        player_points_round_1 += get_points_for_round(opponent, player);
        player_points_round_2 += get_points_for_round(opponent, get_player_choice(opponent, col_2));
    }
    println!("{}", player_points_round_1);
    println!("{}", player_points_round_2);
}

fn get_player_choice(opponent: &str, val: &str) -> &'static str {
    let map = HashMap::from([
        ("rock", HashMap::from([
            ("X", "scissors"),
            ("Y", "rock"),
            ("Z", "paper"),
        ])),
        ("paper", HashMap::from([
            ("X", "rock"),
            ("Y", "paper"),
            ("Z", "scissors")
        ])),
        ("scissors", HashMap::from([
            ("X", "paper"),
            ("Y", "scissors"),
            ("Z", "rock"),
        ]))
    ]);
    return map[opponent][val];
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
