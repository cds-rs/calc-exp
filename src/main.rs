use std::any::type_name;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};
use std::sync::LazyLock;

use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, PrimInt};

trait IntegerOps:
    PrimInt
    + CheckedAdd
    + CheckedSub
    + CheckedMul
    + CheckedDiv
    + fmt::Debug
    + fmt::Display
    + fmt::Binary
{
}

impl<T> IntegerOps for T where
    T: PrimInt
        + CheckedAdd
        + CheckedSub
        + CheckedMul
        + CheckedDiv
        + fmt::Debug
        + fmt::Display
        + fmt::Binary
{
}

struct Calculator<T> {
    op1: T,
    op2: T,
}

impl<T: IntegerOps + std::str::FromStr> Calculator<T> {
    fn parse(a: &str, b: &str) -> Result<Self, String> {
        let op1 = a
            .parse()
            .map_err(|_| format!("{} cannot hold {}", type_name::<T>(), a))?;
        let op2 = b
            .parse()
            .map_err(|_| format!("{} cannot hold {}", type_name::<T>(), b))?;
        Ok(Calculator { op1, op2 })
    }
}

impl<T: IntegerOps> Calculator<T> {
    fn add(&self) -> Option<T> {
        self.op1.checked_add(&self.op2)
    }

    fn sub(&self) -> Option<T> {
        self.op1.checked_sub(&self.op2)
    }

    fn mul(&self) -> Option<T> {
        self.op1.checked_mul(&self.op2)
    }

    fn div(&self) -> Option<T> {
        self.op1.checked_div(&self.op2)
    }

    fn and(&self) -> T {
        self.op1 & self.op2
    }

    fn or(&self) -> T {
        self.op1 | self.op2
    }

    fn xor(&self) -> T {
        self.op1 ^ self.op2
    }
}

impl<T: IntegerOps> fmt::Display for Calculator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} + {} = {:?}", self.op1, self.op2, self.add())?;
        writeln!(f, "{} - {} = {:?}", self.op1, self.op2, self.sub())?;
        writeln!(f, "{} * {} = {:?}", self.op1, self.op2, self.mul())?;
        writeln!(f, "{} / {} = {:?}", self.op1, self.op2, self.div())?;

        let bits = std::mem::size_of::<T>() * 8 + 2;
        writeln!(f, "{:#0w$b} & {:#0w$b} = {:#0w$b}", self.op1, self.op2, self.and(), w = bits)?;
        writeln!(f, "{:#0w$b} | {:#0w$b} = {:#0w$b}", self.op1, self.op2, self.or(), w = bits)?;
        write!(f, "{:#0w$b} ^ {:#0w$b} = {:#0w$b}", self.op1, self.op2, self.xor(), w = bits)
    }
}

enum Command {
    Quit,
    Calculate(String),
}

type ParseFn = fn(&str, &str) -> Result<String, String>;

static TYPES: LazyLock<HashMap<&'static str, ParseFn>> = LazyLock::new(|| {
    HashMap::from([
        ("i8", Calculator::<i8>::parse_and_display as ParseFn),
        ("i16", Calculator::<i16>::parse_and_display as ParseFn),
        ("i32", Calculator::<i32>::parse_and_display as ParseFn),
        ("i64", Calculator::<i64>::parse_and_display as ParseFn),
        ("i128", Calculator::<i128>::parse_and_display as ParseFn),
        ("u8", Calculator::<u8>::parse_and_display as ParseFn),
        ("u16", Calculator::<u16>::parse_and_display as ParseFn),
        ("u32", Calculator::<u32>::parse_and_display as ParseFn),
        ("u64", Calculator::<u64>::parse_and_display as ParseFn),
        ("u128", Calculator::<u128>::parse_and_display as ParseFn),
    ])
});

impl<T: IntegerOps + std::str::FromStr> Calculator<T> {
    fn parse_and_display(a: &str, b: &str) -> Result<String, String> {
        let calc = Self::parse(a, b)?;
        Ok(calc.to_string())
    }
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
