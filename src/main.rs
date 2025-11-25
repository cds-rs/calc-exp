use std::io::{self, Write};

use calc::{CalculatorType, Command, get_typed_function};

fn read_command() -> Option<Command> {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    if input.starts_with(".q") {
        return Some(Command::Quit);
    }

    let mut parts = input.split_whitespace();
    let (Some(num1_plus_type), Some(num2), None) =
        (parts.next(), parts.next(), parts.next())
    else {
        if !input.is_empty() {
            println!("Usage: <num>_<type> <num>");
        }
        return None;
    };

    let (num1, type_name) =
        num1_plus_type.rsplit_once('_').unwrap_or((num1_plus_type, "u8"));

    let Ok(calc_type) = type_name.parse::<CalculatorType>() else {
        println!("Unknown type: {type_name}");
        return None;
    };

    get_typed_function(calc_type)(num1, num2)
        .map(Command::Calculate)
        .inspect_err(|e| println!("{e}"))
        .ok()
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
