use crate::padic::{FinitePadicInteger, PadicError, PadicInteger, RepeatingPadicInteger};

type Padic<'a> = &'a dyn PadicInteger<'a, 7>;

pub mod discrete;
pub mod padic;

fn main() -> Result<(), PadicError> {
    let mut output: Vec<(&str, Padic)> = vec![];
    let a: Padic = &FinitePadicInteger::new_with_digits(vec![1, 1])?;
    output.push(("a", a));
    let b: Padic = &RepeatingPadicInteger::new_with_digits(vec![3])?;
    output.push(("b", b));
    let c: Padic = &(a + b);
    output.push(("c = a + b", c));
    let d: Padic = &(c - a);
    output.push(("d = c - a", d));
    let e: Padic = &RepeatingPadicInteger::new_with_digits(vec![1, 2])?;
    output.push(("e", e));
    let f: Padic = &(e - c);
    output.push(("f = e - c", f));
    let g: Padic = &(e / b);
    output.push(("g = e / b", g));

    
    for (label, number) in output {
        println!("{} : {}", number.as_view(10), label);
    }

    Ok(())
}
