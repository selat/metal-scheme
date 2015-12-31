extern crate nom;
extern crate metal_scheme;

use nom::IResult;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::str::{self,FromStr};
use std::rc::Rc;

#[test]
fn op_add() {
    let test1 = "(+ 2 3)".as_bytes();
    let s = metal_scheme::token(&test1);
    match s {
        IResult::Done(_, o) => {
            assert!(o.len() == 1);
            let mut env = metal_scheme::Environment::new();
            let res = env.eval(Rc::new(metal_scheme::Expression::new(&o[0])));
            match res {
                Ok(v) => assert!(*v == metal_scheme::Expression::Int(5)),
                _ => panic!(),
            }
        },
        _ => panic!("Failed to parse! {}", str::from_utf8(&test1).unwrap()),
    }
}
