use crate::padic::{FinitePadicInteger, PadicInteger};

pub mod discrete;
pub mod padic;

fn main() {
    let num: FinitePadicInteger<u8> = FinitePadicInteger::new_with_digits(4, vec![1, 2, 3]).unwrap();
    println!("{}", num.as_view(3));
}
