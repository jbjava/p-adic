use crate::discrete::Value;
use crate::padic::{PadicAccessor, PadicIntegerAccessor, PadicNumber, PadicNumberScaler};
use std::cell::Cell;
use std::ops::Sub;
use std::rc::Rc;

impl<'a, Digit: Value> Sub for &'a PadicNumber<'a, Digit> {
    type Output = PadicNumber<'a, Digit>;

    fn sub(self, rhs: Self) -> Self::Output {
        PadicNumber::new_from_rc(Rc::new(SubtractivePadicIntegerScaler::new(self.clone(), rhs.clone())))
    }
}

impl<'a, Digit: Value + 'a> Sub for PadicNumber<'a, Digit> {
    type Output = PadicNumber<'a, Digit>;

    fn sub(self, rhs: Self) -> Self::Output {
        PadicNumber::new_from_rc(Rc::new(SubtractivePadicIntegerScaler::new(self, rhs)))
    }
}

pub struct SubtractivePadicIntegerScaler<'a, Digit: Value> {
    inner: SubtractionPadicInteger<'a, Digit>,
    scale: isize,
    scale_adjustment: usize,
}

impl<'a, Digit: Value> SubtractivePadicIntegerScaler<'a, Digit> {
    pub(crate) fn new(lhs: PadicNumber<'a, Digit>, rhs: PadicNumber<'a, Digit>) -> Self {
        let scale = lhs.get_scale().min(rhs.get_scale());
        let inner = SubtractionPadicInteger::new(Rc::new(PadicNumberScaler::new(&lhs, scale)), Rc::new(PadicNumberScaler::new(&rhs, scale)));
        let mut scale_adjustment = 0;
        while inner.get_integer_digit(scale_adjustment).is_zero() && scale_adjustment < 10 {
            scale_adjustment += 1;
        }
        Self { inner, scale, scale_adjustment }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for SubtractivePadicIntegerScaler<'a, Digit> {
    fn get_digit(&self, index: isize) -> Digit {
        let adjusted_index = index - self.scale;
        if adjusted_index < self.scale_adjustment as isize {
            Digit::zero()
        } else {
            self.inner.get_integer_digit(adjusted_index as usize)
        }
    }

    fn get_scale(&self) -> isize {
        self.scale + self.scale_adjustment as isize
    }
}

pub(crate) struct SubtractionPadicInteger<'a, Digit: Value> {
    lhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    rhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    cache: Cell<(Vec<Digit>, bool)>,
}

impl<'a, Digit: Value> SubtractionPadicInteger<'a, Digit> {
    pub(crate) fn new(
        lhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
        rhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    ) -> SubtractionPadicInteger<'a, Digit> {
        SubtractionPadicInteger {
            lhs,
            rhs,
            cache: Cell::new((vec![], false)),
        }
    }
}

impl<'a, Digit: Value + 'a> PadicIntegerAccessor<'a, Digit> for SubtractionPadicInteger<'a, Digit> {
    fn get_integer_digit(&self, index: usize) -> Digit {
        let (mut digit_cache, mut borrow) = self.cache.take();

        let mut digit = *digit_cache.get(index).unwrap_or(&Digit::zero());
        for i in digit_cache.len()..=index {
            let lhs_digit = self.lhs.get_integer_digit(i);
            let rhs_digit = self.rhs.get_integer_digit(i);
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
