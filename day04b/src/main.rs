use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_overlap(room_numbers: &Vec<u32>) -> bool {
    let min1 = room_numbers[0];
    let max1 = room_numbers[1];
    let min2 = room_numbers[2];
    let max2 = room_numbers[3];

    if (min2 >= min1 && min2 <= max1) || (min1 >= min2 && min1 <= max2) {
        return true;
    }

    false
}

fn parse_numbers(line: &str) -> Vec<u32> {
    line.split(['-', ','])
        .map(|room| room.parse().unwrap())
        .collect()
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let lines = BufReader::new(file).lines();

    let mut total_overlaps: u32 = 0;

    for line in lines {
        let line = line.unwrap();
        let room_numbers = parse_numbers(&line);

        let has_overlap = check_overlap(&room_numbers);
        if has_overlap {
            total_overlaps += 1;
        }
    }

    println!("Total overlaps: {}", total_overlaps);
}
