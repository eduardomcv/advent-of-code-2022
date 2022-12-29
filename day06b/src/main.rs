use std::collections::VecDeque;
use std::fs;

const WINDOW_LENGTH: usize = 14;

fn move_window(window: &mut VecDeque<char>, character: char) {
    window.push_back(character);

    if window.len() > WINDOW_LENGTH {
        window.pop_front();
    }
}

fn check_start_of_packet(window: &VecDeque<char>) -> bool {
    if window.len() < WINDOW_LENGTH {
        return false;
    }

    for i in 0..WINDOW_LENGTH - 1 {
        for k in i + 1..WINDOW_LENGTH {
            if window[i] == window[k] {
                return false;
            }
        }
    }

    true
}

fn find_start_of_packet_index(string: &str) -> Option<usize> {
    let mut window = VecDeque::<char>::new();

    for (index, char) in string.chars().enumerate() {
        move_window(&mut window, char);

        let is_start_of_packet = check_start_of_packet(&window);

        if is_start_of_packet {
            return Some(index);
        }
    }

    None
}

fn main() {
    let file_string = fs::read_to_string("./src/input.txt").expect("Error reading file!");

    let index =
        find_start_of_packet_index(&file_string).expect("Couldn't find the start-of-packet!");

    let characters_amount = index + 1;

    println!("Amount of characters: {}", characters_amount);
}
