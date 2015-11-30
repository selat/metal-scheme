#[macro_use]
extern crate nom;
extern crate metal_scheme;

use std::io::{self, Read};
use nom::{IResult};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let s = metal_scheme::token(buffer.as_bytes());
    match s {
        IResult::Done(_, o) => {println!("len: {}", o.len()); for x in o {println!("Parsed: {}", x)}},
        _ => println!("Failed to parse!"),
    }
}
