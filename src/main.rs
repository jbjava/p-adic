use crate::padic::{FinitePadicInteger, PadicError, PadicInteger, RepeatingPadicInteger};

type Padic<'a> = &'a dyn PadicInteger<'a, u8>;

pub mod discrete;
pub mod padic;

fn main() -> Result<(), PadicError> {
    let p = 7;

    let mut output: Vec<(&str, Padic)> = vec![];
    let a: Padic = &FinitePadicInteger::new_with_digits(p, vec![1, 1])?;
    output.push(("a", a));
    let b: Padic = &RepeatingPadicInteger::new_with_digits(p, vec![3])?;
    output.push(("b", b));
    let c: Padic = &(a + b)?;
    output.push(("c = a + b", c));
    let d: Padic = &(c - a)?;
    output.push(("d = c - a", d));
    let e: Padic = &RepeatingPadicInteger::new_with_digits(p, vec![1, 2])?;
    output.push(("e", e));
    let f: Padic = &(e - c)?;
    output.push(("f = e - c", f));
    let g: Padic = &(e - b)?;
    output.push(("g = e - b", g));
    let h: Padic = &(e + e)?;
    output.push(("h = e + e", h));
    let i: Padic = &(h + e)?;
    output.push(("i = h + e", i));
    let j: Padic = &(i + e)?;
    output.push(("j = i + e", j));
    let j2: Padic = &(j - e)?;
    output.push(("j2 = j - e", j2));
    let j3: Padic = &(j2 - e)?;
    output.push(("j3 = j2 - e", j3));
    let j4: Padic = &(j3 - e)?;
    output.push(("j4 = j3 - e", j4));
    let k: Padic = &FinitePadicInteger::new_with_digits(p, vec![])?;
    output.push(("k", k));
    let l: Padic = &FinitePadicInteger::new_with_digits(p, vec![1])?;
    output.push(("l", l));
    let m: Padic = &(k - l)?;
    output.push(("m = k - l", m));

    
    for (label, number) in output {
        println!("{} : {}", number.as_view(10), label);
    }

    Ok(())
}
