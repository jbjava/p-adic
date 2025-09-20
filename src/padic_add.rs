use std::cell::Cell;
use std::ops::Add;
use std::rc::Rc;
use crate::discrete::Value;
use crate::padic::{PadicAccessor, PadicInteger};

impl<'a, Digit: Value> Add for &'a PadicInteger<'a, Digit> {
    type Output = PadicInteger<'a, Digit>;

    fn add(self, rhs: Self) -> Self::Output {
        PadicInteger::new_from_rc(Rc::new(AdditionPadicInteger::new(self.value.clone(), rhs.value.clone())))
    }
}

impl<'a, Digit: Value + 'a> Add for PadicInteger<'a, Digit> {
    type Output = PadicInteger<'a, Digit>;

    fn add(self, rhs: Self) -> Self::Output {
        PadicInteger::new_from_rc(Rc::new(AdditionPadicInteger::new(self.value, rhs.value)))
    }
}

pub(crate) struct AdditionPadicInteger<'a, Digit: Value> {
    lhs: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
    rhs: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
    cache: Cell<(Vec<Digit>, bool)>,
}

impl<'a, Digit: Value> AdditionPadicInteger<'a, Digit> {
    pub(crate) fn new(
        lhs: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
        rhs: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
    ) -> AdditionPadicInteger<'a, Digit> {
        AdditionPadicInteger {
            lhs,
            rhs,
            cache: Cell::new((vec![], false)),
        }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for AdditionPadicInteger<'a, Digit> {
    fn get_digit(&self, index: usize) -> Digit {
        let (mut digit_cache, mut carry) = self.cache.take();

        let mut digit = *digit_cache.get(index).unwrap_or(&Digit::zero());
        for i in digit_cache.len()..=index {
            let lhs_digit = self.lhs.get_digit(i);
            let rhs_digit = self.rhs.get_digit(i);
            let (digit_sum, digit_carry) = lhs_digit.add_carry(rhs_digit);
            let (full_sum, full_carry) = digit_sum.add_carry(Digit::from_bool(carry));
            digit = full_sum;
            carry = digit_carry || full_carry;
            digit_cache.push(digit);
        }

        self.cache.set((digit_cache, carry));

        digit
    }
}
