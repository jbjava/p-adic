use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait Zero {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

pub trait Value:
    Zero + One + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self> + Copy + Ord
{
}

impl Zero for u8 {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for u8 {
    fn one() -> Self {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl Value for u8 {}
