use std::collections::HashMap;
use std::sync::LazyLock;

use crate::calculator::Calculator;

pub enum Command {
    Quit,
    Calculate(String),
}

pub type ParseFn = fn(&str, &str) -> Result<String, String>;

pub static TYPES: LazyLock<HashMap<&'static str, ParseFn>> =
    LazyLock::new(|| {
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
