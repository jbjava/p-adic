use crate::padic::{FinitePadicInteger, PadicError, PadicInteger, RepeatingPadicInteger};

pub mod discrete;
pub mod padic;

fn main() -> Result<(), PadicError> {
    let mut digits = vec![3, 3, 3, 3];
    digits.reverse();
    let num1: &dyn PadicInteger<u8> = &FinitePadicInteger::new_with_digits(4, vec![1, 1])?;
    let num2: &dyn PadicInteger<u8> = &RepeatingPadicInteger::new_with_digits(4, digits.clone())?;
    let num3: &dyn PadicInteger<u8> = &(num1 + num2)?;

    
    println!("{}", num1.as_view(100));
    println!("{}", num2.as_view(100));
    println!("{}", num3.as_view(100));

    Ok(())
}
