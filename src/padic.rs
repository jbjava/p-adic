use std::{cell::Cell, fmt::Display, ops::{Add, Div, Mul, Sub}};

pub use crate::discrete::Value;
use crate::discrete::{BorrowingSub, CarryingAdd};

pub trait PadicInteger<'a, Digit: Value, const P: Digit> {
    fn get_digit(&self, index: usize) -> Digit;
    fn as_view(&'a self, view_size: usize) -> PadicIntegerView<'a, P> {
        PadicIntegerView {
            value: self.as_dyn(),
            view_size: view_size,
        }
    }
    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, Digit, P>;
    
    /// This is not meant to be called, but it provides compile-time
    /// check that P > 0 (although if P is 1, it is probably useless).
    /// A similar method may be able to be used to check if P is prime!
    fn check(&self) {
        struct Check<Digit: Value, const P: Digit>();
        impl<Digit: Value, const P: Digit> Check<P> {
            const POSITIVE_P: () = assert!(P > Digit::zero(), "P must be greater than 0");
        }
        let _ = Check::<Digit, P>::POSITIVE_P;
    }
}

impl<'a, Digit: Value, const P: Digit> Add for &'a dyn PadicInteger<'a, Digit, P> {
    type Output = AdditionPadicInteger<'a, P>;

    fn add(self, rhs: Self) -> Self::Output {
        AdditionPadicInteger::new(self, rhs)
    }
}

impl<'a, Digit: Value, const P: Digit> Sub for &'a dyn PadicInteger<'a, Digit, P> {
    type Output = SubtractionPadicInteger<'a, P>;

    fn sub(self, rhs: Self) -> Self::Output {
        SubtractionPadicInteger::new(self, rhs)
    }
}

impl<'a, Digit: Value, const P: Digit> Mul for &'a dyn PadicInteger<'a, Digit, P> {
    type Output = AdditionPadicInteger<'a, P>;

    fn mul(self, rhs: Self) -> Self::Output {
        AdditionPadicInteger::new(self, rhs)
    }
}

impl<'a, Digit: Value, const P: Digit> Div for &'a dyn PadicInteger<'a, Digit, P> {
    type Output = SubtractionPadicInteger<'a, P>;

    fn div(self, rhs: Self) -> Self::Output {
        struct Check<Digit: Value, const P: Digit>();
        impl<Digit: Value, const P: Digit> Check<Digit, P> {
            const POSITIVE_P: () = assert!(Digit::is_base_invertable(P), "P must be prime in order to use division");
        }
        let _ = Check::<P>::POSITIVE_P;
        SubtractionPadicInteger::new(self, rhs)
    }
}

struct A<T> {
    v: T,
}
pub struct FinitePadicInteger<Digit: Value, const P: Digit> {
    digits: Vec<Digit>,
}

impl<Digit: Value, const P: Digit> FinitePadicInteger<Digit, P> {
    pub fn new() -> Result<Self, PadicError> {
        Self::new_with_digits(vec![])
    }

    pub fn new_with_digits(digits: Vec<Digit>) -> Result<Self, PadicError> {
        if digits.iter().any(|digit| *digit >= P) {
            Err(PadicError::ValuesGreaterThanOrEqualToP)
        } else {
            Ok(FinitePadicInteger { digits })
        }
    }
}

impl<'a, Digit: Value, const P: Digit> PadicInteger<'a, P> for FinitePadicInteger<Digit, P> {
    fn get_digit(&self, index: usize) -> Digit {
        if index >= self.digits.len() {
            Digit::zero()
        } else {
            self.digits[index]
        }
    }

    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, P> {
        self
    }
}

pub struct RepeatingPadicInteger<Digit: Value, const P: Digit> {
    repeating_digits: Vec<Digit>,
}

impl<Digit: Value, const P: Digit> RepeatingPadicInteger<Digit, P> {
    pub fn new_with_digits(repeating_digits: Vec<Digit>) -> Result<Self, PadicError> {
        if repeating_digits.iter().any(|digit| *digit >= P) {
            Err(PadicError::ValuesGreaterThanOrEqualToP)
        } else {
            Ok(RepeatingPadicInteger { repeating_digits })
        }
    }
}

impl<'a, Digit: Value, const P: Digit> PadicInteger<'a, P> for RepeatingPadicInteger<Digit, P> {
    fn get_digit(&self, index: usize) -> Digit {
        self.repeating_digits[index % self.repeating_digits.len()]
    }

    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, P> {
        self
    }
}

pub struct AdditionPadicInteger<'a, Digit: Value, const P: Digit> {
    lhs: &'a dyn PadicInteger<'a, P>,
    rhs: &'a dyn PadicInteger<'a, P>,
    cache: Cell<(Vec<Digit>, bool)>,
}

impl<'a, Digit: Value, const P: Digit> AdditionPadicInteger<'a, Digit, P> {
    pub fn new(
        lhs: &'a dyn PadicInteger<'a, P>,
        rhs: &'a dyn PadicInteger<'a, P>,
    ) -> AdditionPadicInteger<'a, P> {
        AdditionPadicInteger {
            lhs,
            rhs,
            cache: Cell::new((vec![], false)),
        }
    }
}

impl<'a, Digit: Value, const P: Digit> PadicInteger<'a, Digit, P> for AdditionPadicInteger<'a, Digit, P> {
    fn get_digit(&self, index: usize) -> Digit {
        let (mut digit_cache, mut carry) = self.cache.take();

        let mut digit = *digit_cache.get(index).unwrap_or(&Digit::zero());
        for i in digit_cache.len()..=index {
            let lhs_digit = self.lhs.get_digit(i);
            let rhs_digit = self.rhs.get_digit(i);
            let (digit_sum, digit_carry) = lhs_digit.add_carry(rhs_digit, P);
            let (full_sum, full_carry) = digit_sum.add_carry(Digit::from_bool(carry), P);
            digit = full_sum;
            carry = digit_carry || full_carry;
            digit_cache.push(digit);
        }

        self.cache.set((digit_cache, carry));

        digit
    }

    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, P> {
        self
    }
}

pub struct SubtractionPadicInteger<'a, Digit: Value, const P: Digit> {
    lhs: &'a dyn PadicInteger<'a, P>,
    rhs: &'a dyn PadicInteger<'a, P>,
    cache: Cell<(Vec<Digit>, bool)>,
}

impl<'a, Digit: Value, const P: Digit> SubtractionPadicInteger<'a, Digit, P> {
    pub fn new(
        lhs: &'a dyn PadicInteger<'a, P>,
        rhs: &'a dyn PadicInteger<'a, P>,
    ) -> SubtractionPadicInteger<'a, P> {
        SubtractionPadicInteger {
            lhs,
            rhs,
            cache: Cell::new((vec![], false)),
        }
    }
}

impl<'a, Digit: Value, const P: Digit> PadicInteger<'a, Digit, P> for SubtractionPadicInteger<'a, Digit, P> {
    fn get_digit(&self, index: usize) -> Digit {
        let (mut digit_cache, mut borrow) = self.cache.take();

        let mut digit = *digit_cache.get(index).unwrap_or(&Digit::zero());
        for i in digit_cache.len()..=index {
            let lhs_digit = self.lhs.get_digit(i);
            let rhs_digit = self.rhs.get_digit(i);
            let (digit_difference, digit_borrow) = lhs_digit.sub_borrow(rhs_digit, P);
            let (full_difference, full_borrow) = digit_difference.sub_borrow(Digit::from_bool(borrow), P);
            digit = full_difference;
            borrow = digit_borrow || full_borrow;
            digit_cache.push(digit);
        }

        self.cache.set((digit_cache, borrow));

        digit
    }

    fn as_dyn(&'a self) -> &'a dyn PadicInteger<'a, P> {
        self
    }
}

pub struct PadicIntegerView<'a, Digit: Value, const P: Digit> {
    value: &'a dyn PadicInteger<'a, Digit, P>,
    view_size: usize,
}

impl<'a, Digit: Value, const P: Digit> Display for PadicIntegerView<'a, Digit, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.view_size).rev() {
            write!(f, "{}", self.value.get_digit(i))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum PadicError {
    ValuesGreaterThanOrEqualToP,
}
