use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut max_found = 0;
    let mut accumulator = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            if accumulator > max_found {
                max_found = accumulator;
            }
            accumulator = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            accumulator += calories;
        }
    }

    println!("Maximum calories: {}", max_found);
}
