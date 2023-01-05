use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

enum Motion {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_adjacent(&self, other: &Position) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

fn parse_motion(line: &str) -> (Motion, u32) {
    let split: Vec<&str> = line.split(" ").collect();
    let motion = split[0];
    let units: u32 = split[1].parse().expect("Error parsing motion units!");

    let motion = match motion {
        "U" => Motion::Up,
        "R" => Motion::Right,
        "D" => Motion::Down,
        "L" => Motion::Left,
        _ => panic!("Invalid motion!"),
    };

    (motion, units)
}

fn move_head(head: &Position, motion: &Motion) -> Position {
    match motion {
        Motion::Up => Position::new(head.x, head.y + 1),
        Motion::Right => Position::new(head.x + 1, head.y),
        Motion::Down => Position::new(head.x, head.y - 1),
        Motion::Left => Position::new(head.x - 1, head.y),
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let lines = BufReader::new(file).lines();

    let mut head: Position = Position::new(0, 0);
    let mut tail: Position = Position::new(0, 0);
    let mut visited_positions: HashSet<Position> = HashSet::new();

    visited_positions.insert(tail.clone());

    for line in lines {
        let line = line.unwrap();
        let (motion, units) = parse_motion(&line);

        for _ in 0..units {
            let new_head = move_head(&head, &motion);

            if !tail.is_adjacent(&new_head) {
                tail = head.clone();
                visited_positions.insert(tail.clone());
            }

            head = new_head;
        }
    }

    println!("Visited positions: {}", visited_positions.len());
}
