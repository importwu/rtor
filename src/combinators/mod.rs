use std::ops::{RangeBounds, Bound};

use super::traits::{Parser, Input};


pub fn attempt<I, P>(mut parser: P)  -> impl Parser<I, Output = P::Output, Error = P::Error>
    where I: Input,
        P: Parser<I>
{
    move |input: &mut I| {
        let mut c = input.cursor();
        match parser.parse(input) {
            Ok(t) => {
                Ok(t)
            },
            Err(e) => {
                c.restore();
                Err(e)
            }
        }
    }
}


#[inline]
pub fn refer<'a, I, P>(parser: &'a mut P) -> impl Parser<I, Output = P::Output, Error = P::Error> + 'a
    where P: Parser<I>
{
    move |input: &mut I| {
        (*parser).parse(input)
    }
}

#[macro_export]
macro_rules! alt {
    ($parser: expr, $($rest: expr),+) => {
        move|input: &mut _| {
            $crate::__alt_inner!(input, $parser, $($rest),+)
        }
    };
}

#[macro_export]
macro_rules! __alt_inner {
    ($input: expr, $parser: expr, $($rest: expr),+) => {
        match $parser.parse($input) {
            Ok(v) => Ok(v),
            Err(_) => $crate::__alt_inner!($input, $($rest),+)
        }
    };
    ($input: expr, $parser: expr) => {
        $parser.parse($input)
    };
}

#[macro_export]
macro_rules! seq {
    ($parser: expr, $($rest: expr),+) => {
        $crate::__seq_inner!($parser, $($rest),+)
    };
}

#[macro_export]
macro_rules! __seq_inner {
    ($($parser: expr),*) => {
        move |input: &mut _|{
            Ok((
                $(
                    match $parser.parse(input) {
                        Ok(v) => v,
                        Err(e) => return Err(e)
                    },
                )* 
            ))
        }
    };
}

#[macro_export]
macro_rules! skip {
    ($($front: expr,)+  @$reserve: expr $(, $($rest: expr),*)?) => {
        $crate::__skip_inner!($($front,)+ @$reserve, $($($rest),*)?)
    };

    (@$reserve: expr, $($rest: expr),+) => {
        $crate::__skip_inner!(@$reserve, $($rest),+)
    };
}

#[macro_export]
macro_rules! __skip_inner {
    ($($front: expr,)+  @$reserve: expr , $($rest: expr),*) => {
        move|input: &mut _| {
            
            $(
                $front.parse(input)?;
            )*

            let reserve = $reserve.parse(input);

            $(
                $rest.parse(input)?;
            )*

            reserve
        }
    };

    (@$reserve: expr , $($rest: expr),+) => {
        move|input: &mut _| {
            
            let reserve = $reserve.parse(input);

            $(
                $rest.parse(input)?;
            )*

            reserve
        }
    };
}

pub fn between<I, L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) -> impl Parser<I, Output = M::Output, Error = M::Error> 
    where L: Parser<I, Error = M::Error>,
        M: Parser<I>,
        R: Parser<I, Error = M::Error>
{
    move |input: &mut I| {
        lparser.parse(input)?;
        let m = mparser.parse(input)?;
        rparser.parse(input)?;
        Ok(m)
    }
}

pub fn pair<I, L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) ->  impl Parser<I, Output = (L::Output, R::Output), Error = L::Error>
    where L: Parser<I>,
        M: Parser<I, Error = L::Error>,
        R: Parser<I, Error = L::Error>
{
    move |input: &mut I| {
        let left = lparser.parse(input)?;
        mparser.parse(input)?;
        let right = rparser.parse(input)?;
        Ok((left, right))
    }
}

pub fn opt<I, P>(mut parser: P) -> impl Parser<I, Output = Option<P::Output>, Error = P::Error> 
    where I: Input,
        P: Parser<I>
{
    move |input: &mut I| {
        let mut cursor = input.cursor();
        match parser.parse(input) {
            Ok(v) => {
                Ok(Some(v))
            },
            Err(_) => {
                cursor.restore();
                Ok(None)
            }
        }
    }
}

pub fn opt_or_default<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error> 
    where I: Input,
        P: Parser<I>,
        <P as Parser<I>>::Output: Default
{
    move |input: &mut I| {
        let mut cursor = input.cursor();
        match parser.parse(input) {
            Ok(v) => {
                Ok(v)
            },
            Err(_) => {
                cursor.restore();
                Ok(Default::default())
            }
        }
    }
}

pub fn repeat<I, P>(mut parser: P, n: usize) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
    where P: Parser<I>
{
    move |input: &mut I| {
        let mut result = vec![];
        for _ in 0..n {
            result.push(parser.parse(input)?);
        }
        Ok(result)
    }
}

pub fn sepby<I, P, D>(mut parser: P, mut delim: D) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
    where I: Input, 
        P: Parser<I>, 
        D: Parser<I> 
{
    move |input: &mut I| {
        let mut result = vec![];

        loop {
            {
                let mut cursor = input.cursor();
                match parser.parse(input) {
                    Ok(v) => { result.push(v) },
                    Err(_) => { cursor.restore(); break }
                }
            }

            let mut cursor = input.cursor();
            if let Err(_) = delim.parse(input) {
                cursor.restore();
                break
            }
        }
        
        Ok(result)
    }
}

pub fn sepby1<I, P, D>(mut parser: P, mut delim: D) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
    where I: Input, 
        P: Parser<I>, 
        D: Parser<I> 
{
    move |input: &mut I| {
        let mut result = vec![];

        result.push(parser.parse(input)?);

        loop {
            
            {
                let mut cursor = input.cursor();
                if let Err(_) = delim.parse(input) {
                    cursor.restore();
                    break
                }
            }
            
            let mut cursor = input.cursor();
            match parser.parse(input) {
                Ok(v) => {result.push(v)},
                Err(_) => { cursor.restore(); break }
            }
        }
        
        Ok(result)
    }
}

pub fn many<I, P>(mut parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error>   
    where I: Input,
        P: Parser<I>  
{

    move |input: &mut I| {
        let mut result = vec![];

        loop {
            let mut cursor = input.cursor();
            match parser.parse(input) {
                Ok(v) => result.push(v),
                Err(_) => { cursor.restore(); break }
            }
        }

        Ok(result)
    }
}

pub fn many1<I, P>(mut parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
    where I: Input,
        P: Parser<I>  
{

    move |input: &mut I| {
        let mut result = vec![];

        result.push(parser.parse(input)?);

        loop {
            let mut cursor = input.cursor();
            match parser.parse(input) {
                Ok(v) => result.push(v),
                Err(_) => { cursor.restore(); break }
            }
        }

        Ok(result)
    }
}

pub fn range<I, P, R>(mut parser: P, range: R) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
    where I: Input,
        P: Parser<I>,
        R: RangeBounds<usize>  
{

    let min = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&i) => i,
        Bound::Excluded(_) => unreachable!()
    };

    let max = match range.end_bound() {
        Bound::Unbounded => None,
        Bound::Included(_) => unreachable!(),
        Bound::Excluded(&i) => {
            assert!(min <= i, "range({}..{}), is invalid", min, i);
            Some(i)
        }
    };

    move |input: &mut I| {
        let mut result = vec![];
        
        for _ in 0..min {
            let v = parser.parse(input)?;
            result.push(v);
        }

        if let Some(max) = max {
            for _ in min..max {
                let mut cursor = input.cursor();
                match parser.parse(input) {
                    Ok(v) => result.push(v),
                    Err(_) => { cursor.restore(); break }
                }
            }
        }else {
            loop {
                let mut cursor = input.cursor();
                match parser.parse(input) {
                    Ok(v) => result.push(v),
                    Err(_) => { cursor.restore(); break }
                }
            }
        }

        Ok(result)
    }
} 

pub fn ignore<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error> 
    where I: Input,
        P: Parser<I>  
{
    move |input: &mut I| {
        loop {
            let mut cursor = input.cursor();
            match parser.parse(input) {
                Ok(_) => {},
                Err(_) => { cursor.restore(); break }
            }
        }

        Ok(())
    }
}

pub fn peek<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error>
    where I: Input,
        P: Parser<I>
{
    move|input: &mut I| {
        let mut cursor = input.cursor();
        let res = parser.parse(input);
        cursor.restore();
        res
    }
}

pub fn pure<I, T: Clone, E>(v: T) -> impl Parser<I, Output = T, Error = E> {
    move|_input: &mut I| {
        Ok(v.clone())
    }
}