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

pub fn opt<I, P>(mut parser: P) -> impl Parser<I, Output = Option<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        match parser.parse(input.clone()) {
            Ok((o, i)) => Ok((Some(o), i)),
            Err(_) => Ok((None, input))
        }
    }
}

pub fn between<I, L, M, R>(mut left: L, mut middle: M, mut right: R) -> impl Parser<I, Output = M::Output, Error = L::Error> 
where
    I: Input,
    L: Parser<I>,
    M: Parser<I, Error = L::Error>,
    R: Parser<I, Error = L::Error>
{
    move |input: I| {
        let (_, i) = left.parse(input)?;
        let (o, i)= middle.parse(i)?;
        let (_, i) = right.parse(i)?;
        Ok((o, i))
    }
}

pub fn pair<I, L, M, R>(mut left: L, mut middle: M, mut right: R) ->  impl Parser<I, Output = (L::Output, R::Output), Error = L::Error>
where 
    I: Input,
    L: Parser<I>,
    M: Parser<I, Error = L::Error>,
    R: Parser<I, Error = L::Error>
{
    move |input: I| {
        let (o1, i) = left.parse(input)?;
        let (_, i) = middle.parse(i)?;
        let (o2, i) = right.parse(i)?;
        Ok(((o1, o2), i))
    }
}

pub fn many<I, P>(mut parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>,
{
    move |input: I| {
        let mut it = ParserIter::new(input, parser.ref_mut());
        let os = it.collect();
        Ok((os, it.get()))
    }
}

pub fn many1<I, P>(mut parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        let (o, i) = parser.parse(input)?;
        let mut os = vec![o];
        let mut it = ParserIter::new(i, parser.ref_mut());
        it.for_each(|o| os.push(o));
        Ok((os, it.get()))
    }
}

pub fn many_till<I, S, A, B>(mut parser: A, mut pred: B) -> impl Parser<I, Output = Vec<A::Output>, Error = A::Error> 
where
    I: Input,
    A: Parser<I>,
    A::Error: ParseError<I, S>,
    B: Parser<I, Error = A::Error>
{
    move |input: I| {
        let mut it = ParserIter::new(input, not(pred.ref_mut()).andr(parser.ref_mut()));
        let os = it.collect();
        Ok((os, it.get()))
    }
}

pub fn count<I, P>(mut parser: P, n: usize) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        let mut it = ParserIter::new(input, parser.ref_mut());
        let os = it.take(n).collect();
        Ok((os, it.try_get()?))
    }
}

pub fn skip_many<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        let mut it = ParserIter::new(input, parser.ref_mut());
        it.for_each(|_| ());
        Ok(((), it.get()))
    }
}

pub fn skip_many1<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        let (_, i) = parser.parse(input)?;
        let mut it = ParserIter::new(i, parser.ref_mut());
        it.for_each(|_| ());
        Ok(((), it.get()))
    }
}

pub fn skip_till<I, S, A, B>(mut parser: A, mut pred: B) -> impl Parser<I, Output = (), Error = A::Error> 
where
    I: Input,
    A: Parser<I>,
    A::Error: ParseError<I, S>,
    B: Parser<I, Error = A::Error>
{
    move |input: I| {
        let mut it = ParserIter::new(input, not(pred.ref_mut()).andr(parser.ref_mut()));
        it.for_each(|_| ());
        Ok(((), it.get()))
    }
}

pub fn skip<I, P>(mut parser: P, n: usize) -> impl Parser<I, Output = (), Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        let mut it = ParserIter::new(input, parser.ref_mut());
        it.take(n).for_each(|_| ());
        Ok(((), it.try_get()?))
    }
}

pub fn sep_by<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
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

pub fn sep_by1<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |input: I| {
        let (o, i) = parser.parse(input)?;
        let mut os = vec![o];
        let mut it = ParserIter::new(i, sep.ref_mut().andr(parser.ref_mut()));
        it.for_each(|o| os.push(o));
        Ok((os, it.get()))
    }
}

pub fn end_by<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
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

pub fn end_by1<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |input: I| { 
        let (o, i) = parser.ref_mut().andl(sep.ref_mut()).parse(input.clone())?;
        let mut os = vec![o];
        let mut it = ParserIter::new(i, parser.ref_mut().andl(sep.ref_mut()));
        it.for_each(|o| os.push(o));
        Ok((os, it.get()))
    }
}

pub fn peek<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error> 
where
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        match parser.parse(input.clone()) {
            Ok((o, _)) => Ok((o, input)),
            Err(e) => Err(e)
        }
    }
}

pub fn recognize<I, P>(mut parser: P) -> impl Parser<I, Output = I, Error = P::Error> 
where
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        let src = input.clone();
        let (_, i) = parser.parse(input)?;
        Ok((src.diff(&i), i))
    }
}

pub fn not<I, S, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error>
where
    I: Input,
    P: Parser<I>,
    P::Error: ParseError<I, S>
{
    move |mut input: I| {
        match parser.parse(input.clone()) {
            Err(_) => Ok(((), input)),
            Ok(_) => Err(ParseError::unexpect(input.peek(), input))
        }
    }
}

pub fn cond<I, C, P>(mut condition: C, mut parser: P) -> impl Parser<I, Output = Option<P::Output>, Error = P::Error> 
where
    I: Input,
    C: Parser<I>,
    P: Parser<I, Error = C::Error>
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

pub fn eof<I, S, E>(mut input: I) ->  ParseResult<(), I, E>
where
    I: Input,
    E: ParseError<I, S>
{
    match input.peek() {
        None => Ok(((), input)),
        Some(t) => Err(ParseError::unexpect(Some(t), input))
    }
}

pub fn error<I, S, E>(mut input: I) -> ParseResult<(), I, E> 
where
    I: Input,
    E: ParseError<I, S>
{
    Err(ParseError::unexpect(input.peek(), input))
}

pub fn pure<I, S, E, T>(t: T) -> impl Parser<I, Output = T, Error = E> 
where
    I: Input,
    T: Clone,
    E: ParseError<I, S>
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

pub fn manyr<I, P, R>(mut parser: P, range: R) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>,
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

pub fn skipr<I, P, R>(mut parser: P, range: R) -> impl Parser<I, Output = (), Error = P::Error> 
where
    I: Input,
    P: Parser<I>,
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

pub fn alt<I: Input, S, A: Alt<I, S>>(mut list: A) -> impl Parser<I, Output = A::Output, Error = A::Error> {
    move |input: I| list.choice(input)
}

macro_rules! tuple_parser_impl {
    ($a: ident, $($rest: ident),+) => {
        impl<Input, $a, $($rest),+> Parser<Input> for ($a, $($rest),+) 
        where
            $a: Parser<Input>,
            $($rest: Parser<Input, Error = $a::Error>),+
        {
            type Output = ($a::Output, $($rest::Output),+);
            type Error = $a::Error;

            fn parse(&mut self, input: Input) -> Result<(Self::Output, Input), Self::Error> {
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
        impl<Input, Msg, $a, $($rest),+> Alt<Input, Msg> for ($a, $($rest),+) 
        where
            Input: super::Input,
            $a: Parser<Input>,
            $a::Error: ParseError<Input, Msg>,
            $($rest: Parser<Input, Error = $a::Error, Output = $a::Output>),+
        {   
            type Output = $a::Output;
            type Error = $a::Error;

            fn choice(&mut self, input: Input) -> Result<(Self::Output, Input), Self::Error> {
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