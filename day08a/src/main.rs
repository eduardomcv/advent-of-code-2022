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

fn check_visibility(grid: &Vec<Vec<u8>>, row_index: usize, column_index: usize) -> bool {
    let value = grid[row_index][column_index];
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    // check top
    let mut top_visible = true;
    for r in 0..row_index {
        if grid[r][column_index] >= value {
            top_visible = false;
            break;
        }
    }

    if top_visible {
        return true;
    }

    // check right
    let mut right_visible = true;
    for c in column_index + 1..grid_width {
        if grid[row_index][c] >= value {
            right_visible = false;
            break;
        }
    }

    if right_visible {
        return true;
    }

    // check bottom
    let mut bottom_visible = true;
    for r in row_index + 1..grid_height {
        if grid[r][column_index] >= value {
            bottom_visible = false;
            break;
        }
    }

    if bottom_visible {
        return true;
    }

    // check left
    let mut left_visible = true;
    for c in 0..column_index {
        if grid[row_index][c] >= value {
            left_visible = false;
            break;
        }
    }

    left_visible
}

fn count_visible(grid: &Vec<Vec<u8>>) -> u32 {
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let edge_count = grid_height * grid_width - (grid_height - 2) * (grid_width - 2);
    let mut total_visible: u32 = edge_count as u32;

    for i in 1..grid_height - 1 {
        for j in 1..grid_width - 1 {
            let visible = check_visibility(&grid, i, j);
            if visible {
                total_visible += 1;
            }
        }
    }

    total_visible
}

fn main() {
    let file = File::open("./src/input.txt").expect("Error opening file!");
    let grid = parse_grid(&file);

    let grid_height = grid.len();
    let grid_width = grid[0].len();
    println!("Received {} by {} grid.", grid_width, grid_height);

    let total_visible = count_visible(&grid);
    println!("Total visible: {}", total_visible);
}
