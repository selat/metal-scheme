extern crate num;

use std::rc::Rc;
use std::ops::{Add,Sub,Mul};

use num::complex;

use Expression;

macro_rules! apply_op {
    ($a:expr, $b:expr, $func:expr) => {
        match ($a, $b) {
            _ => panic!("Error!")
        }
    }
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a.lt(&b) {a} else {b}
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a.gt(&b) {a} else {b}
}

macro_rules! map_op {
    ($name: ident, $op: ident, $one_argument_case: expr) => {
        pub fn $name(args: Vec<Rc<Expression>>) -> Rc<Expression> {
            if args.len() == 0 {
                panic!("This function requires non-empty list of arguments");
            }
            if args.len() == 1 {
                return Rc::new($one_argument_case((*args[0]).clone()));
            }
            let mut res = (*args[0]).clone();
            let mut id = 0;
            for a in args {
                if id == 0 {
                    id = 1;
                    continue;
                }
                res = match (res, (*a).clone()) {
                    (Expression::Int(v1), Expression::Int(v2)) => {
                        Expression::Int(v1.$op(v2))
                    },
                    (Expression::Int(v1), Expression::Float(v2)) => {
                        Expression::Float((v1 as f32).$op(v2))
                    },
                    (Expression::Int(v1), Expression::Complex(v2)) => {
                        Expression::Complex((complex::Complex32::new(v1 as f32, 0f32)).$op(v2))
                    },
                    (Expression::Float(v1), Expression::Int(v2)) => {
                        Expression::Float(v1.$op(v2 as f32))
                    },
                    (Expression::Float(v1), Expression::Float(v2)) => {
                        Expression::Float(v1.$op(v2))
                    },
                    (Expression::Float(v1), Expression::Complex(v2)) => {
                        Expression::Complex((complex::Complex32::new(v1, 0f32)).$op(v2))
                    },
                    (Expression::Complex(v1), Expression::Int(v2)) => {
                        Expression::Complex(v1.$op(complex::Complex32::new(v2 as f32, 0f32)))
                    },
                    (Expression::Complex(v1), Expression::Float(v2)) => {
                        Expression::Complex(v1.$op(complex::Complex32::new(v2, 0f32)))
                    },
                    (Expression::Complex(v1), Expression::Complex(v2)) => {
                        Expression::Complex(v1.$op(v2))
                    },
                    _ => panic!("Number expected"),
                };
            }
            Rc::new(res)
        }
    };
    ($name: ident, $op: ident) => {map_op!($name, $op, |e: Expression| e);};
}

macro_rules! map_global_op {
    ($name: ident, $op: ident, $one_argument_case: expr) => {
        pub fn $name(args: Vec<Rc<Expression>>) -> Rc<Expression> {
            if args.len() == 0 {
                panic!("This function requires non-empty list of arguments");
            }
            if args.len() == 1 {
                return Rc::new($one_argument_case((*args[0]).clone()));
            }
            let mut res = (*args[0]).clone();
            let mut id = 0;
            for a in args {
                if id == 0 {
                    id = 1;
                    continue;
                }
                res = match (res, (*a).clone()) {
                    (Expression::Int(v1), Expression::Int(v2)) => {
                        Expression::Int($op(v1, v2))
                    },
                    (Expression::Int(v1), Expression::Float(v2)) => {
                        Expression::Float($op(v1 as f32, v2))
                    },
                    (Expression::Float(v1), Expression::Int(v2)) => {
                        Expression::Float($op(v1, v2 as f32))
                    },
                    (Expression::Float(v1), Expression::Float(v2)) => {
                        Expression::Float($op(v1, v2))
                    },
                    (_, Expression::Complex(_)) => {
                        panic!("Can't compare complex numbers");
                    },
                    (Expression::Complex(_), _) => {
                        panic!("Can't compare complex numbers");
                    },
                    _ => panic!("Number expected"),
                };
            }
            Rc::new(res)
        }
    };
    ($name: ident, $op: ident) => {map_global_op!($name, $op, |e: Expression| e);};
}

map_op!(op_add, add);
map_op!(op_sub, sub, |e: Expression|
        match e {
            Expression::Int(v) => Expression::Int(-v),
            Expression::Float(v) => Expression::Float(-v),
            Expression::Complex(v) => Expression::Complex(-v),
            _ => panic!("Number expected")
        });
map_op!(op_mul, mul);
map_global_op!(op_min, min);
map_global_op!(op_max, max);

pub fn op_div(args: Vec<Rc<Expression>>) -> Rc<Expression> {
    if args.len() == 0 {
        panic!("This function requires non-empty list of arguments");
    }
    if args.len() == 1 {
        return Rc::new(match (*args[0]).clone() {
            Expression::Int(v) => Expression::Float(1f32 / (v as f32)),
            Expression::Float(v) => Expression::Float(1f32 / v),
            Expression::Complex(v) => Expression::Complex(complex::Complex32::new(1f32, 0f32) / v),
            _ => panic!("Number expected")
        });
    }
    let mut res = (*args[0]).clone();
    let mut first_iter: bool = true;
    for a in args {
        if first_iter {
            first_iter = false;
            continue;
        }
        res = match (res, (*a).clone()) {
            (Expression::Int(v1), Expression::Int(v2)) => {
                Expression::Float((v1 as f32) / (v2 as f32))
            },
            (Expression::Int(v1), Expression::Float(v2)) => {
                Expression::Float((v1 as f32) / v2)
            },
            (Expression::Int(v1), Expression::Complex(v2)) => {
                Expression::Complex(complex::Complex32::new(v1 as f32, 0f32) / v2)
            },
            (Expression::Float(v1), Expression::Int(v2)) => {
                Expression::Float(v1 / (v2 as f32))
            },
            (Expression::Float(v1), Expression::Float(v2)) => {
                Expression::Float(v1 / v2)
            },
            (Expression::Float(v1), Expression::Complex(v2)) => {
                Expression::Complex(complex::Complex32::new(v1, 0f32) / v2)
            },
            (Expression::Complex(v1), Expression::Int(v2)) => {
                Expression::Complex(v1 / complex::Complex32::new(v2 as f32, 0f32))
            },
            (Expression::Complex(v1), Expression::Float(v2)) => {
                Expression::Complex(v1 / complex::Complex32::new(v2, 0f32))
            },
            (Expression::Complex(v1), Expression::Complex(v2)) => {
                Expression::Complex(v1 / v2)
            },
            _ => panic!("Number expected"),
        };
    }
    Rc::new(res)
}


macro_rules! map_comparsion_op {
    ($name: ident, $op: ident) => {
        pub fn $name(args: Vec<Rc<Expression>>) -> Rc<Expression> {
            if args.len() <= 1 {
                panic!("This function requires at least two arguments");
            }
            let mut last = (*args[0]).clone();
            let mut id = 0;
            for a in args {
                if id == 0 {
                    id = 1;
                    continue;
                }
                match (last, (*a).clone()) {
                    (Expression::Int(v1), Expression::Int(v2)) => {
                        if !v1.$op(&v2) {
                            return Rc::new(Expression::Bool(false));
                        }
                    },
                    (Expression::Int(v1), Expression::Float(v2)) => {
                        if !(v1 as f32).$op(&v2) {
                            return Rc::new(Expression::Bool(false));
                        }
                    },
                    (Expression::Float(v1), Expression::Int(v2)) => {
                        if !v1.$op(&(v2 as f32)) {
                            return Rc::new(Expression::Bool(false));
                        }
                    },
                    (Expression::Float(v1), Expression::Float(v2)) => {
                        if !v1.$op(&v2) {
                            return Rc::new(Expression::Bool(false));
                        }
                    },
                    (_, Expression::Complex(_)) => {
                        panic!("Can't compare complex numbers");
                    },
                    (Expression::Complex(_), _) => {
                        panic!("Can't compare complex numbers");
                    },
                    _ => panic!("Number expected"),
                };
                last = (*a).clone();
            }
            return Rc::new(Expression::Bool(true));
        }
    }
}

map_comparsion_op!(op_eq, eq);
map_comparsion_op!(op_lt, lt);
map_comparsion_op!(op_le, le);
map_comparsion_op!(op_gt, gt);
map_comparsion_op!(op_ge, ge);


#[cfg(test)]
mod tests {
    use super::{op_add,op_sub,op_mul};
    use {Expression,Environment,token};

    use std::rc::Rc;
    use std::str;

    use nom::IResult;
    use num::complex;

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
        // Integer addition
        run_test("(+ 2 3)", Expression::Int(5));
        run_test("(+ 10)", Expression::Int(10));

        // Real addition
        run_test("(+ 1.45 90)", Expression::Float(91.45));

        // Complex addition
        run_test("(+ 1.45+4i +i)", Expression::Complex(complex::Complex32::new(1.45f32, 5f32)));
        run_test("(+ -i +i)", Expression::Complex(complex::Complex32::new(0f32, 0f32)));
    }

    #[test]
    fn test_op_sub() {
        // Integer subtraction
        run_test("(- 2 3)", Expression::Int(-1));
        run_test("(- 10)", Expression::Int(-10));

        // Real subtraction
        run_test("(- 1.45 90)", Expression::Float(1.45f32 - 90f32));

        // Complex subtraction
        run_test("(- 1.45+4i +i)", Expression::Complex(complex::Complex32::new(1.45f32, 3f32)));
        run_test("(- -i -i)", Expression::Complex(complex::Complex32::new(0f32, 0f32)));
        run_test("(- +i)", Expression::Complex(complex::Complex32::new(0f32, -1f32)));
    }

    #[test]
    fn test_op_mul() {
        // Integer multiplication
        run_test("(* 2 3)", Expression::Int(6));
        run_test("(* 10)", Expression::Int(10));

        // Real multiplication
        run_test("(* 1.45 90)", Expression::Float(1.45f32 * 90f32));

        // Complex multiplication
        run_test("(* 1.45+4i +i)", Expression::Complex(
            complex::Complex32::new(1.45f32, 4f32) * complex::Complex32::new(0f32, 1f32)));
        run_test("(* -i -i)", Expression::Complex(
            complex::Complex32::new(0f32, -1f32) * complex::Complex32::new(0f32, -1f32)));
    }

    #[test]
    fn test_op_div() {
        // Integer division
        run_test("(/ 3 2)", Expression::Float(3f32 / 2f32));
        run_test("(/ 10)", Expression::Float(1f32 / 10f32));

        // Real division
        run_test("(/ 1.45 90)", Expression::Float(1.45f32 / 90f32));
        run_test("(/ 0.5)", Expression::Float(1f32 / 0.5f32));

        // // Complex division
        run_test("(/ 1.45+4i +i)", Expression::Complex(
            complex::Complex32::new(1.45f32, 4f32) / complex::Complex32::new(0f32, 1f32)));
        run_test("(/ -i -i)", Expression::Complex(
            complex::Complex32::new(0f32, -1f32) / complex::Complex32::new(0f32, -1f32)));
    }

    #[test]
    fn test_op_min() {
        // Integer min
        run_test("(min 3 2)", Expression::Int(2));
        run_test("(min 10 -100 500 -345 -340)", Expression::Int(-345));

        // Real min
        run_test("(min 1.45 90)", Expression::Float(1.45f32));
        run_test("(min 0.5 -0.56 -0.559999)", Expression::Float(-0.56f32));
    }

    #[test]
    fn test_op_max() {
        // Integer max
        run_test("(max 3 2)", Expression::Int(3));
        run_test("(max 10 -100 500 -345 -340)", Expression::Int(500));

        // Real max
        run_test("(max 1.45 90)", Expression::Float(90f32));
        run_test("(max 0.5 -0.56 -0.559999 0.4999999)", Expression::Float(0.5f32));
    }

    #[test]
    fn test_op_eq() {
        run_test("(= 1 1)", Expression::Bool(true));
        run_test("(= (+ 2 2) 5)", Expression::Bool(false));

        run_test("(= 4.55555 4.55555)", Expression::Bool(true));
        run_test("(= 4.55556 4.55555)", Expression::Bool(false));
    }

    #[test]
    fn test_op_lt() {
        run_test("(< 1 2)", Expression::Bool(true));
        run_test("(< 1 1)", Expression::Bool(false));
        run_test("(< (+ 2 2) 5)", Expression::Bool(true));

        run_test("(< 4.55555 4.55555)", Expression::Bool(false));
        run_test("(< 4.55555 4.55556)", Expression::Bool(true));
    }

    #[test]
    fn test_op_le() {
        run_test("(<= 1 2)", Expression::Bool(true));
        run_test("(<= (+ 2 2) 5)", Expression::Bool(true));
        run_test("(<= (- -5) (+ (* 2 2) 1))", Expression::Bool(true));

        run_test("(<= 4.55555 4.55556)", Expression::Bool(true));
        run_test("(<= 4.55555 4.55555)", Expression::Bool(true));
        run_test("(<= 4.55556 4.55555)", Expression::Bool(false));
    }

    #[test]
    fn test_op_gt() {
        run_test("(> 1 2)", Expression::Bool(false));
        run_test("(> 2 1)", Expression::Bool(true));
        run_test("(> 5 (+ 2 2))", Expression::Bool(true));
        run_test("(> (- -5) (+ (* 2 2) 1))", Expression::Bool(false));

        run_test("(> 4.55555 4.55556)", Expression::Bool(false));
        run_test("(> 4.55555 4.55555)", Expression::Bool(false));
        run_test("(> 4.55556 4.55555)", Expression::Bool(true));
    }

    #[test]
    fn test_op_ge() {
        run_test("(>= 1 2)", Expression::Bool(false));
        run_test("(>= 2 1)", Expression::Bool(true));
        run_test("(>= 5 (+ 2 2))", Expression::Bool(true));
        run_test("(>= (- -5) (+ (* 2 2) 1))", Expression::Bool(true));

        run_test("(>= 4.55555 4.55556)", Expression::Bool(false));
        run_test("(>= 4.55555 4.55555)", Expression::Bool(true));
        run_test("(>= 4.55556 4.55555)", Expression::Bool(true));
    }
}
