#[macro_use]
extern crate nom;
extern crate num;

use nom::{digit, multispace, IResult, ErrorKind};
use nom::Err::*;
use nom::IResult::*;
use std::fmt::{self, Formatter, Display};
use std::str::{self, FromStr};
use std::collections::HashMap;
use std::rc::Rc;
use num::complex;

mod arithmetic;

#[derive(Clone)]
pub enum Token {
    Nil,
    Int(i32),
    Float(f32),
    Complex(complex::Complex32),
    Bool(bool),
    Symbol(String),
    Char(char),
    Cons{first: Box<Token>, rest: Box<Token>},
}

pub fn from_bytes<T: FromStr>(bytes : &[u8]) ->
    Result<T, <T as std::str::FromStr>::Err> {
        T::from_str(str::from_utf8(bytes).unwrap())
    }

pub fn token_from_str(s : &[u8]) -> Result<Token, &str> {
    if s.len() > 0 {
        Ok(Token::Int(from_bytes::<i32>(s).unwrap()))
    } else {
        Err("Empty string")
    }
}

pub fn float_token(sign: f32, int_part: &[u8], float_part: Option<&[u8]>) -> Token {
    Token::Float(
        sign * (from_bytes::<f32>(int_part).unwrap() +
                match float_part {
                    Some(ref part) =>
                        from_bytes::<f32>(part).unwrap()
                        / 10.0f32.powf(part.len() as f32),
                    None => 0f32
                }))
}

named!(real<&[u8], Token>,
       chain!(sign: sign ~
              int_part: digit ~
              tag!(b".") ~
              float_part: digit?,
              || float_token(sign, int_part, float_part)));


named!(real_or_int<&[u8], Token>,
       chain!(sign: sign ~
              int_part: digit ~
              chain!(tag!(b".") ~
                     float_part: digit,
                     || float_part)? ~
              float_part: digit?,
              || float_token(sign, int_part, float_part)));

named!(real_or_int_with_space<&[u8], Token>,
       chain!(multispace? ~
              sign: sign ~
              multispace? ~
              int_part: digit ~
              chain!(tag!(b".") ~
                     float_part: digit,
                     || float_part)? ~
              float_part: digit?,
              || float_token(sign, int_part, float_part)));


named!(pub token<&[u8], Vec<Token> >,
       many1!(chain!(v:delimited!(opt!(multispace), alt!(
           complete!(delimited!(tag!("("), opt!(multispace), tag!(")"))) => {|_| Token::Nil} |
           complete!(delimited!(tag!("("), token_from_array, tag!(")"))) |
           complete!(tag!("nil")) => {|_| Token::Nil} |
           complete!(tag!("#\\space")) => {|_| Token::Char(' ')} |
           complete!(tag!("#\\newline")) => {|_| Token::Char('\n')} |

           chain!(tag!("#\\") ~ res: take!(1),
                  || Token::Char(str::from_utf8(res).unwrap().chars().nth(0).unwrap())) |

           chain!(real_part: real_or_int ~
                  imaginary_part: real_or_int_with_space ~
                  tag!("i"),
                  || match (real_part, imaginary_part) {
                      (Token::Float(v1), Token::Float(v2)) =>
                          Token::Complex(complex::Complex32::new(v1, v2)),
                      _ => panic!("Expected floating parts")}) |

           real |

           complete!(map_res!(is_a_bytes!(b"-0123456789"), // TODO: don't match strings like "-334-3434"
                              token_from_str)) |

           complete!(tag!(b"#t")) => {|_| Token::Bool(true)} |
           complete!(tag!(b"#f")) => {|_| Token::Bool(false)} |
           complete!(identifier) =>
           {|res: &[u8]| Token::Symbol(from_bytes(res).unwrap())}
           ), opt!(multispace)) ~
                     chain!(complete!(tag!(";")) ~ complete!(take_until!("\n")), || "")?,
                     || v)));

pub fn sign(input:&[u8]) -> IResult<&[u8], f32> {
    if input.len() > 0 && input[0] == '-' as u8 {
        Done(&input[1..], -1.0f32)
    } else if input.len() > 0 && input[0] == '+' as u8 {
        println!("whooo!");
        Done(&input[1..], 1.0f32)
    } else {
        Done(input, 1.0f32)
    }
}

pub fn identifier(input:&[u8]) -> IResult<&[u8], &[u8]> {
    for (idx, item) in input.iter().enumerate() {
        if *item == ' ' as u8 || *item == '\r' as u8 || *item == '\n' as u8
            || *item == '(' as u8 || *item == ')' as u8 || *item == ';' as u8 || *item == '\t' as u8 {
                if idx == 0 {
                    return Error(Position(ErrorKind::MultiSpace, input))
                } else {
                    return Done(&input[idx..], &input[0..idx])
                }
            }
    }
    Done(b"", input)
}


pub fn token_f(a : Vec<Token>) -> Token {
    array_to_list(&a.into_boxed_slice())
}

pub fn array_to_list(a : &[Token]) -> Token {
    match a.len() {
        0 => Token::Nil,
        1 => Token::Cons{first: Box::<Token>::new(a[0].clone()),
                         rest: Box::<Token>::new(Token::Nil)},
        _ => Token::Cons{first: Box::<Token>::new(a[0].clone()),
                         rest: Box::<Token>::new(array_to_list(&a[1..]))}
    }
}

named!(token_from_array <&[u8], Token>,
       map!(token,
            token_f));

impl Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.pretty_print())
    }
}

fn printlist(token : &Token) -> String {
    match *token {
        Token::Nil => "".to_string(),
        Token::Cons{ref first, ref rest} => {
            match **rest {
                Token::Nil => first.pretty_print(),
                _ => first.pretty_print() + " " + &printlist(rest)
            }
        },
        _ => {println!("List expected: {}", token); panic!()}
    }
}


impl Token {
    pub fn pretty_print(&self) -> String {
        match *self {
            Token::Nil => "nil".to_string(),
            Token::Int(v) => v.to_string(),
            Token::Float(v) => v.to_string(),
            Token::Complex(v) => v.to_string(),
            Token::Bool(v) => v.to_string(),
            Token::Symbol(ref v) => v.to_string(),
            Token::Char(v) => "#\\".to_string() + &v.to_string(),
            Token::Cons{..} => "(".to_string() +
                &printlist(self) + ")",
        }
    }
}

pub struct Environment {
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
    pub fn symbols_table(&self) -> &HashMap<String, Rc<Expression>> {
        &self.symbols_table
    }
    fn create_symbols_map() -> HashMap<String, Rc<Expression>> {
        let mut symbols_map = HashMap::<String, Rc<Expression>>::new();
        symbols_map.insert("+".to_string(), Rc::new(Expression::NativeFunction(Rc::new(arithmetic::op_add))));
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
            Expression::Complex(_) => Ok(token.clone()),
            Expression::Bool(_) => Ok(token.clone()),
            Expression::Symbol(ref v) => self.lookup_symbol(v),
            Expression::Char(_) => Ok(token.clone()),
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
                let first_evaluated = self.eval(first.clone()).unwrap();
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
pub enum Expression {
    Nil,
    Int(i32),
    Float(f32),
    Complex(complex::Complex32),
    Bool(bool),
    Symbol(String),
    Char(char),
    NativeFunction(Rc<Fn(Vec<Rc<Expression>>)->Rc<Expression>>),
    // NativeMacro(Rc<Fn(Rc<Expression>)->Rc<Expression>>),
    Cons{first: Rc<Expression>, rest: Rc<Expression>},
}

impl PartialEq for Expression {
    fn eq(&self, other: &Expression) -> bool {
        match (self.clone(), other.clone()) {
            (Expression::Nil, Expression::Nil) => true,
            (Expression::Int(v1), Expression::Int(v2)) => v1 == v2,
            (Expression::Float(v1), Expression::Float(v2)) => v1 == v2,
            (Expression::Bool(v1), Expression::Bool(v2)) => v1 == v2,
            (Expression::Symbol(v1), Expression::Symbol(v2)) => v1 == v2,
            (Expression::Char(v1), Expression::Char(v2)) => v1 == v2,
            _ => panic!(),
        }
    }
}

fn printliste(token : &Expression) -> String {
    match *token {
        Expression::Nil => "".to_string(),
        Expression::Cons{ref first, ref rest} => {
            match **rest {
                Expression::Nil => first.pretty_print(),
                _ => first.pretty_print() + " " + &printliste(rest)
            }
        },
        _ => {println!("List expected: {}", token); panic!()}
    }
}

impl Expression {
    pub fn new(exp: &Token) -> Expression {
        match *exp {
            Token::Nil => Expression::Nil,
            Token::Int(v) => Expression::Int(v),
            Token::Float(v) => Expression::Float(v),
            Token::Complex(v) => Expression::Complex(v),
            Token::Bool(v) => Expression::Bool(v),
            Token::Symbol(ref v) => Expression::Symbol(v.to_string()),
            Token::Char(v) => Expression::Char(v),
            Token::Cons{ref first, ref rest} =>
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
            Expression::Complex(v) => v.to_string(),
            Expression::Bool(v) => v.to_string(),
            Expression::Symbol(ref v) => v.to_string(),
            Expression::Char(v) => v.to_string(),
            // Expression::NativeMacro(ref v) => "#<native macro>".to_string(),
            Expression::NativeFunction(..) => "#<native function>".to_string(),
            Expression::Cons{..} => "(".to_string() +
                &printliste(self) + ")",
        }
    }
}

impl Display for Expression {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.pretty_print())
    }
}
