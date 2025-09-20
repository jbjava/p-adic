use crate::discrete::Value;
use crate::padic::PadicAccessor;

pub struct FinitePadicInteger<Digit: Value> {
    digits: Vec<Digit>,
}

impl<'a, Digit: Value> FinitePadicInteger<Digit> {
    pub fn new() -> Self {
        Self::new_with_digits(vec![])
    }

    pub fn new_with_digits(digits: Vec<Digit>) -> Self {
        FinitePadicInteger { digits }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for FinitePadicInteger<Digit> {
    fn get_digit(&self, index: usize) -> Digit {
        if index >= self.digits.len() {
            Digit::zero()
        } else {
            self.digits[index]
        }
    }
}

pub struct RepeatingPadicInteger<Digit: Value> {
    repeating_digits: Vec<Digit>,
}

impl<'a, Digit: Value> RepeatingPadicInteger<Digit> {
    pub fn new_with_digits(repeating_digits: Vec<Digit>) -> Self {
        RepeatingPadicInteger { repeating_digits }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for RepeatingPadicInteger<Digit> {
    fn get_digit(&self, index: usize) -> Digit {
        self.repeating_digits[index % self.repeating_digits.len()]
    }
}
