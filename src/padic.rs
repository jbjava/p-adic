pub use crate::discrete::Value;
use std::fmt::Display;
use std::rc::Rc;

pub struct PadicInteger<'a, Digit: Value> {
    pub(crate) value: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
}

impl<'a, Digit: Value> PadicInteger<'a, Digit> {
    pub fn new_from_rc(value: Rc<dyn PadicAccessor<'a, Digit> + 'a>) -> PadicInteger<'a, Digit> {
        PadicInteger { value }
    }

    pub fn new<Accessor: PadicAccessor<'a, Digit> + 'a>(
        value: Accessor,
    ) -> PadicInteger<'a, Digit> {
        Self::new_from_rc(Rc::new(value))
    }

    pub fn as_view(&self, view_size: usize) -> PadicIntegerView<'a, Digit> {
        PadicIntegerView {
            value: self.value.clone(),
            view_size,
        }
    }
}

impl<'a, Digit: Value> Clone for PadicInteger<'a, Digit> {
    fn clone(&self) -> PadicInteger<'a, Digit> {
        PadicInteger::new_from_rc(self.value.clone())
    }
}

pub trait PadicAccessor<'a, Digit: Value>
where
    Self: 'a,
{
    fn get_digit(&self, index: usize) -> Digit;
    fn to_dyn(self) -> PadicInteger<'a, Digit>
    where
        Self: Sized,
    {
        PadicInteger::new(self)
    }
}

pub struct PadicIntegerView<'a, Digit: Value> {
    value: Rc<dyn PadicAccessor<'a, Digit> + 'a>,
    view_size: usize,
}

impl<'a, Digit: Value + 'a> Display for PadicIntegerView<'a, Digit> {
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
