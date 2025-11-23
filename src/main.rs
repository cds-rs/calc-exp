#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::LazyLock;

use memoffset::offset_of;

use calc::{Calculator, Command, ParseFn, TYPES};

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
    let _profiler = dhat::Profiler::new_heap();

    // Print type layouts
    println!("=== Type Layouts ===\n");

    macro_rules! print_calc_layout {
        ($t:ty, $name:expr) => {
            println!("Calculator<{}>:", $name);
            println!("  size:  {} bytes", std::mem::size_of::<Calculator<$t>>());
            println!("  align: {} bytes", std::mem::align_of::<Calculator<$t>>());
            println!("  op1:   offset {}, size {}", offset_of!(Calculator<$t>, op1), std::mem::size_of::<$t>());
            println!("  op2:   offset {}, size {}\n", offset_of!(Calculator<$t>, op2), std::mem::size_of::<$t>());
        };
    }

    print_calc_layout!(i8, "i8");
    print_calc_layout!(i16, "i16");
    print_calc_layout!(i32, "i32");
    print_calc_layout!(i64, "i64");
    print_calc_layout!(i128, "i128");
    print_calc_layout!(u8, "u8");
    print_calc_layout!(u16, "u16");
    print_calc_layout!(u32, "u32");
    print_calc_layout!(u64, "u64");
    print_calc_layout!(u128, "u128");

    println!("TYPES (HashMap<&str, ParseFn>):");
    println!("  size:  {} bytes", std::mem::size_of_val(&*TYPES));
    println!("  align: {} bytes\n", std::mem::align_of_val(&*TYPES));

    println!("LazyLock<HashMap<...>>:");
    println!("  size:  {} bytes", std::mem::size_of::<LazyLock<HashMap<&'static str, ParseFn>>>());
    println!("  align: {} bytes\n", std::mem::align_of::<LazyLock<HashMap<&'static str, ParseFn>>>());

    println!("Entry (&'static str, ParseFn):");
    println!("  &str size:   {} bytes", std::mem::size_of::<&'static str>());
    println!("  ParseFn size: {} bytes", std::mem::size_of::<ParseFn>());
    println!("  total per entry: {} bytes\n",
        std::mem::size_of::<&'static str>() + std::mem::size_of::<ParseFn>());

    println!("===================\n");

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
