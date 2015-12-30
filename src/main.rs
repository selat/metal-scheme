#[macro_use]
extern crate nom;
extern crate metal_scheme;

use std::io::{self, Read};
use std::collections::{HashMap};
use std::fmt::{self,Display};
use nom::{IResult};
use std::rc::Rc;

struct Environment {
    symbols_table: HashMap<String, Rc<Expression>>,
}

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.message)
    }
}

impl Environment {
    fn create_symbols_map() -> HashMap<String, Rc<Expression>> {
        let mut symbols_map = HashMap::<String, Rc<Expression>>::new();
        symbols_map.insert("+".to_string(), Rc::new(Expression::NativeFunction(Rc::new(op_add))));
        return symbols_map;
    }
    pub fn new() -> Environment {
        Environment {
            symbols_table: Environment::create_symbols_map(),
        }
    }
    // Symbols table lives as long as the environemnt itself
    pub fn lookup_symbol(&self, name: &String) -> Result<Rc<Expression>, RuntimeError> {
        Ok(self.symbols_table.get(name).unwrap().clone())
    }
    // 1. Environment and expression are completely different things with potentially different lifetimes
    pub fn eval(&mut self, token: Rc<Expression>) -> Result<Rc<Expression>, RuntimeError> {
        match *token {
            Expression::Nil => Ok(token.clone()),
            Expression::Int(_) => Ok(token.clone()),
            Expression::Float(_) => Ok(token.clone()),
            Expression::Bool(v) => Ok(token.clone()),
            Expression::Symbol(ref v) => self.lookup_symbol(v),
            Expression::Char(v) => Ok(token.clone()),
            //            Expression::NativeFunction(ref v) => v(
            // 1. Evaluate "first"
            // 2. Evaluate all items in "rest" and save the result in a vector of arguments
            // 3. Call evaluated "first" passing the vector of arguments
            Expression::Cons{ref first, ref rest} => {
                let mut arguments = Vec::<Rc<Expression>>::new();
                let mut cur = (*rest).clone();
                loop {
                    let next : Rc<Expression>;
                    match *cur {
                        Expression::Nil => break,
                        Expression::Cons{ref first, ref rest} => {
                            arguments.push(self.eval(first.clone()).unwrap());
                            next = (*rest).clone();
                        },
                        _ => panic!("List expected"),
                    }
                    cur = next;
                }
                // If it's a closure, we'll have to change it's environment, therefore variable is mutable
                let mut first_evaluated = self.eval(first.clone()).unwrap();
                match *first_evaluated {
                    Expression::NativeFunction(ref v) => Ok(v(arguments)),
                    _ => panic!("Error!"),
                }
            },

            _ => panic!("Unexpected token")
        }
    }
}

#[derive(Clone)]
enum Expression {
    Nil,
    Int(i32),
    Float(f32),
    Bool(bool),
    Symbol(String),
    Char(char),
    NativeFunction(Rc<Fn(Vec<Rc<Expression>>)->Rc<Expression>>),
    // NativeMacro(Rc<Fn(Rc<Expression>)->Rc<Expression>>),
    Cons{first: Rc<Expression>, rest: Rc<Expression>},
}

fn printlist(token : &Expression) -> String {
    match *token {
        Expression::Nil => "".to_string(),
        Expression::Cons{ref first, ref rest} => {
            match **rest {
                Expression::Nil => first.pretty_print(),
                _ => first.pretty_print() + " " + &printlist(rest)
            }
        },
        _ => {println!("List expected: {}", token); panic!()}
    }
}

impl Expression {
    pub fn new(exp: &metal_scheme::Token) -> Expression {
        match *exp {
            metal_scheme::Token::Nil => Expression::Nil,
            metal_scheme::Token::Int(v) => Expression::Int(v),
            metal_scheme::Token::Float(v) => Expression::Float(v),
            metal_scheme::Token::Bool(v) => Expression::Bool(v),
            metal_scheme::Token::Symbol(ref v) => Expression::Symbol(v.to_string()),
            metal_scheme::Token::Char(v) => Expression::Char(v),
            metal_scheme::Token::Cons{ref first, ref rest} =>
                Expression::Cons{
                    first: Rc::new(Expression::new(&first)),
                    rest: Rc::new(Expression::new(&rest))},
        }
    }
    pub fn pretty_print(&self) -> String {
        match *self {
            Expression::Nil => "nil".to_string(),
            Expression::Int(v) => v.to_string(),
            Expression::Float(v) => v.to_string(),
            Expression::Bool(v) => v.to_string(),
            Expression::Symbol(ref v) => v.to_string(),
            Expression::Char(v) => v.to_string(),
            // Expression::NativeMacro(ref v) => "#<native macro>".to_string(),
            Expression::NativeFunction(ref v) => "#<native function>".to_string(),
            Expression::Cons{ref first, ref rest} => "(".to_string() +
                &printlist(self) + ")",
        }
    }
}

impl Display for Expression {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.pretty_print())
    }
}

fn op_add(args: Vec<Rc<Expression>>) -> Rc<Expression> {
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
            (Expression::Int(v1), _) => {
                panic!("Shouldn't happen")
            },
            (Expression::Float(v1), _) => {
                panic!("Shouldn't happen")
            },
            _ => panic!("Number expected"),
        };
    }
    Rc::new(res)
}

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
            let exp = o.into_iter().map(|x| Rc::new(Expression::new(&x)));
            let mut env = Environment::new();
            for e in exp {
                println!("Evaluating expression ,got {}", env.eval(e).unwrap());
            }
        },
        _ => panic!("Failed to parse!"),
    }
}
