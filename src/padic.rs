use std::fmt::Display;

pub use crate::discrete::RingValue;

pub trait PadicInteger<Digit: RingValue> {
    fn get_digit(&self, index: usize) -> Digit;
    fn as_view(&self, view_size: usize) -> PadicIntegerView<'_, Digit> where Self: Sized {
        PadicIntegerView {
            value: self,
            view_size: view_size,
        }
    }
}

pub struct FinitePadicInteger<Digit: RingValue> {
    p: usize,
    digits: Vec<Digit>,
}

impl<Digit: RingValue> FinitePadicInteger<Digit> {
    pub fn new(p: usize) -> Self {
        Self::new_with_digits(p, vec![])
    }

    pub fn new_with_digits(p: usize, digits: Vec<Digit>) -> Self {
        FinitePadicInteger {
            p: p,
            digits: digits,
        }
    }
}

impl<Digit: RingValue + Display> PadicInteger<Digit> for FinitePadicInteger<Digit> {
    fn get_digit(&self, index: usize) -> Digit {
        if self.digits.len() >= index {
            Digit::zero()
        } else {
            self.digits[index]
        }
    }
}

pub struct PadicIntegerView<'a, Digit: RingValue> {
    value: &'a dyn PadicInteger<Digit>,
    view_size: usize,
}

impl<'a, Digit: RingValue + Display> Display for PadicIntegerView<'a, Digit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.view_size).rev() {
            write!(f, "{}", self.value.get_digit(i))?;
        }
        Ok(())
    }
}
