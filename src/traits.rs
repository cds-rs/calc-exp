use std::fmt;

use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, PrimInt};

pub trait IntegerOps:
    PrimInt
    + CheckedAdd
    + CheckedSub
    + CheckedMul
    + CheckedDiv
    + fmt::Debug
    + fmt::Display
    + fmt::Binary
    + fmt::UpperHex
    + Copy
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
        + fmt::UpperHex
        + Copy
{
}
