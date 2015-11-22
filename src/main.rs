#[macro_use]
extern crate nom;

use nom::{IResult, digit, multispace};
use std::fmt::{self, Formatter, Display};
use std::io::{self, Read};
use std::str::{self, FromStr};


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
    let mut buffer : [u8; 100] = [0; 100];
    io::stdin().read(&mut buffer).unwrap();
    let s = token(&buffer);
    match s {
        IResult::Done(_, o) => for x in o {println!("Parsed: {}", x)},
        _ => println!("Failed to parse!"),
    }
}
