use std::{
    cmp,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const ROPE_LENGTH: usize = 10;

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

    fn add(&self, other: &Position) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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

fn update_rope(rope: &mut Vec<Position>, motion: &Motion) {
    let head = &rope[0];

    match motion {
        Motion::Up => rope[0] = Position::new(head.x, head.y + 1),
        Motion::Right => rope[0] = Position::new(head.x + 1, head.y),
        Motion::Down => rope[0] = Position::new(head.x, head.y - 1),
        Motion::Left => rope[0] = Position::new(head.x - 1, head.y),
    };

    for i in 1..ROPE_LENGTH {
        let previous_knot = &rope[i - 1];
        let current_knot = &rope[i];

        if !current_knot.is_adjacent(&previous_knot) {
            let x_delta = previous_knot.x - current_knot.x;
            let y_delta = previous_knot.y - current_knot.y;

            let vector_x = cmp::max(-1, cmp::min(1, x_delta));
            let vector_y = cmp::max(-1, cmp::min(1, y_delta));

            let movement_vector = Position::new(vector_x, vector_y);

            rope[i] = current_knot.add(&movement_vector);
        }
    }
}

fn update_visited_positions(visited_positions: &mut HashSet<Position>, rope: &Vec<Position>) {
    let tail = rope.last().unwrap().clone();
    visited_positions.insert(tail);
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let lines = BufReader::new(file).lines();

    let mut rope: Vec<Position> = vec![Position::new(0, 0); ROPE_LENGTH];

    let mut visited_positions: HashSet<Position> = HashSet::new();
    update_visited_positions(&mut visited_positions, &rope);

    for line in lines {
        let line = line.unwrap();
        let (motion, units) = parse_motion(&line);

        for _ in 0..units {
            update_rope(&mut rope, &motion);
            update_visited_positions(&mut visited_positions, &rope);
        }
    }

    println!("Visited positions: {}", visited_positions.len());
}
