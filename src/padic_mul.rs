use std::cell::Cell;
use std::collections::VecDeque;
use crate::discrete::Value;
use crate::padic::{PadicAccessor, PadicIntegerAccessor, PadicNumber, PadicNumberScaler};
use std::ops::Mul;
use std::rc::Rc;

impl<'a, Digit: Value + 'a> Mul for &'a PadicNumber<'a, Digit> {
    type Output = PadicNumber<'a, Digit>;

    fn mul(self, rhs: Self) -> Self::Output {
        PadicNumber::new_from_rc(Rc::new(MultiplicativePadicIntegerScaler::new(
            self.clone(),
            rhs.clone(),
        )))
    }
}

impl<'a, Digit: Value + 'a> Mul for PadicNumber<'a, Digit> {
    type Output = PadicNumber<'a, Digit>;

    fn mul(self, rhs: Self) -> Self::Output {
        PadicNumber::new_from_rc(Rc::new(MultiplicativePadicIntegerScaler::new(
            self, rhs,
        )))
    }
}

pub struct MultiplicativePadicIntegerScaler<'a, Digit: Value> {
    inner: MultiplicationPadicInteger<'a, Digit>,
    scale: isize,
}

impl<'a, Digit: Value> MultiplicativePadicIntegerScaler<'a, Digit> {
    fn new(lhs: PadicNumber<'a, Digit>, rhs: PadicNumber<'a, Digit>) -> Self {
        let scale = lhs.get_scale() + rhs.get_scale();
        let inner = MultiplicationPadicInteger::new(Rc::new(PadicNumberScaler::new(&lhs, lhs.get_scale())), Rc::new(PadicNumberScaler::new(&rhs, rhs.get_scale())));
        MultiplicativePadicIntegerScaler { inner, scale }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for MultiplicativePadicIntegerScaler<'a, Digit> {
    fn get_digit(&self, index: isize) -> Digit {
        let adjusted_index = index - self.scale;
        if adjusted_index < 0 {
            Digit::zero()
        } else {
            self.inner.get_integer_digit(adjusted_index as usize)
        }
    }

    fn get_scale(&self) -> isize {
        self.scale
    }
}

pub(crate) struct MultiplicationPadicInteger<'a, Digit: Value> {
    lhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    rhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    cache: Cell<(VecDeque<Digit>, VecDeque<Digit>)>,
}

impl<'a, Digit: Value> MultiplicationPadicInteger<'a, Digit> {
    pub(crate) fn new(
        lhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
        rhs: Rc<dyn PadicIntegerAccessor<'a, Digit> + 'a>,
    ) -> MultiplicationPadicInteger<'a, Digit> {
        MultiplicationPadicInteger {
            lhs,
            rhs,
            cache: Cell::new((VecDeque::new(), VecDeque::new())),
        }
    }
}

impl<'a, Digit: Value + 'a> PadicIntegerAccessor<'a, Digit> for MultiplicationPadicInteger<'a, Digit> {
    fn get_integer_digit(&self, index: usize) -> Digit {
        let (mut computed, mut sum) = self.cache.take();

        for index in computed.len()..=index {
            for offset in 0..=index {
                let mul_result = self.lhs.get_integer_digit(offset).mul_overflow(self.rhs.get_integer_digit(index - offset));
                sum = add_vec(sum, VecDeque::from(vec![mul_result.0, mul_result.1]));
            }
            let digit = sum.pop_front().unwrap();
            computed.push_back(digit);
        }

        let result = computed[index];

        self.cache.set((computed, sum));

        result
    }
}

fn add_vec<Digit: Value>(lhs: VecDeque<Digit>, rhs: VecDeque<Digit>) -> VecDeque<Digit> {
    let mut index = 0;
    let mut carry = lhs;
    while let Some(&digit) = rhs.get(index) {
        if let Some(&carry_digit) = carry.get(index) {
            let (sum, mut carry_flag) = digit.add_carry(carry_digit);
            carry[index] = sum;
            let mut propagate_index = index + 1;
            while carry_flag {
                if let Some(digit) = carry.get_mut(propagate_index) {
                    (*digit, carry_flag) = digit.add_carry(Digit::one());
                } else {
                    carry.push_back(Digit::one());
                    carry_flag = false;
                }
                propagate_index += 1;
            }
        } else {
            carry.push_back(digit);
        }
        index += 1;
    }
    carry
}