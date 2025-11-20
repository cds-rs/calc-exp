use std::io::{self, Write};

enum Command {
    Quit,
    Calculate { type_name: String, first: String, second: String },
}

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

    Some(Command::Calculate {
        type_name: type_name.to_string(),
        first: num1.to_string(),
        second: second.to_string(),
    })
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
            Command::Calculate { type_name, first, second } => {
                println!("type: {}, first: {}, second: {}", type_name, first, second);
            }
        }
    }
}
