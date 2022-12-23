use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_round_score(first_char: char, last_char: char) -> i32 {
    match first_char {
        'A' => match last_char {
            'X' => 3,
            'Y' => 6,
            _ => 0,
        },
        'B' => match last_char {
            'Y' => 3,
            'Z' => 6,
            _ => 0,
        },
        'C' => match last_char {
            'X' => 6,
            'Z' => 3,
            _ => 0,
        },
        _ => 0,
    }
}

fn calculate_shape_score(shape: char) -> i32 {
    match shape {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let lines = BufReader::new(file).lines();

    let mut total_score = 0;

    for line in lines {
        let line = line.unwrap();

        if line.len() == 0 {
            continue;
        }

        let mut chars = line.chars();

        let opponent_shape = chars.nth(0).unwrap();
        let player_shape = chars.nth(1).unwrap();

        let round_score = calculate_round_score(opponent_shape, player_shape);
        let shape_score = calculate_shape_score(player_shape);

        total_score += round_score + shape_score;
    }

    println!("Total score: {}", total_score);
}
