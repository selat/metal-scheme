extern crate nom;
extern crate metal_scheme;

use nom::IResult;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::str::{self,FromStr};

fn debug_print(&self) -> String {
    match *self {
        Token::Nil => "nil".to_string(),
        Token::Int(v) => v.to_string()+"~int",
        Token::Float(v) => v.to_string() + "~float",
        Token::Bool(v) => v.to_string(),
        Token::Symbol(ref v) => v.to_string()+"~symbol",
        Token::Char(v) => "#\\".to_string() + &v.to_string(),
        Token::Cons{ref first, ref rest} => "(".to_string() +
            &printlist(self) + ")",
    }
}

#[test]
fn it_works() {
    let mut f = File::open("./tests/parser/test-small.scm").unwrap();
    let mut buffer = vec![0; 0];
    f.read_to_end(&mut buffer).unwrap();
    // println!("Read: {}", String::from_utf8(buffer).unwrap());
    let tarray = &buffer.into_boxed_slice();
    // let mut tarray : [u8; 3] = [0; 3];
    // tarray[0] = '5' as u8;
    // tarray[1] = '4' as u8;
    // tarray[2] = '3' as u8;
    let s = metal_scheme::token(&tarray);
    for (id, c) in tarray.iter().enumerate() {
        println!("{} - {}", id, *c as u8);
    }
    match s {
        IResult::Done(_, o) => if (o.len() == 0) {println!("Empty!");} else {for x in o {println!("Parsed: {}", x)}},
        _ => panic!("Failed to parse! {}", str::from_utf8(&tarray).unwrap()),
    }
}
