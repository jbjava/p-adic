use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::padic;

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

    /// Does a carrying addition, base `base`, between `self` and `rhs`.
    /// 
    /// Returns in format (value, carry?) where it 'carries'
    /// whenever it would go over base, so it just subtracts `base``
    /// from the value it was going to have.
    /// 
    /// `self` and `rhs` must be less than `base`, and `base` should
    /// be at least '2', or the equivelant in whatever `Self` is.
    fn add_carry(self, rhs: Rhs, base: Self) -> (Self::Output, bool);
}

pub trait BorrowingSub<Rhs = Self> {
    type Output;

    /// Does a borrowing subtraction, base `base`, between `self` and `rhs`.
    /// 
    /// Returns in format (value, borrow?) where it 'borrows'
    /// whenever it would go negative, so it just adds `base` to the
    /// negative value it was going to have.
    /// 
    /// `self` and `rhs` must be less than `base`, and `base` should
    /// be at least '2', or the equivelant in whatever `Self` is.
    fn sub_borrow(self, rhs: Rhs, base: Self) -> (Self::Output, bool);
}

pub trait Value:
    Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + CarryingAdd<Output = Self>
    + BorrowingSub<Output = Self>
    + Copy
    + Ord
{
    fn from_bool(value: bool) -> Self {
        if value {
            Self::one()
        } else {
            Self::zero()
        }
    }
}

impl Zero for padic::Digit {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl One for padic::Digit {
    fn one() -> Self {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}

impl CarryingAdd for u8 {
    type Output = Self;
    
    fn add_carry(self, rhs: Self, base: Self) -> (Self::Output, bool) {
        let result_raw = self as u16 + rhs as u16;
        let carry = result_raw >= base as u16;
        return ((result_raw % base as u16) as u8, carry);
    }
}

impl CarryingAdd for u64 {
    type Output = Self;
    
    fn add_carry(self, rhs: Self, base: Self) -> (Self::Output, bool) {
        let result_raw = self as u128 + rhs as u128;
        let carry = result_raw >= base as u128;
        return ((result_raw % base as u128) as u64, carry);
    }
}

impl BorrowingSub for u8 {
    type Output = Self;
    
    fn sub_borrow(self, rhs: Self, base: Self) -> (Self::Output, bool) {
        let result_raw = self as i16 - rhs as i16;
        let borrow = result_raw < 0;
        // we have to use a 'negative-safe' version of %
        return ((result_raw.rem_euclid(base as i16)) as u8, borrow);
    }
}

impl BorrowingSub for u64 {
    type Output = Self;
    
    fn sub_borrow(self, rhs: Self, base: Self) -> (Self::Output, bool) {
        let result_raw = self as i128 - rhs as i128;
        let borrow = result_raw < 0;
        // we have to use a 'negative-safe' version of %
        return ((result_raw.rem_euclid(base as i128)) as u64, borrow);
    }
}


impl Value for padic::Digit {}
