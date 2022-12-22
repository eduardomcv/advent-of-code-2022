use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut accumulator = 0;
    let mut calories_list: Vec<i32> = vec![];

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            calories_list.push(accumulator);
            accumulator = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            accumulator += calories;
        }
    }

    calories_list.sort();
    let length = calories_list.len();

    let first = calories_list[length - 1];
    let second = calories_list[length - 2];
    let third = calories_list[length - 3];

    print!(
        "Top 3:\n1st: {}\n2nd: {}\n3rd: {}\n\n",
        first, second, third
    );
    println!("Calorie sum: {}", first + second + third);
}
