use crate::discrete::{AddGroupU8, One, Zero};
use crate::padic::{PadicAccessor, PadicNumber};
use crate::padic_primitive::{FinitePadicInteger, RepeatingPadicInteger};
use logos::Logos;
use std::collections::HashMap;
use std::iter::Map;
use std::num::{IntErrorKind, ParseIntError};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum PadicToken {
    #[token("+")]
    AdditionSign,
    #[token("-")]
    SubtractionSign,
    #[token("*")]
    MultiplicationSign,
    #[token("/")]
    DivisionSign,
    #[token("^^")]
    Square,
    #[regex(r"_*([0-9]|\([0-9]*\))+(\.([0-9]|\([0-9]*\))*)?")]
    Number,
    #[regex(r"&(\{\w*\})?")]
    Reference,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
}

pub fn parse_padic<'a, const BASE: u8>(
    string: &str,
    arguments: &HashMap<String, PadicNumber<'a, AddGroupU8<BASE>>>,
) -> Result<PadicNumber<'a, AddGroupU8<BASE>>, String> {
    let mut lex = PadicToken::lexer(string);

    let mut stack: Vec<PadicNumber<'a, AddGroupU8<BASE>>> = vec![];

    while let Some(token) = lex.next() {
        match token {
            Ok(token) => match token {
                PadicToken::AdditionSign => binary_operator(&mut stack, |a, b| a + b)?,
                PadicToken::SubtractionSign => binary_operator(&mut stack, |a, b| a - b)?,
                PadicToken::MultiplicationSign => binary_operator(&mut stack, |a, b| a * b)?,
                PadicToken::DivisionSign => binary_operator(&mut stack, |a, b| a / b)?,
                PadicToken::Square => unary_operator(&mut stack, |a| a.clone() * a)?,
                PadicToken::LeftBracket => return Err("Brackets not supported yet!".into()),
                PadicToken::RightBracket => return Err("Brackets not supported yet!".into()),
                PadicToken::Reference => {
                    let str = lex.slice();
                    let reference = if str.len() > 3 {
                        &str[2..str.len() - 1]
                    } else {
                        ""
                    };
                    match arguments.get(reference) {
                        Some(x) => stack.push(x.clone()),
                        None => return Err(format!("Unknown variable \"{}\"", reference)),
                    }
                }
                PadicToken::Number => {
                    let number_str = lex.slice();
                    enum Stage {
                        Repeating,
                        ExpectingDigit,
                        InParenthesis(Vec<u8>),
                    }
                    impl Stage {
                        fn can_accept_digit(&self) -> bool {
                            match self {
                                Self::Repeating => true,
                                Self::ExpectingDigit => true,
                                _ => false,
                            }
                        }
                    }
                    let digit_too_big = |digit: u8| -> String {
                        format!(
                            "Too big of a digit for the base! (digit: {}) (base: {})",
                            digit, BASE
                        )
                    };
                    let mut stage = Stage::Repeating;
                    let mut decimal_index = None;
                    let mut repeat_count: usize = 0;
                    let mut digit_vec: Vec<AddGroupU8<BASE>> = vec![];

                    for &char in number_str.as_bytes() {
                        if char == '_' as u8 {
                            if let Stage::Repeating = stage {
                                repeat_count += 1
                            } else {
                                return Err(format!(
                                    "Didn't expect repetition digit inside a number (in string: \"{}\")",
                                    number_str
                                ));
                            }
                        } else if char == '(' as u8 {
                            if stage.can_accept_digit() {
                                stage = Stage::InParenthesis(vec![])
                            } else {
                                return Err(format!(
                                    "Not expecting digit but found '(' (in string: \"{}\")",
                                    number_str
                                ));
                            }
                        } else if char == ')' as u8 {
                            if let Stage::InParenthesis(ref digits) = stage {
                                let mut sum: u8 = 0;
                                let mut pow = Some(1u8);
                                for digit in digits.iter().rev() {
                                    if let Some(pow) = pow
                                        && let Some(scaled) = digit.checked_mul(pow)
                                        && let Some(new_sum) = sum.checked_add(scaled)
                                    {
                                        sum = new_sum;
                                    } else {
                                        return Err(format!(
                                            "The number got too big (value was {}) (in string: \"{}\")",
                                            digits.iter().fold("".to_owned(), |acc, &v| format!(
                                                "{}{}",
                                                v, acc
                                            )),
                                            number_str
                                        ));
                                    }
                                    pow = pow.and_then(|pow| pow.checked_mul(10));
                                }
                                digit_vec
                                    .push(AddGroupU8::new(sum).ok_or_else(|| digit_too_big(sum))?);
                                stage = Stage::ExpectingDigit;
                            } else {
                                return Err(format!(
                                    "Not expecting digit but found '(' (in string: \"{}\")",
                                    number_str
                                ));
                            }
                        } else if char == '.' as u8 {
                            if let Some(_) = decimal_index {
                                return Err(format!(
                                    "Cannot have two decimal places in number (in string: \"{}\")",
                                    number_str
                                ));
                            }
                            decimal_index = Some(digit_vec.len());
                        } else {
                            let digit = char - '0' as u8;
                            if digit <= 9 {
                                if stage.can_accept_digit() {
                                    digit_vec.push(
                                        AddGroupU8::new(digit)
                                            .ok_or_else(|| digit_too_big(digit))?,
                                    );
                                    stage = Stage::ExpectingDigit;
                                } else if let Stage::InParenthesis(ref mut digits) = stage {
                                    digits.push(digit);
                                }
                            } else {
                                return Err(format!(
                                    "Unexpected number character: {} (as u8: {}) (in string: \"{}\")",
                                    char as char, char, number_str
                                ));
                            }
                        }
                    }
                    let scale = decimal_index.map(|x| digit_vec.len() - x);
                    let unscaled_number = if repeat_count == 0 {
                        digit_vec.reverse();
                        FinitePadicInteger::new_with_digits(digit_vec).to_dyn()
                    } else if repeat_count == digit_vec.len() {
                        digit_vec.reverse();
                        RepeatingPadicInteger::new_with_digits(digit_vec).to_dyn()
                    } else if repeat_count < digit_vec.len() {
                        let repeating_digit_vec =
                            digit_vec.iter().copied().take(repeat_count).rev().collect();
                        let non_repeating_digit_vec =
                            digit_vec.iter().copied().skip(repeat_count).rev().collect();
                        let mut power_vec =
                            vec![AddGroupU8::zero(); digit_vec.len() - repeat_count];
                        power_vec.push(AddGroupU8::one());
                        RepeatingPadicInteger::new_with_digits(repeating_digit_vec).to_dyn()
                            * FinitePadicInteger::new_with_digits(power_vec).to_dyn()
                            + FinitePadicInteger::new_with_digits(non_repeating_digit_vec).to_dyn()
                    } else {
                        return Err(format!(
                            "You cannot repeat more digits than you have (number: {})",
                            number_str
                        ));
                    };
                    stack.push(match scale {
                        Some(scale) => {
                            let mut power_vec = vec![AddGroupU8::zero(); scale];
                            power_vec.push(AddGroupU8::one());
                            unscaled_number
                                / FinitePadicInteger::new_with_digits(power_vec).to_dyn()
                        }
                        None => unscaled_number,
                    });
                }
            },
            Err(_) => {
                return Err(format!("Unknown token: {}", lex.slice().to_string()));
            }
        }
    }

    match stack.len() {
        1 => Ok(stack.pop().unwrap()),
        _ => Err("Wrong remaining amount on stack".into()),
    }
}

fn binary_operator<T: Clone>(stack: &mut Vec<T>, func: fn(T, T) -> T) -> Result<(), String> {
    if let Some(b) = stack.pop()
        && let Some(a) = stack.pop()
    {
        stack.push(func(a.clone(), b.clone()));
        Ok(())
    } else {
        Err("Too few on stack!".into())
    }
}

fn unary_operator<T: Clone>(stack: &mut Vec<T>, func: fn(T) -> T) -> Result<(), String> {
    if let Some(a) = stack.pop() {
        stack.push(func(a.clone()));
        Ok(())
    } else {
        Err("Too few on stack!".into())
    }
}
