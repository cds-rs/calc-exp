use std::io::{self, Write};

use calc::{Command, TYPES};

fn read_command() -> Option<Command> {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if input.starts_with(".q") {
        return Some(Command::Quit);
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    let [first, second] = parts.as_slice() else {
        if !input.is_empty() {
            println!("Usage: <num>_<type> <num>");
        }
        return None;
    };

    // Extract type suffix from first number, default to u8
    let (num1, type_name) = match first.rsplit_once('_') {
        Some((num, typ)) => (num, typ),
        None => (*first, "u8"),
    };

    let Some(parse_fn) = TYPES.get(type_name) else {
        println!("Unknown type: {}", type_name);
        return None;
    };

    match parse_fn(num1, second) {
        Ok(output) => Some(Command::Calculate(output)),
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}

fn main() {
    println!("Calcy REPL");
    println!("Usage: <num>_<type> <num>  (e.g., 10_i32 20)");
    println!("       .quit to exit");

    loop {
        let Some(cmd) = read_command() else {
            continue;
        };

        match cmd {
            Command::Quit => {
                println!("Bye!");
                break;
            }
            Command::Calculate(output) => {
                println!("{}", output);
            }
        }
    }
}
