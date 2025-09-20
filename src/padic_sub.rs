use std::cell::Cell;
use std::ops::Sub;
use std::rc::Rc;
use crate::discrete::Value;
use crate::padic::{PadicAccessor, PadicInteger};

impl<'a, Digit: Value + 'a> Sub for &'a PadicInteger<'a, Digit> {
    type Output = PadicInteger<'a, Digit>;

    fn sub(self, rhs: Self) -> Self::Output {
        PadicInteger::new_from_rc(Rc::new(SubtractionPadicInteger::new(self.value.clone(), rhs.value.clone())))
    }
}

impl<'a, Digit: Value + 'a> Sub for PadicInteger<'a, Digit> {
    type Output = PadicInteger<'a, Digit>;

    fn sub(self, rhs: Self) -> Self::Output {
        PadicInteger::new_from_rc(Rc::new(SubtractionPadicInteger::new(self.value, rhs.value)))
    }
}

pub(crate) struct SubtractionPadicInteger<'a, Digit: Value> {
    lhs: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
    rhs: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
    cache: Cell<(Vec<Digit>, bool)>,
}

impl<'a, Digit: Value> SubtractionPadicInteger<'a, Digit> {
    pub(crate) fn new(
        lhs: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
        rhs: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
    ) -> SubtractionPadicInteger<'a, Digit> {
        SubtractionPadicInteger {
            lhs,
            rhs,
            cache: Cell::new((vec![], false)),
        }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for SubtractionPadicInteger<'a, Digit> {
    fn get_digit(&self, index: usize) -> Digit {
        let (mut digit_cache, mut borrow) = self.cache.take();

        let mut digit = *digit_cache.get(index).unwrap_or(&Digit::zero());
        for i in digit_cache.len()..=index {
            let lhs_digit = self.lhs.get_digit(i);
            let rhs_digit = self.rhs.get_digit(i);
            let (digit_difference, digit_borrow) = lhs_digit.sub_borrow(rhs_digit);
            let (full_difference, full_borrow) =
                digit_difference.sub_borrow(Digit::from_bool(borrow));
            digit = full_difference;
            borrow = digit_borrow || full_borrow;
            digit_cache.push(digit);
        }

        self.cache.set((digit_cache, borrow));

        digit
    }
}
