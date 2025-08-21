use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait Zero {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

pub trait CarryingAdd<Rhs = Self> {
    type Output;
    type CarryOutput;

    /// Does a carrying add base `max` between `self` and `rhs`.
    /// 
    /// `max` should be at least '2', or the equivelant in whatever
    /// `Self` is.
    fn add_carry(self, rhs: Rhs, max: Self) -> (Self::Output, Self::CarryOutput);
}

pub trait Value:
    Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + CarryingAdd<Output = Self, CarryOutput = Self>
    + Copy
    + Ord
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

impl CarryingAdd for u8 {
    type Output = Self;
    type CarryOutput = Self;
    
    fn add_carry(self, rhs: Self, max: Self) -> (Self::Output, Self::CarryOutput) {
        let result_raw = self as u16 + rhs as u16;
        let carry = result_raw / max as u16;
        return ((result_raw % max as u16) as u8, carry as u8);
    }
}

impl Value for u8 {}
