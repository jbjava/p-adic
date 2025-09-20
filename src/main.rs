#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(min_generic_const_args)]
#![feature(generic_const_items)]
#![feature(associated_const_equality)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_cmp)]
use crate::discrete::AddGroupU8;
use crate::padic::{PadicInteger, PadicError, PadicAccessor};
use crate::padic_primitive::{FinitePadicInteger, RepeatingPadicInteger};

pub mod discrete;
pub mod padic;
pub mod padic_add;
pub mod padic_sub;
pub mod padic_primitive;
pub mod padic_div;
pub mod padic_mul;

fn main() -> Result<(), PadicError> {
    let mut output: Vec<(&str, &PadicInteger<AddGroupU8<7>>)> = vec![];
    let a = FinitePadicInteger::new_with_digits(new_vec(vec![1, 1])).to_dyn();
    output.push(("a", &a));
    let b = RepeatingPadicInteger::new_with_digits(new_vec(vec![3])).to_dyn();
    output.push(("b", &b));
    let c = a.clone() + b.clone();
    output.push(("c = a + b", &c));
    let d = c.clone() - a.clone();
    output.push(("d = c - a", &d));
    let e = RepeatingPadicInteger::new_with_digits(new_vec(vec![1, 2])).to_dyn();
    output.push(("e", &e));
    let f = e.clone() - c.clone();
    output.push(("f = e - c", &f));
    // let g = &e / &b;
    // output.push(("g = e / b", g));

    for (label, number) in output {
        println!("{} : {}", number.as_view(10), label);
    }

    Ok(())
}

fn new_vec<const B: u8>(vec: Vec<u8>) -> Vec<AddGroupU8<B>> {
    AddGroupU8::new_vec(vec).unwrap()
}
