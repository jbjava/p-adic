use crate::discrete::Value;
use crate::padic::{PadicAccessor, PadicIntegerAccessor, PadicNumber, PadicNumberScaler};
use std::cell::Cell;
use std::ops::Add;
use std::rc::Rc;

impl<'a, Digit: Value> Add for &'a PadicNumber<'a, Digit> {
    type Output = PadicNumber<'a, Digit>;

    fn add(self, rhs: Self) -> Self::Output {
        PadicNumber::new_from_rc(Rc::new(AdditivePadicIntegerScaler::new(self.clone(), rhs.clone())))
    }
}

impl<'a, Digit: Value + 'a> Add for PadicNumber<'a, Digit> {
    type Output = PadicNumber<'a, Digit>;

    fn add(self, rhs: Self) -> Self::Output {
        PadicNumber::new_from_rc(Rc::new(AdditivePadicIntegerScaler::new(self, rhs)))
    }
}

pub struct AdditivePadicIntegerScaler<'a, Digit: Value> {
    inner: AdditionPadicInteger<'a, Digit>,
    scale: isize,
    scale_adjustment: usize,
}

impl<'a, Digit: Value> AdditivePadicIntegerScaler<'a, Digit> {
    pub(crate) fn new(lhs: PadicNumber<'a, Digit>, rhs: PadicNumber<'a, Digit>) -> Self {
        let scale = lhs.get_scale().min(rhs.get_scale());
        let inner = AdditionPadicInteger::new(Rc::new(PadicNumberScaler::new(&lhs, scale)), Rc::new(PadicNumberScaler::new(&rhs, scale)));
        let mut scale_adjustment = 0;
        while inner.get_integer_digit(scale_adjustment).is_zero() && scale_adjustment <= 10 {
            scale_adjustment += 1;
        }
        Self { inner, scale, scale_adjustment }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for AdditivePadicIntegerScaler<'a, Digit> {
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

pub(crate) struct AdditionPadicInteger<'a, Digit: Value> {
    lhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    rhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    cache: Cell<(Vec<Digit>, bool)>,
}

impl<'a, Digit: Value> AdditionPadicInteger<'a, Digit> {
    pub(crate) fn new(
        lhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
        rhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    ) -> AdditionPadicInteger<'a, Digit> {
        AdditionPadicInteger {
            lhs,
            rhs,
            cache: Cell::new((vec![], false)),
        }
    }
}

impl<'a, Digit: Value + 'a> PadicIntegerAccessor<'a, Digit> for AdditionPadicInteger<'a, Digit> {
    fn get_integer_digit(&self, index: usize) -> Digit {
        let (mut digit_cache, mut carry) = self.cache.take();

        let mut digit = *digit_cache.get(index).unwrap_or(&Digit::zero());
        for i in digit_cache.len()..=index {
            let lhs_digit = self.lhs.get_integer_digit(i);
            let rhs_digit = self.rhs.get_integer_digit(i);
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
