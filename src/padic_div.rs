use crate::discrete::Value;
use crate::padic::{
    PadicAccessor, PadicIntegerAccessor, PadicIntegerToNumber, PadicNumber, PadicNumberScaler,
};
use crate::padic_primitive::FinitePadicInteger;
use std::cell::Cell;
use std::ops::Div;
use std::rc::Rc;

impl<'a, Digit: Value + 'a> Div for &'a PadicNumber<'a, Digit> {
    type Output = PadicNumber<'a, Digit>;

    fn div(self, rhs: Self) -> Self::Output {
        PadicNumber::new_from_rc(Rc::new(DivisivePadicIntegerScaler::new(
            self.clone(),
            rhs.clone(),
        )))
    }
}

impl<'a, Digit: Value + 'a> Div for PadicNumber<'a, Digit> {
    type Output = PadicNumber<'a, Digit>;

    fn div(self, rhs: Self) -> Self::Output {
        PadicNumber::new_from_rc(Rc::new(DivisivePadicIntegerScaler::new(self, rhs)))
    }
}

pub struct DivisivePadicIntegerScaler<'a, Digit: Value> {
    inner: DivisionPadicInteger<'a, Digit>,
    scale: isize,
}

impl<'a, Digit: Value> DivisivePadicIntegerScaler<'a, Digit> {
    fn new(lhs: PadicNumber<'a, Digit>, rhs: PadicNumber<'a, Digit>) -> Self {
        let scale = lhs.get_scale() - rhs.get_scale();
        let inner = DivisionPadicInteger::new(
            Rc::new(PadicNumberScaler::new(&lhs, lhs.get_scale())),
            Rc::new(PadicNumberScaler::new(&rhs, rhs.get_scale())),
        );
        DivisivePadicIntegerScaler { inner, scale }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for DivisivePadicIntegerScaler<'a, Digit> {
    fn get_digit(&self, index: isize) -> Digit {
        self.inner.get_digit(index - self.scale)
    }

    fn get_scale(&self) -> isize {
        self.scale + self.inner.get_scale()
    }
}

pub(crate) struct DivisionPadicInteger<'a, Digit: Value> {
    rhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    scale_adjustment: usize,
    cache: Cell<
        Option<(
            Vec<Digit>,
            Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
            usize,
        )>,
    >,
}

impl<'a, Digit: Value> DivisionPadicInteger<'a, Digit> {
    pub(crate) fn new(
        lhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
        rhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    ) -> DivisionPadicInteger<'a, Digit> {
        DivisionPadicInteger {
            rhs: rhs.clone(),
            scale_adjustment: {
                let mut scale_adjustment = 0;
                while rhs.get_integer_digit(scale_adjustment).is_zero() {
                    scale_adjustment += 1;
                    if scale_adjustment.is_multiple_of(100) {
                        println!(
                            "Might be dividing by zero! {} zeros so far...",
                            scale_adjustment,
                        );
                    }
                }
                scale_adjustment
            },

            cache: Cell::new(Some((vec![], lhs, 0))),
        }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for DivisionPadicInteger<'a, Digit> {
    fn get_digit(&self, index: isize) -> Digit {
        let (mut digit_cache, mut remaining, mut remaining_offset) = self.cache.take().unwrap();

        let result = match usize::try_from(index + self.scale_adjustment as isize) {
            Ok(adjusted_index) => {
                if adjusted_index < digit_cache.len() {
                    digit_cache[adjusted_index]
                } else {
                    for index in digit_cache.len()..=adjusted_index {
                        let next_remaining_digit = remaining.get_integer_digit(remaining_offset);
                        let d = if next_remaining_digit.is_zero() {
                            Digit::zero()
                        } else {
                            next_remaining_digit
                                * self.rhs.get_integer_digit(self.scale_adjustment).inverse()
                        };
                        if !d.is_zero() {
                            remaining = Rc::new(PadicNumberScaler::new(
                                &(PadicIntegerToNumber::new(remaining, 0).to_dyn()
                                    - (FinitePadicInteger::new_with_digits({
                                        let mut v = vec![Digit::zero(); index + 1];
                                        v[index] = d;
                                        v
                                    })
                                    .to_dyn()
                                        * PadicIntegerToNumber::new(self.rhs.clone(), self.scale_adjustment as isize).to_dyn())),
                                0,
                            ));
                        }
                        remaining_offset += 1;
                        digit_cache.push(d);
                    }
                    digit_cache[adjusted_index]
                }
            }
            Err(..) => Digit::zero(),
        };

        self.cache
            .set(Some((digit_cache, remaining, remaining_offset)));

        result
    }

    fn get_scale(&self) -> isize {
        -(self.scale_adjustment as isize)
    }
}
