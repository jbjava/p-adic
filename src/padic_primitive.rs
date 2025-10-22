use crate::discrete::Value;
use crate::padic::PadicAccessor;

pub struct FinitePadicInteger<Digit: Value> {
    digits: Vec<Digit>,
}

impl<Digit: Value> FinitePadicInteger<Digit> {
    pub fn new() -> Self {
        Self::new_with_digits(vec![])
    }

    pub fn new_with_digits(digits: Vec<Digit>) -> Self {
        FinitePadicInteger { digits }
    }
}

impl<Digit: Value> Default for FinitePadicInteger<Digit> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for FinitePadicInteger<Digit> {
    fn get_digit(&self, index: isize) -> Digit {
        if index >= self.digits.len() as isize || index < 0 {
            Digit::zero()
        } else {
            self.digits[index as usize]
        }
    }

    fn get_scale(&self) -> isize {
        0
    }
}

pub struct RepeatingPadicInteger<Digit: Value> {
    repeating_digits: Vec<Digit>,
}

impl<Digit: Value> RepeatingPadicInteger<Digit> {
    pub fn new_with_digits(repeating_digits: Vec<Digit>) -> Self {
        RepeatingPadicInteger { repeating_digits }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for RepeatingPadicInteger<Digit> {
    fn get_digit(&self, index: isize) -> Digit {
        if index < 0 {
            Digit::zero()
        } else {
            self.repeating_digits[index as usize % self.repeating_digits.len()]
        }
    }

    fn get_scale(&self) -> isize {
        0
    }
}
