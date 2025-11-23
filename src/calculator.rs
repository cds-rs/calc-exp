use std::any::type_name;
use std::fmt;

use static_assertions::const_assert_eq;

use crate::format::format_binary;
use crate::traits::IntegerOps;

pub struct Calculator<T> {
    pub op1: T,
    pub op2: T,
}

// Compile-time layout assertions
// TODO: remove me
const_assert_eq!(std::mem::size_of::<Calculator<i8>>(), 2);
const_assert_eq!(std::mem::size_of::<Calculator<i16>>(), 4);
const_assert_eq!(std::mem::size_of::<Calculator<i32>>(), 8);
const_assert_eq!(std::mem::size_of::<Calculator<i64>>(), 16);
const_assert_eq!(std::mem::size_of::<Calculator<i128>>(), 32);
const_assert_eq!(std::mem::size_of::<Calculator<u8>>(), 2);
const_assert_eq!(std::mem::size_of::<Calculator<u16>>(), 4);
const_assert_eq!(std::mem::size_of::<Calculator<u32>>(), 8);
const_assert_eq!(std::mem::size_of::<Calculator<u64>>(), 16);
const_assert_eq!(std::mem::size_of::<Calculator<u128>>(), 32);

impl<T: IntegerOps + std::str::FromStr> Calculator<T> {
    pub fn parse(a: &str, b: &str) -> Result<Self, String> {
        let op1 = a
            .parse()
            .map_err(|_| format!("{} cannot hold {}", type_name::<T>(), a))?;
        let op2 = b
            .parse()
            .map_err(|_| format!("{} cannot hold {}", type_name::<T>(), b))?;
        Ok(Calculator {
            op1,
            op2,
        })
    }
}

impl<T: IntegerOps> Calculator<T> {
    pub fn add(&self) -> Option<T> {
        self.op1.checked_add(&self.op2)
    }

    pub fn sub(&self) -> Option<T> {
        self.op1.checked_sub(&self.op2)
    }

    pub fn mul(&self) -> Option<T> {
        self.op1.checked_mul(&self.op2)
    }

    pub fn div(&self) -> Option<T> {
        self.op1.checked_div(&self.op2)
    }

    pub fn and(&self) -> T {
        self.op1 & self.op2
    }

    pub fn or(&self) -> T {
        self.op1 | self.op2
    }

    pub fn xor(&self) -> T {
        self.op1 ^ self.op2
    }
}

impl<T: IntegerOps> fmt::Display for Calculator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} + {} = {:?}", self.op1, self.op2, self.add())?;
        writeln!(f, "{} - {} = {:?}", self.op1, self.op2, self.sub())?;
        writeln!(f, "{} * {} = {:?}", self.op1, self.op2, self.mul())?;
        writeln!(f, "{} / {} = {:?}", self.op1, self.op2, self.div())?;

        let bits = std::mem::size_of::<T>() * 8;
        let hex_w = std::mem::size_of::<T>() * 2 + 2;

        for (op_symbol, result) in
            [("&", self.and()), ("|", self.or()), ("^", self.xor())]
        {
            writeln!(
                f,
                "\n  {} ({:#0hw$X}) ({})\n{} {} ({:#0hw$X}) ({})\n= {} ({:#0hw$X}) ({})",
                format_binary(self.op1, bits),
                self.op1,
                self.op1,
                op_symbol,
                format_binary(self.op2, bits),
                self.op2,
                self.op2,
                format_binary(result, bits),
                result,
                result,
                hw = hex_w
            )?;
        }
        Ok(())
    }
}

impl<T: IntegerOps + std::str::FromStr> Calculator<T> {
    pub fn parse_and_display(a: &str, b: &str) -> Result<String, String> {
        let calc = Self::parse(a, b)?;
        Ok(calc.to_string())
    }
}
