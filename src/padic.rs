pub use crate::discrete::Value;
use std::fmt::Display;
use std::rc::Rc;

pub struct PadicNumber<'a, Digit: Value> {
    pub(crate) value: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
}

impl<'a, Digit: Value + 'a> PadicNumber<'a, Digit> {
    pub fn new_from_rc(value: Rc<dyn PadicAccessor<'a, Digit> + 'a>) -> PadicNumber<'a, Digit> {
        PadicNumber { value }
    }

    pub fn new<Accessor: PadicAccessor<'a, Digit> + 'a>(
        value: Accessor,
    ) -> PadicNumber<'a, Digit> {
        Self::new_from_rc(Rc::new(value))
    }

    pub fn as_view(&self, view_size: isize) -> PadicNumberView<'a, Digit> {
        PadicNumberView {
            value: self.value.clone(),
            view_size,
        }
    }

    pub fn get_scale(&self) -> isize {
        self.value.get_scale()
    }
}

impl<'a, Digit: Value> Clone for PadicNumber<'a, Digit> {
    fn clone(&self) -> PadicNumber<'a, Digit> {
        PadicNumber::new_from_rc(self.value.clone())
    }
}

pub trait PadicAccessor<'a, Digit: Value + 'a>
where
    Self: 'a,
{
    fn get_digit(&self, index: isize) -> Digit;
    fn get_scale(&self) -> isize;
    fn to_dyn(self) -> PadicNumber<'a, Digit>
    where
        Self: Sized,
    {
        PadicNumber::new(self)
    }
}

pub trait PadicIntegerAccessor<'a, Digit: Value + 'a>
where
    Self: 'a,
{
    fn get_integer_digit(&self, index: usize) -> Digit;
}

pub(crate) struct PadicNumberScaler<'a, Digit: Value> {
    inner: PadicNumber<'a, Digit>,
    scale: isize,
}

impl<'a, Digit: Value + 'a> PadicNumberScaler<'a, Digit> {
    pub(crate) fn new(number: &PadicNumber<'a, Digit>, scale: isize) -> PadicNumberScaler<'a, Digit> {
        PadicNumberScaler {
            inner: number.clone(),
            scale,
        }
    }
}

impl<'a, Digit: Value + 'a> PadicIntegerAccessor<'a, Digit> for PadicNumberScaler<'a, Digit> {
    fn get_integer_digit(&self, index: usize) -> Digit {
        self.inner.value.get_digit(index as isize + self.scale)
    }
}

pub(crate) struct PadicIntegerToNumber<'a, Digit: Value> {
    inner: Rc<dyn PadicIntegerAccessor<'a, Digit>>,
    scale: isize,
}

impl<'a, Digit: Value + 'a> PadicIntegerToNumber<'a, Digit> {
    pub(crate) fn new(inner: Rc<dyn PadicIntegerAccessor<'a, Digit>>, scale: isize) -> Self {
        PadicIntegerToNumber { inner, scale }
    }
}

impl<'a, Digit: Value + 'a> PadicAccessor<'a, Digit> for PadicIntegerToNumber<'a, Digit> {
    fn get_digit(&self, index: isize) -> Digit {
        if index + self.scale < 0 {
            Digit::zero()
        } else {
            self.inner.get_integer_digit((index + self.scale) as usize)
        }
    }

    fn get_scale(&self) -> isize {
        self.scale
    }
}

pub struct PadicNumberView<'a, Digit: Value> {
    value: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
    view_size: isize,
}

impl<'a, Digit: Value + 'a> Display for PadicNumberView<'a, Digit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.view_size).rev() {
            write!(f, "{}", self.value.get_digit(i))?;
        }
        if self.value.get_scale() < 0 {
            let mut digit_buffer = vec![];
            for i in self.value.get_scale()..0 {
                let value = self.value.get_digit(i);
                if !(digit_buffer.is_empty() && value.is_zero()) {
                    digit_buffer.push(value);
                }
            }
            if digit_buffer.len() > 0 {
                write!(f, ".")?;
                for digit in digit_buffer {
                    write!(f, "{}", digit)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum PadicError {
    ValuesGreaterThanOrEqualToP,
}
