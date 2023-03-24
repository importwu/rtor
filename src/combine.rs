use std::ops::{
    RangeBounds, 
    Bound
};

use crate::{Parser, State
};


// pub fn attempt<I, P>(mut parser: P)  -> impl Parser<I, Output = P::Output, Error = P::Error>
//     where I: Input,
//         P: Parser<I, Error = ParseError>
// {
//     move |input: &mut I| {
//         let mut cursor = input.cursor();
//         match parser.parse(&mut cursor) {
//             Ok(t) => Ok(t),
//             Err(e) => match &e.message {
//                 Message::Fatal(_) => Err(e),
//                 _ => {
//                     cursor.rollback();
//                     Err(e)
//                 }
//             }
//         }
//     }
// }


// #[inline]
// pub fn refer<'a, I, P>(parser: &'a mut P) -> impl Parser<I, Output = P::Output, Error = P::Error> + 'a
//     where P: Parser<I>
// {
//     move |input: &mut I| {
//         (*parser).parse(input)
//     }
// }



// pub fn between<I, L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) -> impl Parser<I, Output = M::Output, Error = M::Error> 
//     where L: Parser<I, Error = M::Error>,
//         M: Parser<I, Error = ParseError>,
//         R: Parser<I, Error = M::Error>
// {
//     move |input: &mut I| {
//         lparser.parse(input)?;
//         let m = mparser.parse(input).map_err(|e| e.into_consume())?;
//         rparser.parse(input).map_err(|e| e.into_consume())?;
//         Ok(m)
//     }
// }

// pub fn pair<I, L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) ->  impl Parser<I, Output = (L::Output, R::Output), Error = L::Error>
//     where L: Parser<I>,
//         M: Parser<I, Error = L::Error>,
//         R: Parser<I, Error = L::Error>
// {
//     move |input: &mut I| {
//         let left = lparser.parse(input)?;
//         mparser.parse(input)?;
//         let right = rparser.parse(input)?;
//         Ok((left, right))
//     }
// }

// pub fn opt<I, P>(mut parser: P) -> impl Parser<I, Output = Option<P::Output>, Error = P::Error> 
//     where I: Input,
//         P: Parser<I>
// {
//     move |input: &mut I| {
//         match parser.parse(input) {
//             Ok(v) => {
//                 Ok(Some(v))
//             },
//             Err(_) => {
//                 Ok(None)
//             }
//         }
//     }
// }

// pub fn opt_or_default<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error> 
//     where I: Input,
//         P: Parser<I>,
//         <P as Parser<I>>::Output: Default
// {
//     move |input: &mut I| {
//         let mut cursor = input.cursor();
//         match parser.parse(&mut cursor) {
//             Ok(v) => {
//                 Ok(v)
//             },
//             Err(_) => {
//                 cursor.rollback();
//                 Ok(Default::default())
//             }
//         }
//     }
// }

// pub fn repeat<I, P>(mut parser: P, n: usize) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
//     where P: Parser<I>
// {
//     move |input: &mut I| {
//         let mut result = vec![];
//         for _ in 0..n {
//             result.push(parser.parse(input)?);
//         }
//         Ok(result)
//     }
// }

// pub fn sepby<I, P, D>(mut parser: P, mut delim: D) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
//     where I: Input, 
//         P: Parser<I>, 
//         D: Parser<I> 
// {
//     move |input: &mut I| {
//         let mut result = vec![];

//         match parser.parse(input) {
//             Ok(t) => result.push(t),
//             Err(_) => return Ok(result)
//         }

//         loop {
            
//             if let Err(_) = delim.parse(input) {
//                 break
//             }

//             result.push(parser.parse(input)?);
//         }

//         Ok(result)
//     }
// }

// pub fn sepby1<I, P, D>(mut parser: P, mut delim: D) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
//     where I: Input, 
//         P: Parser<I>, 
//         D: Parser<I> 
// {
//     move |input: &mut I| {
//         let mut result = vec![];

//         result.push(parser.parse(input)?);

//         loop {
            
//             {
//                 if let Err(_) = delim.parse(input) {
//                     break
//                 }
//             }
            
//             result.push(parser.parse(input)?);
//         }
        
//         Ok(result)
//     }
// }

// pub fn many<I, P>(mut parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error>   
//     where I: Input,
//         P: Parser<I>  
// {

//     move |input: &mut I| {
//         let mut result = vec![];

//         loop {
//             match parser.parse(input) {
//                 Ok(v) => result.push(v),
//                 Err(_) => break
//             }
//         }

//         Ok(result)
//     }
// }

// pub fn many1<I, P>(mut parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
//     where I: Input,
//         P: Parser<I>  
// {

//     move |input: &mut I| {
//         let mut result = vec![];

//         result.push(parser.parse(input)?);

//         loop {
//             match parser.parse(input) {
//                 Ok(v) => result.push(v),
//                 Err(_) => break
//             }
//         }

//         Ok(result)
//     }
// }

// pub fn range<I, P, R>(mut parser: P, range: R) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
//     where I: Input,
//         P: Parser<I>,
//         R: RangeBounds<usize>  
// {

//     let min = match range.start_bound() {
//         Bound::Unbounded => 0,
//         Bound::Included(&i) => i,
//         Bound::Excluded(_) => unreachable!()
//     };

//     let max = match range.end_bound() {
//         Bound::Unbounded => None,
//         Bound::Included(_) => unreachable!(),
//         Bound::Excluded(&i) => {
//             assert!(min <= i, "range({}..{}), is invalid", min, i);
//             Some(i)
//         }
//     };

//     move |input: &mut I| {
//         let mut result = vec![];
        
//         for _ in 0..min {
//             let v = parser.parse(input)?;
//             result.push(v);
//         }

//         if let Some(max) = max {
//             for _ in min..max {
//                 match parser.parse(input) {
//                     Ok(v) => result.push(v),
//                     Err(_) => break
//                 }
//             }
//         }else {
//             loop {
//                 match parser.parse(input) {
//                     Ok(v) => result.push(v),
//                     Err(_) => break
//                 }
//             }
//         }

//         Ok(result)
//     }
// } 

// pub fn ignore<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error> 
//     where I: Input,
//         P: Parser<I>  
// {
//     move |input: &mut I| {
//         loop {
//             match parser.parse(input) {
//                 Ok(_) => (),
//                 Err(_) => break
//             }
//         }

//         Ok(())
//     }
// }

// pub fn peek<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error>
//     where I: Input,
//         P: Parser<I>
// {
//     move|input: &mut I| {
//         let mut cursor = input.cursor();
//         let res = parser.parse(&mut cursor);
//         cursor.rollback();
//         res
//     }
// }

// pub fn pure<I, T: Clone, E>(v: T) -> impl Parser<I, Output = T, Error = E> {
//     move|_input: &mut I| {
//         Ok(v.clone())
//     }
// }



pub fn between<U, L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) -> impl Parser<U, Output = M::Output> where 
    L: Parser<U>,
    M: Parser<U>,
    R: Parser<U>
{
    move |state: &mut State<U>| {

        let _ = lparser.parse(state)?;
        let res= mparser.parse(state)?;
        let _ = rparser.parse(state)?;

        Ok(res)
    }
}

pub fn pair<U, L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) ->  impl Parser<U, Output = (L::Output, R::Output)>
    where L: Parser<U>,
        M: Parser<U>,
        R: Parser<U>
{
    move |state: &mut State<U>| {
        let left = lparser.parse(state)?;
        mparser.parse(state)?;
        let right = rparser.parse(state)?;
        Ok((left, right))
    }
}


pub fn many<U, P>(mut parser: P) -> impl Parser<U, Output = Vec<P::Output>> where
    U: Clone,
    P: Parser<U>
{
    move |state: &mut State<U>| {
        let mut res: Vec<P::Output> = vec![];

        loop {
            let mut state_cloned = state.clone();
            match parser.parse(&mut state_cloned) {
                Ok(t) => {
                    *state = state_cloned;
                    res.push(t);    
                }
                Err(_) => break
            }
        }

        Ok(res)
    }
}

pub fn many1<U, P>(mut parser: P) -> impl Parser<U, Output = Vec<P::Output>> where
    U: Clone,
    P: Parser<U>
{
    move |state: &mut State<U>| {
        let mut res: Vec<P::Output> = vec![];

        res.push(parser.parse(state)?);

        loop {
            let mut state_cloned = state.clone();
            match parser.parse(&mut state_cloned) {
                Ok(t) => {
                    *state = state_cloned;
                    res.push(t);    
                }
                Err(_) => break
            }
        }

        Ok(res)
    }
}

pub fn sepby<U, P, S>(mut parser: P, mut sep: S) -> impl Parser<U, Output = Vec<P::Output>> where
        U: Clone,
        P: Parser<U>, 
        S: Parser<U> 
{
    move |state: &mut State<U>| {
        let mut result = vec![];

        let mut state_cloned = state.clone();

        match parser.parse(&mut state_cloned) {
            Ok(t) => {
                *state = state_cloned;
                result.push(t);
            },
            Err(_) => return Ok(result)
        }

        loop {
            let mut state_cloned = state.clone();

            if let Err(_) = sep.parse(&mut state_cloned) {
                break
            }

            match parser.parse(&mut state_cloned) {
                Ok(t) => {
                    *state = state_cloned;
                    result.push(t);
                },
                Err(_) => return Ok(result)
            }
        }

        Ok(result)
    }
}

mod test {

    use crate::primitive::char;

    use super::*;

    #[test]
    fn test() {
        let mut state = State::new("1,1aaaaa1avaaa");

        let mut p = sepby(char('1'), char(','));

        println!("{:?}", p.parse(&mut state));

        println!("{:?}", state)
    }
}