use std::ops::{
    RangeBounds,
    Bound
};

use crate::{
    Input, 
    Parser, 
    Error, 
    AsChar, 
    primitive::ascii::space, 
    ParserIter, 
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

pub fn many_till<I, A, B>(mut parser: A, mut pred: B) -> impl Parser<I, Output = Vec<A::Output>, Error = A::Error> 
where
    I: Input,
    A: Parser<I>,
    A::Error: Error<I>,
    B: Parser<I, Error = A::Error>
{
    move |input: I| {
        let mut it = ParserIter::new(input, not(pred.ref_mut()).andr(parser.ref_mut()));
        let os = it.collect();
        Ok((os, it.get()))
    }
}

pub fn many_till1<I, A, B>(mut parser: A, mut pred: B) -> impl Parser<I, Output = Vec<A::Output>, Error = A::Error> 
where
    I: Input,
    A: Parser<I>,
    A::Error: Error<I>,
    B: Parser<I, Error = A::Error>
{
    move |input: I| {
        let (o, i) = not(pred.ref_mut()).andr(parser.ref_mut()).parse(input)?;
        let mut os = vec![o];
        let mut it = ParserIter::new(i, not(pred.ref_mut()).andr(parser.ref_mut()));
        it.for_each(|o| os.push(o));
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

pub fn skip_till<I, A, B>(mut parser: A, mut pred: B) -> impl Parser<I, Output = (), Error = A::Error> 
where
    I: Input,
    A: Parser<I>,
    A::Error: Error<I>,
    B: Parser<I, Error = A::Error>
{
    move |input: I| {
        let mut it = ParserIter::new(input, not(pred.ref_mut()).andr(parser.ref_mut()));
        it.for_each(|_| ());
        Ok(((), it.get()))
    }
}

pub fn skip_till1<I, A, B>(mut parser: A, mut pred: B) -> impl Parser<I, Output = (), Error = A::Error> 
where
    I: Input,
    A: Parser<I>,
    A::Error: Error<I>,
    B: Parser<I, Error = A::Error>
{
    move |input: I| {
        let (_, i) = not(pred.ref_mut()).andr(parser.ref_mut()).parse(input)?;
        let mut it = ParserIter::new(i, not(pred.ref_mut()).andr(parser.ref_mut()));
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

pub fn sepby<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
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

pub fn sepby1<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
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

pub fn endby<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
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

pub fn endby1<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
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

pub fn not<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error>
where
    I: Input,
    P: Parser<I>,
    P::Error: Error<I>
{
    move |mut input: I| {
        match parser.parse(input.clone()) {
            Err(_) => Ok(((), input)),
            Ok(_) => Err(Error::unexpect(input.next()))
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

pub fn token<I, P, E>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    move |input: I| {
        let (_, i) = skip_many(space).parse(input)?;
        parser.parse(i)
    }
}


pub fn many_range<I, P, R>(mut parser: P, range: R) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>,
    R: RangeBounds<usize>
{
    move |input: I| {

        let mut it = ParserIter::new(input, parser.ref_mut());
        let os = match range.start_bound() {
            Bound::Excluded(_) => unreachable!(),
            Bound::Included(&s) => match range.end_bound() {
                Bound::Excluded(&e) => {
                    it.take(s)
                    .scan((s, e), |state, v| {

                        
                        Some(v)
                    })
                    .collect()
                },
                Bound::Included(&e) => {
                    // it.take(s)
                    // .scan(e + 1, |e, v| {
                    //     if *e == 0 {return None}
                    //     *e = *e - 1;
                    //     Some(v)
                    // })
                    // .collect()
                    todo!()
                }
                Bound::Unbounded => todo!(),
            }
            Bound::Unbounded => match range.end_bound() {
                Bound::Excluded(e) => todo!(),
                Bound::Included(e) => todo!(),
                Bound::Unbounded => todo!(),
            }
        };
        
        // let os = it.collect();
        Ok((os, it.try_get()?))
    }
}



use super::primitive::{char, anychar};
use super::error::ParseError;

#[test]
fn test() {
    // let r: Result<(Vec<char>, &str), ParseError<&str>> = many_range(char('1'), 2..3).parse("11");
    let r: Result<(Vec<char>, &str), ParseError<&str>> = many_range(char('1'), 1..3).parse("11112");

    println!("{:?}", r);

    let a = ..2;
}