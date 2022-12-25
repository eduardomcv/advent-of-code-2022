use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_priority_value(c: char) -> u16 {
    let char_value = c as u16;

    let a_uppercase_value: u16 = 65;
    let a_undercase_value: u16 = 97;

    if char_value >= a_undercase_value {
        char_value - a_undercase_value + 1
    } else {
        char_value - a_uppercase_value + 27
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let lines = BufReader::new(file).lines();

    let mut priority_sum: u32 = 0;

    for line in lines {
        let line = line.unwrap();
        let line_str = line.as_str();
        let middle = line_str.len() / 2;

        let first_compartment = &line_str[..middle];
        let second_compartment = &line_str[middle..];

        let mut found_char: char = 'a';

        for char in first_compartment.chars() {
            if second_compartment.contains(char) {
                found_char = char;
                break;
            }
        }

        let priority_value = get_priority_value(found_char);
        priority_sum += priority_value as u32;
    }

    println!("priority sum: {}", priority_sum);
}
