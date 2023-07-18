use crate::{
    Parser,
    Input, 
    ParseResult,
    ParseError, 
    Alt, 
    Seq, 
};

/// Try to apply `parser`, if fails, returns [`None`] without cosuming input, otherwise 
/// return [`Some`] the value returned by `parser`.
/// # Example
/// ```
/// use rtor::{Parser, ParseResult};
/// use rtor::char::char;
/// use rtor::combinator::opt;
/// 
/// fn parser(i: &str) -> ParseResult<Option<char>, &str> {
///     opt(char('a'))(i)
/// }
/// 
/// assert_eq!(parser("abc"), Ok((Some('a'), "bc")));
/// assert_eq!(parser("bbc"), Ok((None, "bbc")));
/// ```
pub fn opt<P, I, E>(mut parser: P) -> impl FnMut(I) -> ParseResult<Option<P::Output>, I, E>
where
    I: Input,
    E: ParseError<I>,
    P: Parser<I, E>,
{
    move |input: I| {
        match parser.parse(input.clone()) {
            Ok((o, i)) => Ok((Some(o), i)),
            Err(_) => Ok((None, input))
        }
    }
}

/// Apply `parser` between parser `left` and parser `right`, the value returned by `parser`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::between;
/// 
/// fn parser(i: &str) -> ParseResult<char, &str> {
///     between(char('('), char('a'), char(')'))(i)
/// }
/// 
/// assert_eq!(parser("(a)"), Ok(('a', "")));
/// ```
pub fn between<I, E, L, P, R>(mut open: L, mut parser: P, mut close: R) -> impl FnMut(I) -> ParseResult<P::Output, I, E>
where
    I: Input,
    E: ParseError<I>,
    L: Parser<I, E>,
    P: Parser<I, E>,
    R: Parser<I, E>
{
    move |input: I| {
        let (_, i) = open.parse(input)?;
        let (o, i)= parser.parse(i)?;
        let (_, i) = close.parse(i)?;
        Ok((o, i))
    }
}

/// Returns value by parser `left` and parser `right` in tuple.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::pair;
/// 
/// fn parser(i: &str) -> ParseResult<(char, char), &str> {
///     pair(char('a'), char(':'), char('b'))(i)
/// }
/// 
/// assert_eq!(parser("a:b"), Ok((('a', 'b'), "")));
/// ```
pub fn pair<I, E, L, S, R>(mut left: L, mut sep: S, mut right: R) ->  impl FnMut(I) -> ParseResult<(L::Output, R::Output), I, E>
where 
    I: Input,
    L: Parser<I, E>,
    S: Parser<I, E>,
    R: Parser<I, E>
{
    move |input: I| {
        let (o1, i) = left.parse(input)?;
        let (_, i) = sep.parse(i)?;
        let (o2, i) = right.parse(i)?;
        Ok(((o1, o2), i))
    }
}


pub fn take<F, I, E>(mut f: F, n: usize) -> impl FnMut(I) -> ParseResult<I, I, E> 
where
    I: Input,
    E: ParseError<I>,
    F: FnMut(&I::Token) -> bool
{
    move |mut input: I| {
        let src = input.clone();
        for _ in 0..n {
            match input.peek() {
                Some(t) if f(&t) => { input.next(); }
                _ => return Err(E::unexpect(input))
            }
        }
        Ok((src.diff(&input), input))
    }
}

pub fn take_while<F, I, E>(mut f: F) -> impl FnMut(I) -> ParseResult<I, I, E> 
where
    I: Input,
    E: ParseError<I>,
    F: FnMut(&I::Token) -> bool
{
    move |mut input: I| {
        let src = input.clone();
        while let Some(t) = input.peek() {
            if f(&t) {
                input.next();
                continue;
            }
            break;
        }
        Ok((src.diff(&input), input))
    }
}

pub fn take_while1<F, I, E>(mut f: F) -> impl FnMut(I) -> ParseResult<I, I, E> 
where
    I: Input,
    E: ParseError<I>,
    F: FnMut(&I::Token) -> bool
{
    move |mut input: I| {
        let src = input.clone();
        match input.peek() {
            Some(t) if f(&t) => { input.next(); }
            _ => return Err(E::unexpect(input))
        }
        while let Some(t) = input.peek() {
            if f(&t) {
                input.next();
                continue;
            }
            break;
        }
        Ok((src.diff(&input), input))
    }
}

pub fn take_till<F, I, E>(mut f: F) -> impl FnMut(I) -> ParseResult<I, I, E> 
where
    I: Input,
    E: ParseError<I>,
    F: FnMut(&I::Token) -> bool
{
    move |mut input: I| {
        let src = input.clone();
        loop {
            match input.peek() {
                Some(t) if f(&t) => break,
                Some(_) => { input.next(); }
                None => return Err(E::unexpect(input))
            }
        }
        Ok((src.diff(&input), input))
    }
}

pub fn take_till1<F, I, E>(mut f: F) -> impl FnMut(I) -> ParseResult<I, I, E> 
where
    I: Input,
    E: ParseError<I>,
    F: FnMut(&I::Token) -> bool
{
    move |mut input: I| {
        let src = input.clone();
        match input.peek() {
            Some(t) if f(&t) => return Err(E::unexpect(input)),
            _ => { input.next(); }
        }
        loop {
            match input.peek() {
                Some(t) if f(&t) => break,
                Some(_) => { input.next(); }
                None => return Err(E::unexpect(input))
            }
        }
        Ok((src.diff(&input), input))
    }
}

/// Apply `parser` zero or more times, the results in a [`Vec`].
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::many;
/// 
/// fn parser(i: &str) -> ParseResult<Vec<char>, &str> {
///     many(char('a'))(i)
/// }
/// 
/// assert_eq!(parser("aaab"), Ok((vec!['a', 'a', 'a'], "b")));
/// assert_eq!(parser("b"), Ok((vec![], "b")));
/// ```
pub fn many<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    E: ParseError<I>,
    P: Parser<I, E>,
{
    move |mut input: I| {
        let mut result = vec![];
        while let Ok((o, i)) = parser.parse(input.clone()) {
            result.push(o);
            input = i;
        }
        Ok((result, input))
    }
}

/// Apply `parser` one or more times, the results in a [`Vec`].
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::many1;
/// 
/// fn parser(i: &str) -> ParseResult<Vec<char>, &str> {
///     many1(char('a'))(i)
/// }
/// 
/// assert_eq!(parser("aaab"), Ok((vec!['a', 'a', 'a'], "b")));
/// assert_eq!(parser("b"), Err(SimpleError::Unexpected(Some('b'))));
/// ```
pub fn many1<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        let (o, mut input) = parser.parse(input)?;
        let mut result = vec![o];
        while let Ok((o, i)) = parser.parse(input.clone()) {
            result.push(o);
            input = i;
        }
        Ok((result, input))
    }
}


/// Apply `parser` zero or more times until parser `pred` succeed, the results in a [`Vec`].
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::many_till;
/// 
/// fn parser(i: &str) -> ParseResult<Vec<char>, &str> {
///     many_till(char('a'), char('b'))(i)
/// }
/// 
/// assert_eq!(parser("aaab"), Ok((vec!['a', 'a', 'a'], "b")));
/// assert_eq!(parser("b"), Ok((vec![], "b")));
/// assert_eq!(parser("acb"), Err(SimpleError::Unexpected(Some('c'))));
/// assert_eq!(parser("aa"), Err(SimpleError::Unexpected(None)));
/// ```
pub fn many_till<P, F, I, E>(mut parser: P, mut f: F) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E>  
where
    I: Input,
    E: ParseError<I>,
    P: Parser<I, E>,
    F: Parser<I, E>,
{
    move |mut input: I| {
        let mut result = vec![];
        while let Err(_) = f.parse(input.clone()) {
            let (o, i) = parser.parse(input)?;
            result.push(o);
            input = i;
        }
        Ok((result, input))
    }
}

/// Apply `parser` specify `n` times, the results in a [`Vec`].
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::count;
/// 
/// fn parser(i: &str) -> ParseResult<Vec<char>, &str> {
///     count(char('a'), 3)(i)
/// }
/// 
/// assert_eq!(parser("aaa"), Ok((vec!['a', 'a', 'a'], "")));
/// assert_eq!(parser("aaaa"), Ok((vec!['a', 'a', 'a'], "a")));
/// assert_eq!(parser("aa"), Err(SimpleError::Unexpected(None)));
/// ```
pub fn count<I, E, P>(mut parser: P, n: usize) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E>  
where 
    I: Input,
    P: Parser<I, E>
{
    move |mut input: I| {
        let mut result = vec![];
        for _  in 0..n {
            let (o, i) = parser.parse(input)?;
            result.push(o);
            input = i;
        }
        Ok((result, input))
    }
}

/// Apply `parser` zero or more times, discard results.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::skip_many;
/// 
/// fn parser(i: &str) -> ParseResult<(), &str> {
///     skip_many(char('a'))(i)
/// }
/// 
/// assert_eq!(parser("aaab"), Ok(((), "b")));
/// assert_eq!(parser("b"), Ok(((), "b")));
/// ```
pub fn skip_many<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<(), I, E>  
where 
    I: Input,
    P: Parser<I, E>
{
    move |mut input: I| {
        while let Ok((_, i)) = parser.parse(input.clone()) {
            input = i;
        }
        Ok(((), input))
    }
}

/// Apply `parser` one or more times, discard results.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::skip_many1;
/// 
/// fn parser(i: &str) -> ParseResult<(), &str> {
///     skip_many1(char('a'))(i)
/// }
/// 
/// assert_eq!(parser("aaab"), Ok(((), "b")));
/// assert_eq!(parser("b"), Err(SimpleError::Unexpected(Some('b'))));
/// ```
pub fn skip_many1<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<(), I, E> 
where 
    I: Input,
    P: Parser<I, E>
{
    move |input: I| {
        let (_, mut input) = parser.parse(input)?;
        while let Ok((_, i)) = parser.parse(input.clone()) {
            input = i;
        }
        Ok(((), input))
    }
}

/// Apply `parser` zero or more times until parser `pred` succeed, discard results.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::skip_till;
/// 
/// fn parser(i: &str) -> ParseResult<(), &str> {
///     skip_till(char('a'), char('b'))(i)
/// }
/// 
/// assert_eq!(parser("aaab"), Ok(((), "b")));
/// assert_eq!(parser("b"), Ok(((), "b")));
/// assert_eq!(parser("acb"), Err(SimpleError::Unexpected(Some('c'))));
/// assert_eq!(parser("aa"), Err(SimpleError::Unexpected(None)));
/// ```
pub fn skip_till<P, F, I, E>(mut parser: P, mut f: F) -> impl FnMut(I) -> ParseResult<(), I, E> 
where
    I: Input,
    E: ParseError<I>,
    P: Parser<I, E>,
    F: Parser<I, E>
{
    move |mut input: I| {
        while let Err(_) = f.parse(input.clone()) {
            let (_, i) = parser.parse(input)?;
            input = i;
        }
        Ok(((), input))
    }
}

/// Apply `parser` specify `n` times, discard results.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::skip;
/// 
/// fn parser(i: &str) -> ParseResult<(), &str> {
///     skip(char('a'), 3)(i)
/// }
/// 
/// assert_eq!(parser("aaa"), Ok(((), "")));
/// assert_eq!(parser("aaaa"), Ok(((), "a")));
/// assert_eq!(parser("aa"), Err(SimpleError::Unexpected(None)));
/// ```
pub fn skip<I, E, P>(mut parser: P, n: usize) -> impl FnMut(I) -> ParseResult<(), I, E> 
where 
    I: Input,
    P: Parser<I, E>
{
    move |mut input: I| {
        for _  in 0..n {
            let (_, i) = parser.parse(input)?;
            input = i;
        }
        Ok(((), input))
    }
}

/// Apply `parser` zero or more times, separated by parser `sep`, the results in a [`Vec`].
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::sep_by;
/// 
/// fn parser(i: &str) -> ParseResult<Vec<char>, &str> {
///     sep_by(char('a'), char(','))(i)
/// }
/// 
/// assert_eq!(parser("a,a,a"), Ok((vec!['a', 'a', 'a'], "")));
/// assert_eq!(parser("a"), Ok((vec!['a'], "")));
/// assert_eq!(parser("b"), Ok((vec![], "b")));
/// ```
pub fn sep_by<I, E, P, S>(mut parser: P, mut sep: S) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>, 
    S: Parser<I, E>
{
    move |input: I| {
        let (mut result, mut input) = match parser.parse(input.clone()) {
            Ok((o, i)) => (vec![o], i),
            Err(_) => return Ok((vec![], input))
        };
        while let Ok((_, i)) = sep.parse(input.clone()) {
            let (o, i) = parser.parse(i)?;
            result.push(o);
            input = i;
        }
        Ok((result, input))
    }
}

/// Apply `parser` one or more times, separated by parser `sep`, the results in a [`Vec`].
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::sep_by1;
/// 
/// fn parser(i: &str) -> ParseResult<Vec<char>, &str> {
///     sep_by1(char('a'), char(','))(i)
/// }
/// 
/// assert_eq!(parser("a,a,a"), Ok((vec!['a', 'a', 'a'], "")));
/// assert_eq!(parser("a"), Ok((vec!['a'], "")));
/// assert_eq!(parser("b"), Err(SimpleError::Unexpected(Some('b'))));
/// ```
pub fn sep_by1<I, E, P, S>(mut parser: P, mut sep: S) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>, 
    S: Parser<I, E>
{
    move |input: I| {
        let (o, mut input) = parser.parse(input)?;
        let mut result = vec![o];
        while let Ok((_, i)) = sep.parse(input.clone()) {
            let (o, i) = parser.parse(i)?;
            result.push(o);
            input = i;
        }
        Ok((result, input))
    }
}

/// Apply `parser` zero or more times, separated end by parser `sep`, the results in a [`Vec`].
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::end_by;
/// 
/// fn parser(i: &str) -> ParseResult<Vec<char>, &str> {
///     end_by(char('a'), char(';'))(i)
/// }
/// 
/// assert_eq!(parser("a;a;a;"), Ok((vec!['a', 'a', 'a'], "")));
/// assert_eq!(parser("a;"), Ok((vec!['a'], "")));
/// assert_eq!(parser("b"), Ok((vec![], "b")));
/// ```
pub fn end_by<I, E, P, S>(mut parser: P, mut sep: S) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E>
where
    I: Input,
    P: Parser<I, E>, 
    S: Parser<I, E>
{
    move |input: I| { 
        let mut result = vec![];
        let (o, i) = match parser.parse(input.clone()) {
            Ok((o, i)) => (o, i),
            Err(_) => return Ok((vec![], input))
        };
        let (_, mut input) = sep.parse(i)?;
        result.push(o);
        while let Ok((o, i)) = parser.parse(input.clone()) {
            let (_, i) = sep.parse(i)?;
            result.push(o);
            input = i;            
        }
        Ok((result, input))
    }
}

/// Apply `parser` one or more times, separated end by parser `sep`, the results in a [`Vec`].
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::end_by1;
/// 
/// fn parser(i: &str) -> ParseResult<Vec<char>, &str> {
///     end_by1(char('a'), char(';'))(i)
/// }
/// 
/// assert_eq!(parser("a;a;a;"), Ok((vec!['a', 'a', 'a'], "")));
/// assert_eq!(parser("a;"), Ok((vec!['a'], "")));
/// assert_eq!(parser("a"), Err(SimpleError::Unexpected(None)));
/// assert_eq!(parser("b"), Err(SimpleError::Unexpected(Some('b'))));
/// ```
pub fn end_by1<I, E, P, S>(mut parser: P, mut sep: S) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    P: Parser<I, E>, 
    S: Parser<I, E>
{
    move |input: I| { 
        let mut result = vec![];
        let (o, i) = parser.parse(input)?;
        let (_, mut input) = sep.parse(i)?;
        result.push(o);
        while let Ok((o, i)) = parser.parse(input.clone()) {
            let (_, i) = sep.parse(i)?;
            result.push(o);
            input = i;            
        }
        Ok((result, input))
    }
}


/// Apply `parser` without cosuming input, the value returned by `parser`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::peek;
/// 
/// fn parser(i: &str) -> ParseResult<char, &str> {
///     peek(char('a'))(i)
/// }
/// 
/// assert_eq!(parser("abc"), Ok(('a', "abc")));
/// assert_eq!(parser("cbc"), Err(SimpleError::Unexpected(Some('c'))));
/// ```
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

/// Apply `parser`, if succeed, return consumed input.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::{recognize, many};
/// 
/// fn parser(i: &str) -> ParseResult<&str, &str> {
///     recognize(many(char('a')))(i)
/// }
/// 
/// assert_eq!(parser("aaab"), Ok(("aaa", "b")));
/// ```
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

/// Succeeds if `parser` failed.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::not;
/// 
/// fn parser(i: &str) -> ParseResult<(), &str> {
///     not(char('a'))(i)
/// }
/// 
/// assert_eq!(parser("ba"), Ok(((), "ba")));
/// assert_eq!(parser("ab"), Err(SimpleError::Unexpected(Some('a'))));
/// ```
pub fn not<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<(), I, E>
where
    I: Input,
    E: ParseError<I>,
    P: Parser<I, E>,
{
    move |input: I| {
        match parser.parse(input.clone()) {
            Err(_) => Ok(((), input)),
            Ok(_) => Err(ParseError::unexpect(input))
        }
    }
}

/// Apply parser `cond`, if fails, returns [`None`] without consuming `input`, otherwise apply `parser`, the value returned by `parser`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::cond;
/// 
/// fn parser(i: &str) -> ParseResult<Option<char>, &str> {
///     cond(char('a'), char('b'))(i)
/// }
/// 
/// assert_eq!(parser("abc"), Ok((Some('b'), "c")));
/// assert_eq!(parser("cbc"), Ok((None, "cbc")));
/// assert_eq!(parser("acb"), Err(SimpleError::Unexpected(Some('c'))));
/// ```
pub fn cond<F, P, I, E>(mut f: F, mut parser: P) -> impl FnMut(I) -> ParseResult<Option<P::Output>, I, E> 
where
    I: Input,
    F: Parser<I, E>,
    P: Parser<I, E>
{
    move |input: I| {
        match f.parse(input.clone()) {
            Ok((_, i)) => match parser.parse(i) {
                Ok((o, i)) => Ok((Some(o), i)),
                Err(e) => Err(e)
            }
            Err(_) => Ok((None, input))
        }
    }
}

/// Apply parser `first` then apply parser `second`, the value returned by parser `second`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::preceded;
/// 
/// fn parser(i: &str) -> ParseResult<char, &str> {
///     preceded(char('a'), char('b'))(i)
/// }
/// 
/// assert_eq!(parser("abc"), Ok(('b', "c")));
/// ```
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

/// Apply parser `first` then apply parser `second`, the value returned by parser `first`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser};
/// use rtor::char::char;
/// use rtor::combinator::terminated;
/// 
/// fn parser(i: &str) -> ParseResult<char, &str> {
///     terminated(char('a'), char('b'))(i)
/// }
/// 
/// assert_eq!(parser("abc"), Ok(('a', "c")));
/// ```
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

/// Succeed if at the end of `input`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::eof;
/// 
/// fn parser(i: &str) -> ParseResult<(), &str> {
///     eof(i)
/// }
/// 
/// assert_eq!(parser(""), Ok(((), "")));
/// assert_eq!(parser("abc"), Err(SimpleError::Unexpected(Some('a'))));
/// ```
pub fn eof<I, E>(mut input: I) ->  ParseResult<(), I, E>
where
    I: Input,
    E: ParseError<I>
{
    match input.peek() {
        None => Ok(((), input)),
        Some(_) => Err(ParseError::unexpect(input))
    }
}

pub fn empty<I, E>(input: I) ->  ParseResult<(), I, E>
where
    I: Input,
    E: ParseError<I>
{
    Ok(((), input))
}

/// Always fail.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::error;
/// 
/// fn parser(i: &str) -> ParseResult<(), &str> {
///     error(i)
/// }
/// 
/// assert_eq!(parser("abc"), Err(SimpleError::Unexpected(Some('a'))));
/// ```
pub fn error<I, E>(input: I) -> ParseResult<(), I, E> 
where
    I: Input,
    E: ParseError<I>
{
    Err(ParseError::unexpect(input))
}

/// Always succeed without consuming `input`.
/// # Example
/// ```
/// use rtor::{ParseResult, Parser, SimpleError};
/// use rtor::char::char;
/// use rtor::combinator::pure;
/// 
/// fn parser(i: &str) -> ParseResult<char, &str> {
///     pure('1')(i)
/// }
/// 
/// assert_eq!(parser("abc"), Ok(('1', "abc")));
/// assert_eq!(parser(""), Ok(('1', "")));
/// ```
pub fn pure<T, I, E>(t: T) -> impl FnMut(I) -> ParseResult<T, I, E> 
where
    I: Input,
    E: ParseError<I>,
    T: Clone,
{
    move|input: I| Ok((t.clone(), input))
}

pub fn value<T, P, I, E>(v: T, mut parser: P) -> impl FnMut(I) -> ParseResult<T, I, E> 
where
    I: Input,
    E: ParseError<I>,
    P: Parser<I, E>,
    T: Clone,
{
    move |input: I| {
        let (_, i) = parser.parse(input)?;
        Ok((v.clone(), i))
    }
}

pub fn verify<P, F, I, E>(mut parser: P, f: F) -> impl FnMut (I) -> ParseResult<P::Output, I, E> 
where
    I: Input,
    E: ParseError<I>,
    P: Parser<I, E>,
    F: Fn(&P::Output) -> bool
{
    move |input| {
        let src = input.clone();
        let (o, i) = parser.parse(input)?;
        if f(&o) {
            Ok((o, i))
        }else {
            Err(ParseError::unexpect(src))
        }
    }
}

pub fn alt<I, E, List>(mut list: List) -> impl FnMut(I) -> ParseResult<List::Output, I, E> 
where 
    I: Input,
    E: ParseError<I>,
    List: Alt<I, E>
{
    move |input: I| list.choice(input)
}

pub fn seq<I, E, List>(mut list: List) -> impl FnMut(I) -> ParseResult<List::Output, I, E> 
where 
    I: Input,
    E: ParseError<I>,
    List: Seq<I, E>
{
    move |input: I| list.parse(input)
}