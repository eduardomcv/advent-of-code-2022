use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_round_score(round_outcome: char) -> i32 {
    match round_outcome {
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    }
}

fn calculate_shape_score(opponent_shape: char, round_outcome: char) -> i32 {
    match opponent_shape {
        'A' => match round_outcome {
            'X' => 3,
            'Y' => 1,
            _ => 2,
        },
        'B' => match round_outcome {
            'X' => 1,
            'Y' => 2,
            _ => 3,
        },
        'C' => match round_outcome {
            'X' => 2,
            'Y' => 3,
            _ => 1,
        },
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
        let round_outcome = chars.nth(1).unwrap();

        let round_score = calculate_round_score(round_outcome);
        let shape_score = calculate_shape_score(opponent_shape, round_outcome);

        total_score += round_score + shape_score;
    }

    println!("Total score: {}", total_score);
}
