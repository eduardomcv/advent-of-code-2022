use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_grid(file: &File) -> Vec<Vec<u8>> {
    let lines = BufReader::new(file).lines();
    let mut grid: Vec<Vec<u8>> = vec![];

    for (line_index, line) in lines.enumerate() {
        let line = line.unwrap();

        grid.push(vec![]);

        for char in line.chars() {
            let value: u8 = char.try_into().expect("Error converting character!");
            grid[line_index].push(value);
        }
    }

    grid
}

fn calculate_scenic_score(grid: &Vec<Vec<u8>>, row_index: usize, column_index: usize) -> u32 {
    let value = grid[row_index][column_index];
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    // check top
    let mut top_score = 0;
    for r in (0..row_index).rev() {
        top_score += 1;
        if grid[r][column_index] >= value {
            break;
        }
    }

    // check right
    let mut right_score = 0;
    for c in column_index + 1..grid_width {
        right_score += 1;
        if grid[row_index][c] >= value {
            break;
        }
    }

    // check bottom
    let mut bottom_score = 0;
    for r in row_index + 1..grid_height {
        bottom_score += 1;
        if grid[r][column_index] >= value {
            break;
        }
    }

    // check left
    let mut left_score = 0;
    for c in (0..column_index).rev() {
        left_score += 1;
        if grid[row_index][c] >= value {
            break;
        }
    }

    top_score * right_score * bottom_score * left_score
}

fn get_highest_scenic_score(grid: &Vec<Vec<u8>>) -> u32 {
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut highest_score: u32 = 0;

    for i in 0..grid_height {
        for j in 0..grid_width {
            let scenic_score = calculate_scenic_score(&grid, i, j);
            if highest_score == 0 || scenic_score > highest_score {
                highest_score = scenic_score;
            }
        }
    }

    highest_score
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let grid = parse_grid(&file);

    let scenic_score = get_highest_scenic_score(&grid);
    println!("Highest score: {}", scenic_score);
}
