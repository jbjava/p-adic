use std::{fmt::Display, ops::Add};

pub use crate::discrete::Value;

pub trait PadicInteger<'a, Digit: Value> {
    fn get_p(&self) -> Digit;
    fn get_digit(&self, index: usize) -> Digit;
    fn as_view(&'a self, view_size: usize) -> PadicIntegerView<'a, Digit> {
        PadicIntegerView {
            value: self.as_dyn(),
            view_size: view_size,
        }
    }
    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, Digit>;
}

impl<'a, Digit: Value> Add for &'a dyn PadicInteger<'a, Digit> {
    type Output = Result<AdditionPadicInteger<'a, Digit>, PadicError>;

    fn add(self, rhs: Self) -> Self::Output {
        AdditionPadicInteger::new(self, rhs)
    }
}

pub struct FinitePadicInteger<Digit: Value> {
    p: Digit,
    digits: Vec<Digit>,
}

impl<Digit: Value> FinitePadicInteger<Digit> {
    pub fn new(p: Digit) -> Result<Self, PadicError> {
        Self::new_with_digits(p, vec![])
    }

    pub fn new_with_digits(p: Digit, digits: Vec<Digit>) -> Result<Self, PadicError> {
        if p.is_zero() {
            Err(PadicError::TooSmallP)
        } else if digits.iter().any(|digit| *digit >= p) {
            Err(PadicError::ValuesGreaterThanOrEqualToP)
        } else {
            Ok(FinitePadicInteger { p, digits })
        }
    }
}

impl<'a, Digit: Value> PadicInteger<'a, Digit> for FinitePadicInteger<Digit> {
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

    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, Digit> {
        self
    }
}

pub struct RepeatingPadicInteger<Digit: Value> {
    p: Digit,
    repeating_digits: Vec<Digit>,
}

impl<Digit: Value> RepeatingPadicInteger<Digit> {
    pub fn new_with_digits(p: Digit, repeating_digits: Vec<Digit>) -> Result<Self, PadicError> {
        if p.is_zero() {
            Err(PadicError::TooSmallP)
        } else if repeating_digits.iter().any(|digit| *digit >= p) {
            Err(PadicError::ValuesGreaterThanOrEqualToP)
        } else {
            Ok(RepeatingPadicInteger {
                p,
                repeating_digits,
            })
        }
    }
}

impl<'a, Digit: Value> PadicInteger<'a, Digit> for RepeatingPadicInteger<Digit> {
    fn get_p(&self) -> Digit {
        self.p
    }

    fn get_digit(&self, index: usize) -> Digit {
        self.repeating_digits[index % self.repeating_digits.len()]
    }

    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, Digit> {
        self
    }
}

pub struct AdditionPadicInteger<'a, Digit: Value> {
    p: Digit,
    lhs: &'a dyn PadicInteger<'a, Digit>,
    rhs: &'a dyn PadicInteger<'a, Digit>,
}

impl<'a, Digit: Value> AdditionPadicInteger<'a, Digit> {
    pub fn new(
        lhs: &'a dyn PadicInteger<'a, Digit>,
        rhs: &'a dyn PadicInteger<'a, Digit>,
    ) -> Result<AdditionPadicInteger<'a, Digit>, PadicError> {
        if lhs.get_p() != rhs.get_p() {
            Err(PadicError::MismatchP)
        } else {
            Ok(AdditionPadicInteger {
                p: lhs.get_p(),
                lhs,
                rhs,
            })
        }
    }
}

impl<'a, Digit: Value> PadicInteger<'a, Digit> for AdditionPadicInteger<'a, Digit> {
    fn get_p(&self) -> Digit {
        self.p
    }

    fn get_digit(&self, index: usize) -> Digit {
        self.lhs.get_digit(index) + self.rhs.get_digit(index)
    }

    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, Digit> {
        self
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

#[derive(Debug)]
pub enum PadicError {
    MismatchP,
    TooSmallP,
    ValuesGreaterThanOrEqualToP,
}
