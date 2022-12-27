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

fn find_common_character(first_line: &str, second_line: &str, third_line: &str) -> char {
    for char in first_line.chars() {
        if second_line.contains(char) && third_line.contains(char) {
            return char;
        }
    }

    0 as char
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let lines = BufReader::new(file).lines();

    let mut priority_sum: u32 = 0;
    let mut line_counter: u8 = 0;

    let mut first_line = String::new();
    let mut second_line = String::new();
    let mut third_line: String;

    for line in lines {
        let line = line.unwrap();

        if line_counter == 0 {
            first_line = line.clone();
            line_counter += 1;
        } else if line_counter == 1 {
            second_line = line.clone();
            line_counter += 1;
        } else if line_counter == 2 {
            third_line = line.clone();

            let found_char = find_common_character(&first_line, &second_line, &third_line);
            let priority_value = get_priority_value(found_char);
            priority_sum += priority_value as u32;

            line_counter = 0;
        }
    }

    println!("Priority sum: {}", priority_sum);
}
