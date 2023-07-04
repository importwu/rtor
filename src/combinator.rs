use std::ops::{
    RangeBounds,
    Bound
};

use crate::{
    Input, 
    Parser, 
    ParseError, 
    ParseResult,
    ParserIter, 
    Alt
};



// #[test]
// fn test() {
//     let mut parser = opt(super::char::char('a'));
//     let result: ParseResult<Option<char>, &str> = parser.parse("abc");
//     let a =  Ok::<i32, i32>(2);
// }


/// Apply `parser`, if fails, returns [`None`] without cosuming input, otherwise 
/// returns [`Some`] the value returned by `parser`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::opt;
/// 
/// fn parser(i: &str) -> ParseResult<Option<char>, &str> {
///     opt(char('a'))(i)
/// }
/// 
/// assert_eq!(parser("abc"), Ok((Some('a'), "bc")));
/// assert_eq!(parser("bbc"), Ok((None, "bbc")))
/// ```
pub fn opt<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<Option<P::Output>, I, E>
where
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        match parser.parse(input.clone()) {
            Ok((o, i)) => Ok((Some(o), i)),
            Err(_) => Ok((None, input))
        }
    }
}

/// Apply `parser` between `left` parser and `right` parser, the value returned by `parser`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::between;
/// 
/// fn parser(i: &str) -> ParseResult<char, &str> {
///     between(char('a'), char('b'), char('c'))(i)
/// }
/// 
/// assert_eq!(parser("abc"), Ok(('b', "")))
/// ```
pub fn between<I, E, L, P, R>(mut left: L, mut parser: P, mut right: R) -> impl FnMut(I) -> ParseResult<P::Output, I, E>
where
    I: Input,
    L: Parser<I, E>,
    P: Parser<I, E>,
    R: Parser<I, E>
{
    move |input: I| {
        let (_, i) = left.parse(input)?;
        let (o, i)= parser.parse(i)?;
        let (_, i) = right.parse(i)?;
        Ok((o, i))
    }
}


pub fn pair<I, E, L, P, R>(mut left: L, mut parser: P, mut right: R) ->  impl FnMut(I) -> ParseResult<(L::Output, R::Output), I, E>
where 
    I: Input,
    L: Parser<I, E>,
    P: Parser<I, E>,
    R: Parser<I, E>
{
    move |input: I| {
        let (o1, i) = left.parse(input)?;
        let (_, i) = parser.parse(i)?;
        let (o2, i) = right.parse(i)?;
        Ok(((o1, o2), i))
    }
}

pub fn many<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>,
{
    move |input: I| {
        let mut it = ParserIter::new(input, parser.ref_mut());
        let os = it.collect();
        Ok((os, it.get()))
    }
}

pub fn many1<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        let (o, i) = parser.parse(input)?;
        let mut os = vec![o];
        let mut it = ParserIter::new(i, parser.ref_mut());
        it.for_each(|o| os.push(o));
        Ok((os, it.get()))
    }
}

pub fn many_till<I, E, S, A, B>(mut parser: A, mut pred: B) -> impl FnMut(I) -> ParseResult<Vec<A::Output>, I, E>  
where
    I: Input,
    E: ParseError<I, S>,
    A: Parser<I, E>,
    B: Parser<I, E>,
{
    move |input: I| {
        let mut it = ParserIter::new(input, not(pred.ref_mut()).andr(parser.ref_mut()));
        let os = it.collect();
        Ok((os, it.get()))
    }
}

pub fn count<I, E, P>(mut parser: P, n: usize) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E>  
where 
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        let mut it = ParserIter::new(input, parser.ref_mut());
        let os = it.take(n).collect();
        Ok((os, it.try_get()?))
    }
}

pub fn skip_many<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<(), I, E>  
where 
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        let mut it = ParserIter::new(input, parser.ref_mut());
        it.for_each(|_| ());
        Ok(((), it.get()))
    }
}

pub fn skip_many1<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<(), I, E> 
where 
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        let (_, i) = parser.parse(input)?;
        let mut it = ParserIter::new(i, parser.ref_mut());
        it.for_each(|_| ());
        Ok(((), it.get()))
    }
}

pub fn skip_till<I, E, S, A, B>(mut parser: A, mut pred: B) -> impl FnMut(I) -> ParseResult<(), I, E> 
where
    I: Input,
    E: ParseError<I, S>,
    A: Parser<I, E>,
    B: Parser<I, E>
{
    move |input: I| {
        let mut it = ParserIter::new(input, not(pred.ref_mut()).andr(parser.ref_mut()));
        it.for_each(|_| ());
        Ok(((), it.get()))
    }
}

pub fn skip<I, E, P>(mut parser: P, n: usize) -> impl FnMut(I) -> ParseResult<(), I, E> 
where 
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        let mut it = ParserIter::new(input, parser.ref_mut());
        it.take(n).for_each(|_| ());
        Ok(((), it.try_get()?))
    }
}

pub fn sep_by<I, E, P, S>(mut parser: P, mut sep: S) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>, 
    S: Parser<I, E>
{
    move |input: I| {
        let (mut os, i) = match parser.parse(input.clone()) {
            Ok((o, i)) => (vec![o], i),
            Err(_) => return Ok((vec![], input))
        };
        let mut it = ParserIter::new(i, sep.ref_mut().andr(parser.ref_mut()));
        it.for_each(|o| os.push(o));
        Ok((os, it.get()))
    }
}

pub fn sep_by1<I, E, P, S>(mut parser: P, mut sep: S) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>, 
    S: Parser<I, E>
{
    move |input: I| {
        let (o, i) = parser.parse(input)?;
        let mut os = vec![o];
        let mut it = ParserIter::new(i, sep.ref_mut().andr(parser.ref_mut()));
        it.for_each(|o| os.push(o));
        Ok((os, it.get()))
    }
}

pub fn end_by<I, E, P, S>(mut parser: P, mut sep: S) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E>
where
    I: Input,
    P: Parser<I, E>, 
    S: Parser<I, E>
{
    move |input: I| { 
        let (mut os, i) = match parser.ref_mut().andl(sep.ref_mut()).parse(input.clone()) {
            Ok((o, i)) => (vec![o], i),
            Err(_) => return Ok((vec![], input))
        };
        let mut it = ParserIter::new(i, parser.ref_mut().andl(sep.ref_mut()));
        it.for_each(|o| os.push(o));
        Ok((os, it.get()))
    }
}

pub fn end_by1<I, E, P, S>(mut parser: P, mut sep: S) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>, 
    S: Parser<I, E>
{
    move |input: I| { 
        let (o, i) = parser.ref_mut().andl(sep.ref_mut()).parse(input.clone())?;
        let mut os = vec![o];
        let mut it = ParserIter::new(i, parser.ref_mut().andl(sep.ref_mut()));
        it.for_each(|o| os.push(o));
        Ok((os, it.get()))
    }
}

pub fn peek<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<P::Output, I, E> 
where
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        match parser.parse(input.clone()) {
            Ok((o, _)) => Ok((o, input)),
            Err(e) => Err(e)
        }
    }
}

pub fn recognize<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<I, I, E>
where
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        let src = input.clone();
        let (_, i) = parser.parse(input)?;
        Ok((src.diff(&i), i))
    }
}

pub fn not<I, E, S, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<(), I, E>
where
    I: Input,
    E: ParseError<I, S>,
    P: Parser<I, E>,
{
    move |mut input: I| {
        match parser.parse(input.clone()) {
            Err(_) => Ok(((), input)),
            Ok(_) => Err(ParseError::unexpect(input.peek(), input))
        }
    }
}

pub fn cond<I, E, C, P>(mut condition: C, mut parser: P) -> impl FnMut(I) -> ParseResult<Option<P::Output>, I, E> 
where
    I: Input,
    C: Parser<I, E>,
    P: Parser<I, E>
{
    move |input: I| {
        match condition.parse(input.clone()) {
            Ok((_, i)) => match parser.parse(i) {
                Ok((o, i)) => Ok((Some(o), i)),
                Err(e) => Err(e)
            }
            Err(_) => Ok((None, input))
        }
    }
}

pub fn preceded<I, E, A, B>(mut first: A, mut second: B) -> impl FnMut(I) -> ParseResult<B::Output, I, E> 
where
    I: Input,
    A: Parser<I, E>,
    B: Parser<I, E>
{
    move |input: I| {
        let (_, i) = first.parse(input)?;
        second.parse(i)
    }
}

pub fn terminated<I, E, A, B>(mut first: A, mut second: B) -> impl FnMut(I) -> ParseResult<A::Output, I, E> 
where
    I: Input,
    A: Parser<I, E>,
    B: Parser<I, E>
{
    move |input: I| {
        let (o, i) = first.parse(input)?;
        let (_, i) = second.parse(i)?;
        Ok((o, i))
    }
}

pub fn eof<I, E, S>(mut input: I) ->  ParseResult<(), I, E>
where
    I: Input,
    E: ParseError<I, S>
{
    match input.peek() {
        None => Ok(((), input)),
        Some(t) => Err(ParseError::unexpect(Some(t), input))
    }
}

pub fn error<I, E, S>(mut input: I) -> ParseResult<(), I, E> 
where
    I: Input,
    E: ParseError<I, S>
{
    Err(ParseError::unexpect(input.peek(), input))
}

pub fn pure<I, E, S, T>(t: T) -> impl FnMut(I) -> ParseResult<T, I, E> 
where
    I: Input,
    E: ParseError<I, S>,
    T: Clone,
{
    move|input: I| Ok((t.clone(), input))
}

fn map_range<R: RangeBounds<usize>>(range: R) -> (Option<usize>, Option<usize>) {
    match range.start_bound() {
        Bound::Excluded(&s) => match range.end_bound() {
            Bound::Excluded(&e) => (Some(s.saturating_sub(1)), Some(e.saturating_sub(1))),
            Bound::Included(&e) => (Some(s), Some(e)),
            Bound::Unbounded => (Some(s), None),
        }
        Bound::Included(&s) => match range.end_bound() {
            Bound::Excluded(&e) => (Some(s), Some(e.saturating_sub(1))),
            Bound::Included(&e) => (Some(s), Some(e)),
            Bound::Unbounded => (Some(s), None),
        }
        Bound::Unbounded => match range.end_bound() {
            Bound::Excluded(&e) => (None, Some(e.saturating_sub(1))),
            Bound::Included(&e) => (None, Some(e)),
            Bound::Unbounded => (None, None),
        }
    }
}

pub fn manyr<I, E, P, R>(mut parser: P, range: R) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E>
where
    I: Input,
    P: Parser<I, E>,
    R: RangeBounds<usize>
{
    let (start, end) = map_range(range);
    move |mut input: I| {
        let mut os = vec![];
        match start {
            Some(s) => {
                for _ in 0..s {
                    let (o, i) = parser.parse(input.clone())?;
                    input = i;
                    os.push(o);
                }
                match end {
                    Some(e) => {
                        for _ in 0..(e.saturating_sub(s)) {
                            match parser.parse(input.clone()) {
                                Ok((o, i)) => {
                                    input = i;
                                    os.push(o);
                                }
                                Err(_) => break
                            }
                        }
                    }
                    None => {
                        while let Ok((o, i)) = parser.parse(input.clone()) {
                            input = i;
                            os.push(o);
                        }
                    }
                }
            }
            None => match end {
                Some(e) => {
                    for _ in 0..e {
                        match parser.parse(input.clone()) {
                            Ok((o, i)) => {
                                input = i;
                                os.push(o);
                            }
                            Err(_) => break
                        }
                    }
                }
                None => {
                    while let Ok((o, i)) = parser.parse(input.clone()) {
                        input = i;
                        os.push(o);
                    }
                }
            }
        }

        Ok((os, input))
    }
}

pub fn skipr<I, E, P, R>(mut parser: P, range: R) -> impl FnMut(I) -> ParseResult<(), I, E> 
where
    I: Input,
    P: Parser<I, E>,
    R: RangeBounds<usize>
{
    let (start, end) = map_range(range);
    move |mut input: I| {
        match start {
            Some(s) => {
                for _ in 0..s {
                    let (_, i) = parser.parse(input.clone())?;
                    input = i;
                }
                match end {
                    Some(e) => {
                        for _ in 0..(e - s) {
                            match parser.parse(input.clone()) {
                                Ok((_, i)) => {
                                    input = i;
                                }
                                Err(_) => break
                            }
                        }
                    }
                    None => {
                        while let Ok((_, i)) = parser.parse(input.clone()) {
                            input = i;
                        }
                    }
                }
            }
            None => match end {
                Some(e) => {
                    for _ in 0..e {
                        match parser.parse(input.clone()) {
                            Ok((_, i)) => {
                                input = i;
                            }
                            Err(_) => break
                        }
                    }
                }
                None => {
                    while let Ok((_, i)) = parser.parse(input.clone()) {
                        input = i;
                    }
                }
            }
        }

        Ok(((), input))
    }
}

pub fn alt<I, E, S, A>(mut list: A) -> impl FnMut(I) -> ParseResult<A::Output, I, E> 
where 
    I: Input,
    E: ParseError<I, S>,
    A: Alt<I, E, S>
{
    move |input: I| list.choice(input)
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

tuple_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);

macro_rules! alt_parser_impl {
    ($a: ident, $($rest: ident),+) => {
        impl<Input, Error, Msg, $a, $($rest),+> Alt<Input, Error, Msg> for ($a, $($rest),+) 
        where
            Input: super::Input,
            $a: Parser<Input, Error>,
            Error: ParseError<Input, Msg>,
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