use std::{
    collections::HashMap,
    fs::File as fs_File,
    io::{BufRead, BufReader},
};

enum Command {
    Ls,
    Cd(String),
}

fn parse_command(line: &str) -> Command {
    let mut split = line.split(" ");
    let command = split.nth(1).unwrap();

    if command == "ls" {
        return Command::Ls;
    }

    let path = split.next().unwrap();

    Command::Cd(path.to_string())
}

fn parse_file_size(line: &str) -> u32 {
    line.split(" ")
        .next()
        .expect("Couldn't get file size!")
        .parse()
        .expect("Couldn't parse file size!")
}

fn build_directory_path(dir_stack: &[String]) -> String {
    let mut path = String::from(dir_stack[0].as_str());

    if dir_stack.len() < 1 {
        return path;
    }

    for i in 1..dir_stack.len() {
        path.push_str(dir_stack[i].as_str());
        path.push_str("/");
    }

    path
}

fn update_directory_size(dir_map: &mut HashMap<String, u32>, dir_path: &String, file_size: &u32) {
    let mut size = file_size.to_owned();

    if dir_map.contains_key(dir_path) {
        let previous_size = dir_map.get(dir_path).unwrap().to_owned();
        size += previous_size;
    }

    dir_map.insert(dir_path.to_owned(), size);
}

fn get_total_size(dir_map: &HashMap<String, u32>) -> u32 {
    let mut total_size: u32 = 0;

    for size in dir_map.values() {
        if size.to_owned() <= 100000 {
            total_size += size;
        }
    }

    total_size
}

fn main() {
    let file = fs_File::open("./src/input.txt").expect("Error opening file.");
    let lines = BufReader::new(file).lines();

    let mut directory_stack: Vec<String> = vec![];
    let mut directory_size_map: HashMap<String, u32> = HashMap::new();

    for line in lines {
        let line = line.unwrap();

        if line.starts_with("$") {
            let command = parse_command(&line);

            match command {
                Command::Ls => {
                    // ignore ls
                    continue;
                }
                Command::Cd(path) => {
                    if path == ".." {
                        directory_stack.pop();
                    } else {
                        directory_stack.push(path)
                    }
                }
            }
        } else if line.starts_with("dir") {
            // ignore dir lines
            continue;
        } else {
            let file_size = parse_file_size(&line);
            let dir_path = build_directory_path(&directory_stack);

            update_directory_size(&mut directory_size_map, &dir_path, &file_size);

            let parents_length = directory_stack.len() - 1;

            if parents_length > 0 {
                for i in 0..parents_length {
                    let parent_slice = &directory_stack[0..i + 1];
                    let parent_path = build_directory_path(parent_slice);

                    update_directory_size(&mut directory_size_map, &parent_path, &file_size)
                }
            }
        }
    }

    let total_size = get_total_size(&directory_size_map);
    println!("Total size: {}", total_size);
}
