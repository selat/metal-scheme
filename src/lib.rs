#[macro_use]
extern crate nom;

use nom::{digit, multispace, IResult, ErrorKind};
use nom::Err::*;
use nom::IResult::*;
use std::fmt::{self, Formatter, Display};
use std::str::{self, FromStr};
#[derive(Clone)]
pub enum Token {
    Nil,
    Int(i32),
    Float(f32),
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

named!(pub token<&[u8], Vec<Token> >,
       many1!(chain!(v:delimited!(opt!(multispace), alt!(
           complete!(delimited!(tag!("("), opt!(multispace), tag!(")"))) => {|_| Token::Nil} |
           complete!(delimited!(tag!("("), token_from_array, tag!(")"))) |
           complete!(tag!("nil")) => {|_| Token::Nil} |
           complete!(tag!("#\\space")) => {|_| Token::Char(' ')} |
           complete!(tag!("#\\newline")) => {|_| Token::Char('\n')} |

           chain!(tag!("#\\") ~ res: take!(1),
                  || Token::Char(str::from_utf8(res).unwrap().chars().nth(0).unwrap())) |

           delimited!(opt!(multispace), chain!(
               int_part: digit ~
                   tag!(b".") ~
                   float_part: digit,
               || Token::Float(from_bytes::<f32>(int_part).unwrap() +
                               from_bytes::<f32>(float_part).unwrap() /
                               10.0f32.powf(float_part.len() as f32))),
                      opt!(multispace)) |
           complete!(map_res!(delimited!(opt!(multispace), digit, opt!(multispace)),
                              token_from_str)) |

           complete!(tag!(b"#t")) => {|_| Token::Bool(true)} |
           complete!(tag!(b"#f")) => {|_| Token::Bool(false)} |
           complete!(identifier) =>
           {|res: &[u8]| Token::Symbol(from_bytes(res).unwrap())}
           ), opt!(multispace)) ~
                     chain!(complete!(tag!(";")) ~ complete!(take_until!("\n")), || "")?,
                     || v)));

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
            Token::Bool(v) => v.to_string(),
            Token::Symbol(ref v) => v.to_string(),
            Token::Char(v) => "#\\".to_string() + &v.to_string(),
            Token::Cons{ref first, ref rest} => "(".to_string() +
                &printlist(self) + ")",
        }
    }
}
