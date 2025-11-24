use strum::EnumString;

pub enum Command {
    Quit,
    Calculate(String),
}

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum CalculatorType {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
}

pub type ParseFn = fn(&str, &str) -> Result<String, String>;
