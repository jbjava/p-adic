use std::fmt::Display;

pub use crate::discrete::Value;

pub trait PadicInteger<'a, Digit: Value> {
    fn get_p(&self) -> Digit;
    fn get_digit(&self, index: usize) -> Digit;
    fn as_view(&'a self, view_size: usize) -> PadicIntegerView<'a, Digit>
    where
        Self: Sized,
    {
        PadicIntegerView {
            value: self,
            view_size: view_size,
        }
    }
}

pub struct FinitePadicInteger<Digit: Value> {
    p: Digit,
    digits: Vec<Digit>,
}

impl<'a, Digit: Value> FinitePadicInteger<Digit> {
    pub fn new(p: Digit) -> Option<Self> {
        Self::new_with_digits(p, vec![])
    }

    pub fn new_with_digits(p: Digit, digits: Vec<Digit>) -> Option<Self> {
        if p.is_zero() || digits.iter().any(|digit| *digit >= p) {
            None
        } else {
            Some(FinitePadicInteger {
                p: p,
                digits: digits,
            })
        }
    }
}

impl<'a, Digit: Value + Display> PadicInteger<'a, Digit> for FinitePadicInteger<Digit> {
    fn get_p(&self) -> Digit {
        self.p
    }
    fn get_digit(&self, index: usize) -> Digit {
        if index >= self.digits.len() {
            Digit::zero()
        } else {
            self.digits[index]
        }
    }
}

pub struct PadicIntegerView<'a, Digit: Value> {
    value: &'a dyn PadicInteger<'a, Digit>,
    view_size: usize,
}

impl<'a, Digit: Value + Display> Display for PadicIntegerView<'a, Digit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.view_size).rev() {
            write!(f, "{}", self.value.get_digit(i))?;
        }
        Ok(())
    }
}
