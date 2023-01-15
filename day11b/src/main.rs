use std::{cell::RefCell, fs, process};

const NUMBER_OF_ROUNDS: u64 = 10000;

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
}

fn parse_items(line: &str) -> Vec<u64> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse_operation(line: &str) -> Box<dyn Fn(u64) -> u64> {
    let ops: Vec<&str> = line
        .split("new = old ")
        .nth(1)
        .unwrap()
        .split(" ")
        .collect();

    let operation = ops[0];
    let value = ops[1];

    match (operation, value) {
        ("+", "old") => Box::new(|old| {
            let worry = old + old;
            println!("    Worry level increases by {old} to {worry}.");
            worry
        }),
        ("+", _) => {
            let value = value.parse::<u64>().unwrap();

            return Box::new(move |old| {
                let worry = old + value;
                println!("    Worry level increases by {value} to {worry}.");
                worry
            });
        }
        ("*", "old") => Box::new(|old| {
            let worry = old * old;
            println!("    Worry level is multiplied by {old} to {worry}.");
            worry
        }),
        ("*", _) => {
            let value = value.parse::<u64>().unwrap();

            return Box::new(move |old| {
                let worry = old * value;
                println!("    Worry level is multiplied by {value} to {worry}.");
                worry
            });
        }
        _ => panic!("Invalid operation!"),
    }
}

fn parse_test<'a>(
    test: &'a str,
    true_clause: &'a str,
    false_clause: &'a str,
) -> (Box<dyn Fn(u64) -> usize>, u64) {
    let divisor: u64 = test.split(" ").last().unwrap().parse().unwrap();
    let true_value: usize = true_clause.split(" ").last().unwrap().parse().unwrap();
    let false_value: usize = false_clause.split(" ").last().unwrap().parse().unwrap();

    let test = Box::new(move |worry| {
        if worry % divisor == 0 {
            println!("    Current worry level is divisible by {divisor}.");
            true_value
        } else {
            println!("    Current worry level is not divisible by {divisor}.");
            false_value
        }
    });

    (test, divisor)
}

fn parse_monkeys(contents: String) -> (Vec<RefCell<Monkey>>, u64) {
    let lines: Vec<&str> = contents.lines().filter(|line| !line.is_empty()).collect();
    let line_chunks = lines.chunks(6);

    let mut monkeys: Vec<RefCell<Monkey>> = vec![];
    let mut modulo: u64 = 1;

    for chunk in line_chunks {
        let items = parse_items(chunk[1]);
        let operation = parse_operation(chunk[2]);
        let (test, divisor) = parse_test(chunk[3], chunk[4], chunk[5]);

        modulo *= divisor;

        monkeys.push(RefCell::new(Monkey {
            operation,
            test,
            items,
        }));
    }

    (monkeys, modulo)
}

fn run(contents: String) {
    let (monkeys, modulo) = parse_monkeys(contents);
    let mut inspection_counters: Vec<u64> = vec![0; monkeys.len()];

    for round in 1..=NUMBER_OF_ROUNDS {
        for monkey_index in 0..monkeys.len() {
            let mut monkey = monkeys[monkey_index].borrow_mut();
            println!("Monkey {monkey_index}:");

            let items = &monkey.items;

            for item_index in 0..items.len() {
                let item = items[item_index];
                println!("  Monkey inspects an item with a worry level of {item}.");
                inspection_counters[monkey_index] += 1;

                let operation = &monkey.operation;
                let worry = operation(item) % modulo;
                println!("    Worry is managed to {worry}. Keep it cool!");

                let test = &monkey.test;
                let throw_index = test(worry);
                let mut other_monkey = monkeys[throw_index].borrow_mut();
                other_monkey.items.push(worry);
                println!("    Item with worry level {worry} is thrown to monkey {throw_index}.");
            }

            monkey.items = vec![];
        }

        print!("\nAfter round {round}, the monkeys are holding items with these worry levels:\n");

        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys[monkey_index].borrow();

            let items = monkey
                .items
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            println!("Monkey {monkey_index}: {items}");
        }

        print!("\n");
    }

    for (index, counter) in inspection_counters.iter().enumerate() {
        println!("Monkey {index} inspected items {counter} times.");
    }

    inspection_counters.sort();

    let monkey_business: u64 = inspection_counters[inspection_counters.len() - 1]
        * inspection_counters[inspection_counters.len() - 2];

    println!("The level of monkey business is {}.", monkey_business);
}

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap_or_else(|err| {
        eprintln!("Error reading file: {err}");
        process::exit(1);
    });

    run(file_contents);
}
