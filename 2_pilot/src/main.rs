use std::io;
use std::io::BufRead;


fn main() {
    let Input { commands } = Input::read_stdin();
    // println!("{:?}", commands);

    let position = determine_position(commands.as_slice());
    println!("{:?}", position);
    println!("{}", position.horizontal * position.depth);

    let position = determine_position_aim(commands.as_slice());
    println!("{:?}", position);
    println!("{}", position.horizontal * position.depth);
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32)
}

impl Command {
    fn from_input(input: &str) -> Self {
        let words: Vec<&str> = input.trim().split_whitespace().collect();
        let value: u32 = match words[1].parse() {
            Ok(num) => num,
            Err(_) => panic!("Cannot parse value {}", input),
        };
        match words[0] {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => panic!("Cannot parse command {}", input)
        }
    }
}

#[derive(Debug)]
struct Input {
    commands: Vec<Command>
}

impl Input {
    fn read_stdin() -> Self {
        let mut commands: Vec<Command> = Vec::new();

        for line in io::stdin().lock().lines() {
            let input = line.expect("error: unable to read user input");
            let command = Command::from_input(&input);
            commands.push(command);
        }

        Input {
            commands
        }
    }
}

#[derive(Debug)]
struct Position {
    horizontal: i64,
    depth: i64,
}

fn determine_position(commands: &[Command]) -> Position {
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;

    for command in commands {
        match command {
            Command::Down(value) => depth += *value as i64,
            Command::Up(value) => depth -= *value as i64,
            Command::Forward(value) => horizontal += *value as i64,
        }
    }

    Position {
        horizontal,
        depth,
    }
}

fn determine_position_aim(commands: &[Command]) -> Position {
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    for command in commands {
        match command {
            Command::Down(value) => aim += *value as i64,
            Command::Up(value) => aim -= *value as i64,
            Command::Forward(value) => {
                horizontal += *value as i64;
                depth += aim * *value as i64;
            },
        }
    }

    Position {
        horizontal,
        depth,
    }
}
