#[macro_use]
extern crate nom;
extern crate metal_scheme;

use std::io::{self, Read};
use std::collections::{HashMap};
use std::fmt::{self,Display};
use nom::{IResult};
use std::rc::Rc;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let s = metal_scheme::token(buffer.as_bytes());
    match s {
        IResult::Done(_, o) => {
            println!("len: {}", o.len());
            for x in &o {
                println!("Parsed: {}", x)
            }
            let exp = o.into_iter().map(|x| Rc::new(metal_scheme::Expression::new(&x)));
            let mut env = metal_scheme::Environment::new();
            for e in exp {
                println!("Evaluating expression ,got {}", env.eval(e).unwrap());
            }
        },
        _ => panic!("Failed to parse!"),
    }
}
