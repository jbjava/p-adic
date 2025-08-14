use std::ops::{Add, Mul, Neg, Sub};

pub trait Zero {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

pub trait RingValue:
    PartialEq
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + Copy {

}