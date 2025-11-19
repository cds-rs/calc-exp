use std::fmt;

// TRAITS...

pub trait AdditiveOperations<T> {
    fn add(&self) -> Option<T>;
    fn sub(&self) -> Option<T>;
}

pub trait MultiplicativeOperations<T> {
    fn div(&self) -> Option<T>;
    fn mul(&self) -> Option<T>;
}

pub trait BinaryOperations<T> {
    fn and(&self) -> Option<T>;
    fn or(&self) -> Option<T>;
    fn xor(&self) -> Option<T>;
}

#[derive(Debug)]
struct Calculator<T>
where
    T: Default,
    T: PartialEq,
{
    lhs: T,
    rhs: T,
}

impl<T> fmt::Display for Calculator<T>
where
    T: std::ops::Add<Output = T>
        + Copy
        + Default
        + PartialEq
        + std::fmt::Debug
        + std::fmt::Display
        + std::ops::BitAnd<Output = T>
        + std::ops::BitOr<Output = T>
        + std::ops::BitXor<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Calculator({}, {})", self.lhs, self.rhs)?;
        writeln!(f, "Sum c: {:?}", self.add())?;
        writeln!(f, "Dif c: {:?}", self.sub())?;
        writeln!(f, "Div c: {:?}", self.div())?;
        writeln!(f, "Mul c: {:?}", self.mul())?;
        writeln!(f, "And c: {:?}", self.and())?;
        writeln!(f, "Or  c: {:?}", self.or())?;
        write!(f, "Xor c: {:?}", self.xor())
    }
}

// T must implement Add/Sub for arithmetic, and Copy to allow copying values from &self
// The Add and Sub trait
// pub trait Add<Rhs = Self> {
//   type Output;    <-- an associated type that could be anything
//   fn add(self, rhs:Rhs) -> Self::Output   <-- "
// }
//
// So we have to declare the Output = Ttype in the trait bounds because our methods return
// Option<T>. Without this, the compiler can't guarnantee that (T +/- T) produces a T.
//
impl<T> AdditiveOperations<T> for Calculator<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + Copy
        + Default
        + PartialEq,
{
    fn add(&self) -> Option<T> {
        Some(self.lhs + self.rhs) // copy needed here
    }

    fn sub(&self) -> Option<T> {
        Some(self.lhs - self.rhs) // copy needed here
    }
}

impl<T> MultiplicativeOperations<T> for Calculator<T>
where
    T: std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + Copy
        + Default
        + PartialEq,
{
    fn div(&self) -> Option<T> {
        // PartialEq needed for equality check
        // Default needed to check for Zero (all numbers, have zero as their default)
        if self.rhs == T::default() {
            return None;
        }

        Some(self.lhs / self.rhs)
    }

    fn mul(&self) -> Option<T> {
        Some(self.lhs * self.rhs)
    }
}

impl<
    T: std::ops::BitAnd<Output = T>
        + std::ops::BitOr<Output = T>
        + std::ops::BitXor<Output = T>
        + PartialEq
        + Default
        + Copy,
> BinaryOperations<T> for Calculator<T>
{
    fn and(&self) -> Option<T> {
        Some(self.lhs & self.rhs)
    }
    fn or(&self) -> Option<T> {
        Some(self.lhs | self.rhs)
    }
    fn xor(&self) -> Option<T> {
        Some(self.lhs ^ self.rhs)
    }
}

fn main() {
    let c = Calculator {
        lhs: 13,
        rhs: 4,
    };
    println!("Calculator: {}", c);
}
