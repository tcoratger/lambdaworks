use core::{
    fmt::Display,
    ops::{Add, BitAnd, Shr, ShrAssign},
};

pub trait IsUnsignedInteger:
    Shr<usize, Output = Self>
    + BitAnd<Output = Self>
    + ShrAssign<usize>
    + Eq
    + Ord
    + From<u16>
    + Copy
    + Display
    + Add<Self, Output = Self>
{
}

impl IsUnsignedInteger for u128 {}
impl IsUnsignedInteger for u64 {}
impl IsUnsignedInteger for u32 {}
impl IsUnsignedInteger for u16 {}
impl IsUnsignedInteger for usize {}
