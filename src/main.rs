#[macro_use]
extern crate nom;

mod helper;

use nom::{IResult, digit, is_digit, multispace};
use helper::*;
use std::fmt::{self, Formatter, Display};
use std::io::{self, Read};
use std::str::{self, FromStr};

named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));

// Returns the remaining input and i32
named!(sign <&[u8], i32>, alt!( // alt! returns result of first succesfull parse
    tag!("-") => { |_| -1 } | // tag! matches a given byte array
    tag!("+") => { |_| 1 }
    )
       );

macro_rules! check(
    ($input:expr, $submac:ident!( $($args:tt)* )) => (

        {
            let mut failed = false;
            for idx in 0..$input.len() {
                if !$submac!($input[idx], $($args)*) {
                    failed = true;
                    break;
                }
            }
            if failed {
                nom::IResult::Error(nom::Err::Position(nom::ErrorKind::Custom(20),$input))
            } else {
                nom::IResult::Done(&b""[..], $input)
            }
        }
        );
    ($input:expr, $f:expr) => (
        check!($input, call!($f));
        );
    );

named!(pub take_4_digits, flat_map!(take!(4), check!(is_digit)));
named!(pub take_2_digits, flat_map!(take!(2), check!(is_digit)));

named!(positive_year  <&[u8], i32>, map!(call!(take_4_digits), buf_to_i32));
named!(pub year <&[u8], i32>, chain!(
    pref: opt!(sign) ~
        y:    positive_year
        ,
    || {
        pref.unwrap_or(1) * y
    }));


#[derive(Clone)]
pub enum Token {
    Nil,
    Int(i32),
    Float(f32),
    Bool(bool),
    Symbol(String),
    Cons{first: Box<Token>, rest: Box<Token>},
}

pub fn from_bytes<T: FromStr>(bytes : &[u8]) ->
    Result<T, <T as std::str::FromStr>::Err> {
    T::from_str(str::from_utf8(bytes).unwrap())
}

named!(token <&[u8], Vec<Token> >, many0!(alt!(
    delimited!(
        delimited!(opt!(multispace), tag!("("), opt!(multispace)),
        token_from_array,
        delimited!(opt!(multispace), tag!(")"), opt!(multispace))
            )
        |
    tag!("nil") => {|_| Token::Nil} |

    delimited!(opt!(multispace), chain!(
        int_part: digit ~
            tag!(".") ~
            float_part: digit,
        || Token::Float(from_bytes::<f32>(int_part).unwrap() +
                        from_bytes::<f32>(float_part).unwrap() /
                        10.0f32.powf(float_part.len() as f32))),
               opt!(multispace)) |

    delimited!(opt!(multispace), digit, opt!(multispace)) =>
    {|res: &[u8]| Token::Int(from_bytes(res).unwrap())} |

    tag!("#t") => {|_| Token::Bool(true)} |

    tag!("#f") => {|_| Token::Bool(false)} |

    delimited!(opt!(multispace),
               is_not!(b" \r\t\n()"),
               opt!(multispace)) =>
    {|res: &[u8]| Token::Symbol(from_bytes(res).unwrap())})));


pub fn token_f(a : Vec<Token>) -> Token {
    array_to_list(&a.into_boxed_slice())
}

pub fn array_to_list(a : &[Token]) -> Token {
    // println!("ok!");
    // for t in a {
    //     print!("{} ", t);
    // }
    // println!("");
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

impl Token {
    pub fn pretty_print(&self) -> String {
        match *self {
            Token::Nil => "nil".to_string(),
            Token::Int(v) => v.to_string(),
            Token::Float(v) => v.to_string(),
            Token::Bool(v) => v.to_string(),
            Token::Symbol(ref v) => v.to_string(),
            Token::Cons{ref first, ref rest} => "(".to_string() + &first.pretty_print() + " . "
                + &rest.pretty_print() + ")",
        }
    }
}

fn main() {
    println!("Hello, World");
    let mut buffer : [u8; 100] = [0; 100];
    io::stdin().read(&mut buffer).unwrap();
    let s = token(&buffer);
    match s {
        IResult::Done(_, o) => for x in o {println!("Parsed: {}", x)},
        _ => println!("Failed to parse!"),
    }
}
