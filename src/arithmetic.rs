use std::rc::Rc;
use std::str::{self,FromStr};

use nom::IResult;

use {Expression,Environment,token};

pub fn op_add(args: Vec<Rc<Expression>>) -> Rc<Expression> {
    if args.len() == 0 {
        panic!("\"+\" function requires non-empty list of arguments");
    }
    let mut res = Expression::Int(0);
    for a in args {
        res = match ((*a).clone(), res) {
            (Expression::Int(v1), Expression::Int(v2)) => {
                Expression::Int(v1 + v2)
            },
            (Expression::Int(v1), Expression::Float(v2)) => {
                Expression::Float(v1 as f32 + v2)
            },
            (Expression::Float(v1), Expression::Int(v2)) => {
                Expression::Float(v1 + v2 as f32)
            },
            (Expression::Float(v1), Expression::Float(v2)) => {
                Expression::Float(v1 + v2)
            },
            (Expression::Int(_), _) => {
                panic!("Shouldn't happen")
            },
            (Expression::Float(_), _) => {
                panic!("Shouldn't happen")
            },
            _ => panic!("Number expected"),
        };
    }
    Rc::new(res)
}

fn run_test(test: &'static str, expected_res: Expression) {
    let test1 = test.as_bytes();
    let s = token(&test1);
    match s {
        IResult::Done(_, o) => {
            assert!(o.len() == 1);
            let mut env = Environment::new();
            let res = env.eval(Rc::new(Expression::new(&o[0])));
            match res {
                Ok(v) => assert!(*v == expected_res),
                _ => panic!(),
            }
        },
        _ => panic!("Failed to parse! {}", str::from_utf8(&test1).unwrap()),
    }
}

#[test]
fn test_op_add() {
    run_test("(+ 2 3)", Expression::Int(5));
    run_test("(+ 10)", Expression::Int(10));
    run_test("(+ 1.45 90)", Expression::Float(91.45));
}
