use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

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

    /// Does a carrying addition, wrapping on the base, between `self` and `rhs`.
    ///
    /// Returns in format (value, carry?) where it 'carries' whenever it would go over base, so it
    /// just subtracts the base from the value it was going to have.
    fn add_carry(self, rhs: Rhs) -> (Self::Output, bool);
}

pub trait BorrowingSub<Rhs = Self> {
    type Output;

    /// Does a borrowing subtraction, wrapping on the base, between `self` and `rhs`.
    ///
    /// Returns in format (value, borrow?) where it 'borrows' whenever it would go negative, so it
    /// just adds the base to the negative value it was going to have.
    fn sub_borrow(self, rhs: Rhs) -> (Self::Output, bool);
}

pub trait OverflowingMul<Rhs = Self> {
    type Output;

    /// Does an overflowing multiplication, wrapping on the base, between `self` and `rhs`.
    ///
    /// Returns in format (value, overflow) where the result acts as a two-digit number in the
    /// format "`output.1` `output.0`".
    fn mul_overflow(self, rhs: Rhs) -> (Self::Output, Self::Output);
}

pub trait Invertible {
    fn is_invertible() -> bool;

    fn inverse(self) -> Self;
}

pub trait Value:
    Zero
    + One
    + Invertible
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + CarryingAdd<Output = Self>
    + BorrowingSub<Output = Self>
    + OverflowingMul<Output = Self>
    + Copy
    + Ord
    + Display
{
    fn from_bool(value: bool) -> Self {
        if value { Self::one() } else { Self::zero() }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AddGroupU8<const BASE: u8> {
    val: u8,
}

impl<const BASE: u8> AddGroupU8<BASE> {
    pub const fn new(val: u8) -> Option<Self> {
        if val < BASE {
            Some(AddGroupU8 { val })
        } else {
            None
        }
    }

    pub fn new_vec(values: Vec<u8>) -> Option<Vec<Self>> {
        values.iter().map(|x| Self::new(*x)).collect()
    }
}

impl<const BASE: u8> Zero for AddGroupU8<BASE> {
    fn zero() -> Self {
        Self::new(0).unwrap()
    }

    fn is_zero(&self) -> bool {
        self.val == 0
    }
}

impl<const BASE: u8> One for AddGroupU8<BASE> {
    fn one() -> Self {
        Self::new(1).unwrap()
    }

    fn is_one(&self) -> bool {
        self.val == 1
    }
}

impl<const BASE: u8> Invertible for AddGroupU8<BASE> {
    fn is_invertible() -> bool {
        let mut i = 2;
        while i * i <= BASE {
            if BASE.is_multiple_of(i) {
                return false;
            }
            i += 1;
        }
        true
    }

    fn inverse(self) -> Self {
        for i in 1..BASE {
            if (i as u16 * self.val as u16) % BASE as u16 == 1 {
                return Self::new(i).unwrap();
            }
        }
        panic!("Either {} isn't prime or {} (this struct) is zero", BASE, self)
    }
}

impl<const BASE: u8> Add for AddGroupU8<BASE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new((self.val + rhs.val).rem_euclid(BASE)).unwrap()
    }
}

impl<const BASE: u8> Sub for AddGroupU8<BASE> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new((self.val - rhs.val).rem_euclid(BASE)).unwrap()
    }
}

impl<const BASE: u8> Mul for AddGroupU8<BASE> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new((self.val as u16 * rhs.val as u16).rem_euclid(BASE as u16) as u8).unwrap()
    }
}

impl<const BASE: u8> Div for AddGroupU8<BASE> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.val / rhs.val).unwrap()
    }
}

impl<const BASE: u8> Rem for AddGroupU8<BASE> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.val % rhs.val).unwrap()
    }
}

impl<const BASE: u8> CarryingAdd for AddGroupU8<BASE> {
    type Output = Self;

    fn add_carry(self, rhs: Self) -> (Self::Output, bool) {
        let result_raw = self.val as u16 + rhs.val as u16;
        let carry = result_raw >= BASE as u16;
        (Self::new((result_raw % BASE as u16) as u8).unwrap(), carry)
    }
}

impl<const BASE: u8> BorrowingSub for AddGroupU8<BASE> {
    type Output = Self;

    fn sub_borrow(self, rhs: Self) -> (Self::Output, bool) {
        let result_raw = self.val as i16 - rhs.val as i16;
        let borrow = result_raw < 0;
        // we have to use a 'negative-safe' version of %
        (
            Self::new(result_raw.rem_euclid(BASE as i16) as u8).unwrap(),
            borrow,
        )
    }
}

impl<const BASE: u8> OverflowingMul for AddGroupU8<BASE> {
    type Output = Self;

    fn mul_overflow(self, rhs: Self) -> (Self::Output, Self::Output) {
        let result_raw = self.val as u16 * rhs.val as u16;
        (
            Self::new((result_raw % BASE as u16) as u8).unwrap(),
            Self::new((result_raw / BASE as u16) as u8).unwrap(),
        )
    }
}

impl<const BASE: u8> PartialOrd for AddGroupU8<BASE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BASE: u8> Ord for AddGroupU8<BASE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl<const BASE: u8> Display for AddGroupU8<BASE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if BASE > 10 {
            write!(f, "({})", self.val)
        } else {
            Display::fmt(&self.val, f)
        }
    }
}

impl<const BASE: u8> Value for AddGroupU8<BASE> {}
