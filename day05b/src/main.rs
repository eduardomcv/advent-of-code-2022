use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_move_command(line: &str) -> (u32, usize, usize) {
    let word_vec = line.split(" ").collect::<Vec<&str>>();

    let amount: u32 = word_vec[1].parse().unwrap();
    let from_index: usize = word_vec[3].parse::<usize>().unwrap() - 1;
    let to_index: usize = word_vec[5].parse::<usize>().unwrap() - 1;

    (amount, from_index, to_index)
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let lines = BufReader::new(file).lines();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let mut should_read_header = true;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            should_read_header = false;
            continue;
        }

        if should_read_header {
            for (index, char) in line.chars().enumerate() {
                let char_value = char as u16;

                if char_value < 0x41 || char_value > 0x5A {
                    continue;
                }

                let stack_index = (index - 1) / 4;

                stacks[stack_index].insert(0, char);
            }
        } else {
            let (amount, from, to) = parse_move_command(&line);
            let mut temp_vec: Vec<char> = vec![];

            for _ in 0..amount {
                let item = stacks[from].pop();

                match item {
                    Some(item) => temp_vec.push(item),
                    None => println!("Couldn't pop item from stack {}", from),
                }
            }

            for _ in 0..amount {
                let item = temp_vec.pop();

                match item {
                    Some(item) => stacks[to].push(item),
                    None => println!("Couldn't pop item from temporary vector"),
                }
            }
        }
    }

    print!("Top crates: ");
    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    print!("\n");
}
