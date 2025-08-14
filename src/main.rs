use crate::padic::{FinitePadicInteger, PadicError, PadicInteger, RepeatingPadicInteger};

pub mod discrete;
pub mod padic;

fn main() -> Result<(), PadicError> {
    let mut digits = vec![1, 2, 3, 1];
    digits.reverse();
    let num: &dyn PadicInteger<u8> = &FinitePadicInteger::new_with_digits(4, digits.clone())?;
    let num2: &dyn PadicInteger<u8> = &RepeatingPadicInteger::new_with_digits(4, digits.clone())?;
    println!("{}", num.as_view(100));
    println!("{}", num2.as_view(100));
    println!("{}", (num + num2)?.as_view(100));

    Ok(())
}
