#![feature(str_split_whitespace_remainder)]
extern crate core;

use crate::discrete::AddGroupU8;
use crate::padic::PadicNumber;
use crate::padic_lang::parse_padic;
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, stdin};
// #![feature(generic_const_exprs)]
// #![feature(min_generic_const_args)]
// #![feature(generic_const_items)]
// #![feature(unsized_const_params)]
// #![feature(adt_const_params)]
// #![feature(associated_const_equality)]
// #![feature(const_trait_impl)]
// #![feature(const_ops)]
// #![feature(const_cmp)]

pub mod discrete;
pub mod padic;
pub mod padic_add;
pub mod padic_div;
pub mod padic_lang;
pub mod padic_mul;
pub mod padic_primitive;
pub mod padic_sub;

const BASE: u8 = 3;

fn main() -> Result<(), String> {
    let mut length = 10;
    let stdin = stdin();
    let mut saved_values: HashMap<String, PadicNumber<AddGroupU8<BASE>>> = HashMap::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(equation) => {
                if let Some(x) = equation.chars().nth(0) {
                    match x {
                        'e' => evaluate(&equation[2..], &mut saved_values, length),
                        's' => {
                            let mut split = equation[2..].split_ascii_whitespace();
                            let var = split.next();
                            let equation = split.remainder();
                            if let Some(var) = var
                                && let Some(ch) = var.chars().nth(0)
                                && 'a' <= ch
                                && ch <= 'z'
                                && let Some(equation) = equation
                            {
                                match parse_padic::<BASE>(&equation, &saved_values) {
                                    Ok(number) => {
                                        saved_values.insert(var.to_owned(), number.clone());
                                        println!(
                                            "{} = {} : {}",
                                            var,
                                            equation,
                                            number.as_view(length)
                                        )
                                    }
                                    Err(e) => println!("Error: {}", e),
                                }
                            } else {
                                println!(
                                    "Bad format, expecting: s <var name starts with lowercase letter> <expression>. Ex: s my_num 0 1 -"
                                );
                            }
                        }
                        'l' => {
                            if equation.len() > 1 && let Some(new_length) = equation[2..]
                                .split_ascii_whitespace()
                                .next()
                                .and_then(|x| x.parse().ok())
                            {
                                length = new_length
                            } else {
                                println!("Bad format, expecting: l <new length (a number)>. Ex: l 50")
                            }
                        }
                        'v' => {
                            for (name, val) in saved_values.iter() {
                                println!("{} = {}", name, val.as_view(length));
                            }
                        }
                        'q' => return Ok(()),
                        'h' => println!(
                            "Available commands: e (evaluate), s (set), l (set the length), v (list variables) q (quit), h (help (you're here!))"
                        ),
                        'a'..'z' | 'A'..'Z' => println!("Unknown command: {}, use the 'h' command for help", x),
                        _ => evaluate(&equation, &mut saved_values, length),
                    }
                } else {
                    println!("Format: <cmd> [args]. Use 'h' for help.")
                }
            }
            Err(msg) => println!("{}", msg),
        }
    }

    Ok(())
}

fn evaluate(equation: &str, saved_values: &mut HashMap<String, PadicNumber<AddGroupU8<BASE>>>, length: isize) {
    match parse_padic::<BASE>(&equation, &saved_values) {
        Ok(number) => {
            saved_values.insert("".to_owned(), number.clone());
            println!("{} : {}", &equation, number.as_view(length))
        }
        Err(e) => println!("Error: {}", e),
    }
}
