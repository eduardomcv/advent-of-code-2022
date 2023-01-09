use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

enum Instruction {
    NoOp,
    AddX(i32),
}

#[derive(Debug)]
struct InvalidInstructionError;

impl Error for InvalidInstructionError {}

impl fmt::Display for InvalidInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid instruction")
    }
}

fn parse_instruction(line: &str) -> Result<Instruction, Box<dyn Error>> {
    let split: Vec<&str> = line.split(" ").collect();
    let instruction_name = split[0];

    match instruction_name {
        "noop" => Ok(Instruction::NoOp),
        "addx" => {
            let value: i32 = split[1].parse()?;
            Ok(Instruction::AddX(value))
        }
        _ => Err(Box::new(InvalidInstructionError)),
    }
}

fn get_instruction_cycles(instruction: &Instruction) -> u32 {
    match instruction {
        Instruction::NoOp => 1,
        Instruction::AddX(_) => 2,
    }
}

fn print_pixel(cycles: i32, register: i32) {
    let mod_cycles = cycles % 40;

    if mod_cycles >= register - 1 && mod_cycles <= register + 1 {
        print!("#");
    } else {
        print!(".");
    }

    if mod_cycles == 39 {
        print!("\n");
    }
}

fn run(file: File) -> Result<(), Box<dyn Error>> {
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());

    let mut cycles = 0;
    let mut x_register = 1;

    for line in lines {
        let instruction = parse_instruction(&line)?;
        let instruction_cycles = get_instruction_cycles(&instruction);

        for _ in 0..instruction_cycles {
            print_pixel(cycles, x_register);
            cycles += 1;
        }

        if let Instruction::AddX(value) = instruction {
            x_register += value;
        }
    }

    Ok(())
}

fn main() {
    let file = File::open("./input.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file: {err}");
        process::exit(1);
    });

    if let Err(e) = run(file) {
        eprintln!("Error running program: {e}");
        process::exit(1);
    }
}
