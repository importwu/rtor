mod parser;
mod error;
pub mod char;
pub mod combinator;
mod input;
mod state;

pub use self::{
    error::{
        SimpleError,
        ParseError
    },
    parser::Parser,
    input::{
        Input,
    },
    state::{
        State,
        Pos
    }
};

pub type ParseResult<O, I, E = SimpleError<<I as Input>::Token>> = Result<(O, I), E>;

pub trait AsChar {
    fn as_char(&self) -> char;
}

impl AsChar for u8 {
    fn as_char(&self) -> char {
        *self as char
    }
}

impl AsChar for char {
    fn as_char(&self) -> char {
        *self
    }
}

pub trait FindToken<T> {

    fn find_token(&self, token: &T) -> bool;
}

impl<'a> FindToken<char> for &'a str {
    fn find_token(&self, token: &char) -> bool {
        self.chars().any(|x| x == *token)
    }
}

impl<'a> FindToken<u8> for &'a str {
    fn find_token(&self, token: &u8) -> bool {
        self.chars().any(|x| x == *token as char)
    }
}

impl<T: PartialEq, const N: usize> FindToken<T> for [T; N] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

impl<'a, T: PartialEq, const N: usize> FindToken<T> for &'a [T; N] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

impl<'a, T: PartialEq> FindToken<T> for &'a [T] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

pub trait Alt<I, E> {
    type Output;
    
    fn choice(&mut self, input: I) -> ParseResult<Self::Output, I, E>;
}

pub trait Seq<I, E> {
    type Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E>;
}

macro_rules! tuple_parser_impl {
    ($a: ident, $($rest: ident),+) => {
        impl<Input, Error, $a, $($rest),+> Parser<Input, Error> for ($a, $($rest),+) 
        where
            $a: Parser<Input, Error>,
            $($rest: Parser<Input, Error>),+
        {
            type Output = ($a::Output, $($rest::Output),+);

            fn parse(&mut self, input: Input) -> Result<(Self::Output, Input), Error> {
                tuple_parser_inner!(o1, 0, self, input, (), $a, $($rest),+);
            }
        }
    };
}

macro_rules! tuple_parser_inner {
    ($o:ident, $field:tt, $self:expr, $input:expr, (), $a:ident, $($rest:ident),*) => {
        let ($o, i) = $self.$field.parse($input)?;
        succ_tuple_parer_inner!($field, $self, i, ($o), $($rest),*)
    };
    ($o:ident, $field:tt, $self:expr, $input:expr, ($($os:tt)*), $a:ident, $($rest:ident),*) => {
        let ($o, i) = $self.$field.parse($input)?;
        succ_tuple_parer_inner!($field, $self, i, ($($os)*, $o), $($rest),*)
    };
    ($o:ident, $field:tt, $self:expr, $input:expr, ($($os:tt)*), $a:ident) => {
        let ($o, i) = $self.$field.parse($input)?;
        return Ok((($($os)*, $o), i));
    };
}

macro_rules! succ_tuple_parer_inner {
    (0, $($p:tt),*) => (tuple_parser_inner!(o2, 1, $($p),*));
    (1, $($p:tt),*) => (tuple_parser_inner!(o3, 2, $($p),*));
    (2, $($p:tt),*) => (tuple_parser_inner!(o4, 3, $($p),*));
    (3, $($p:tt),*) => (tuple_parser_inner!(o5, 4, $($p),*));
    (4, $($p:tt),*) => (tuple_parser_inner!(o6, 5, $($p),*));
    (5, $($p:tt),*) => (tuple_parser_inner!(o7, 6, $($p),*));
    (6, $($p:tt),*) => (tuple_parser_inner!(o8, 7, $($p),*));
    (7, $($p:tt),*) => (tuple_parser_inner!(o9, 8, $($p),*));
    (8, $($p:tt),*) => (tuple_parser_inner!(o10, 9, $($p),*));
    (9, $($p:tt),*) => (tuple_parser_inner!(o11, 10, $($p),*));
    (10, $($p:tt),*) => (tuple_parser_inner!(o12, 11, $($p),*));
    (11, $($p:tt),*) => (tuple_parser_inner!(o13, 12, $($p),*));
    (12, $($p:tt),*) => (tuple_parser_inner!(o14, 13, $($p),*));
    (13, $($p:tt),*) => (tuple_parser_inner!(o15, 14, $($p),*));
    (14, $($p:tt),*) => (tuple_parser_inner!(o16, 15, $($p),*));
    (15, $($p:tt),*) => (tuple_parser_inner!(o17, 16, $($p),*));
    (16, $($p:tt),*) => (tuple_parser_inner!(o18, 17, $($p),*));
    (17, $($p:tt),*) => (tuple_parser_inner!(o19, 18, $($p),*));
    (18, $($p:tt),*) => (tuple_parser_inner!(o20, 19, $($p),*));
    (19, $($p:tt),*) => (tuple_parser_inner!(o21, 20, $($p),*));
    (20, $($p:tt),*) => (tuple_parser_inner!(o22, 21, $($p),*));
}

macro_rules! tuple_parser {
    ($a:ident, $b:ident, $($rest:ident),*) => {
        tuple_parser_impl!($a, $b, $($rest),*);
        tuple_parser!($b, $($rest),*);
    };
    ($a:ident, $b:ident) => {
        tuple_parser_impl!($a, $b);
    }
}

impl<I: Input, E> Parser<I, E> for () {
    type Output = ();

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        Ok(((), input))
    }
}

impl<P, I: Input, E> Parser<I, E> for (P, ) 
where 
    I: Input,
    P: Parser<I, E>
{
    type Output = P::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        self.0.parse(input)
    }
}

tuple_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);


macro_rules! seq_parser_impl {
    ($a: ident, $($rest: ident),+) => {
        impl<Input, Error, $a, $($rest),+> Seq<Input, Error> for ($a, $($rest),+) 
        where
            $a: Parser<Input, Error>,
            $($rest: Parser<Input, Error>),+
        {
            type Output = ($a::Output, $($rest::Output),+);

            fn parse(&mut self, input: Input) -> Result<(Self::Output, Input), Error> {
                tuple_parser_inner!(o1, 0, self, input, (), $a, $($rest),+);
            }
        }
    };
}

macro_rules! seq_parser {
    ($a:ident, $b:ident, $($rest:ident),*) => {
        seq_parser_impl!($a, $b, $($rest),*);
        seq_parser!($b, $($rest),*);
    };
    ($a:ident, $b:ident) => {
        seq_parser_impl!($a, $b);
    }
}

impl<I: Input, E> Seq<I, E> for () {
    type Output = ();

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        Ok(((), input))
    }
}

impl<P, I: Input, E> Seq<I, E> for (P, ) 
where 
    I: Input,
    P: Parser<I, E>
{
    type Output = P::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        self.0.parse(input)
    }
}

seq_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);

macro_rules! alt_parser_impl {
    ($a: ident, $($rest: ident),+) => {
        impl<Input, Error, $a, $($rest),+> Alt<Input, Error> for ($a, $($rest),+) 
        where
            Input: self::Input,
            $a: Parser<Input, Error>,
            Error: ParseError<Input>,
            $($rest: Parser<Input, Error, Output = $a::Output>),+
        {   
            type Output = $a::Output;

            fn choice(&mut self, input: Input) -> Result<(Self::Output, Input), Error> {
                alt_parser_inner!(0, (), self, input, $a, $($rest),+)
            }
        }
    };
}

macro_rules! alt_parser_inner {
    ($field:tt, (), $self:expr, $input:expr, $a:ident, $($rest:ident),*) => {
        match $self.$field.parse($input.clone()) {
            Ok(t) => Ok(t),
            Err(e1) => succ_alt_parser_inner!($field, (e1), $self, $input, $($rest),*)
        }
    };
    ($field:tt, ($err:expr), $self:expr, $input:expr, $a:ident, $($rest:ident),*) => {
        match $self.$field.parse($input.clone()) {
            Ok(t) => Ok(t),
            Err(e2) => {
                let e1 = $err.merge(e2);
                succ_alt_parser_inner!($field, (e1), $self, $input, $($rest),*)
            }
        }
    };
    ($field:tt, ($err:expr), $self:expr, $input:expr, $a:ident) => { 
        match $self.$field.parse($input.clone()) {
            Ok(t) => Ok(t),
            Err(e2) => Err($err.merge(e2))
        }
    }
}

macro_rules! succ_alt_parser_inner {
    (0, $($p:tt),*) => (alt_parser_inner!(1, $($p),*));
    (1, $($p:tt),*) => (alt_parser_inner!(2, $($p),*));
    (2, $($p:tt),*) => (alt_parser_inner!(3, $($p),*));
    (3, $($p:tt),*) => (alt_parser_inner!(4, $($p),*));
    (4, $($p:tt),*) => (alt_parser_inner!(5, $($p),*));
    (5, $($p:tt),*) => (alt_parser_inner!(6, $($p),*));
    (6, $($p:tt),*) => (alt_parser_inner!(7, $($p),*));
    (7, $($p:tt),*) => (alt_parser_inner!(8, $($p),*));
    (8, $($p:tt),*) => (alt_parser_inner!(9, $($p),*));
    (9, $($p:tt),*) => (alt_parser_inner!(10, $($p),*));
    (10, $($p:tt),*) => (alt_parser_inner!(11, $($p),*));
    (11, $($p:tt),*) => (alt_parser_inner!(12, $($p),*));
    (12, $($p:tt),*) => (alt_parser_inner!(13, $($p),*));
    (13, $($p:tt),*) => (alt_parser_inner!(14, $($p),*));
    (14, $($p:tt),*) => (alt_parser_inner!(15, $($p),*));
    (15, $($p:tt),*) => (alt_parser_inner!(16, $($p),*));
    (16, $($p:tt),*) => (alt_parser_inner!(17, $($p),*));
    (17, $($p:tt),*) => (alt_parser_inner!(18, $($p),*));
    (18, $($p:tt),*) => (alt_parser_inner!(19, $($p),*));
    (19, $($p:tt),*) => (alt_parser_inner!(20, $($p),*));
    (20, $($p:tt),*) => (alt_parser_inner!(21, $($p),*));
}

macro_rules! alt_parser {
    ($a:ident, $b:ident, $($rest:ident),*) => {
        alt_parser_impl!($a, $b, $($rest),*);
        alt_parser!($b, $($rest),*);
    };
    ($a:ident, $b:ident) => {
        alt_parser_impl!($a, $b);
    }
}

alt_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);