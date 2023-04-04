use crate::{
    Input, 
    Parser, 
    iterator
};

#[inline]
pub fn refmut<I, P>(parser: &mut P) -> impl Parser<I, Output = P::Output, Error = P::Error> + '_ 
where
    I: Input,
    P: Parser<I>
{
    #[inline]
    move |input: I| {
        (*parser).parse(input)
    }
}

pub fn opt<I, P>(mut parser: P) -> impl Parser<I, Output = Option<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>
{
    move |input: I| {

        match parser.parse(input.clone()) {
            Ok((o, i)) => {
                Ok((Some(o), i))
            },
            Err(_) => Ok((None, input))
        }
    }
}

pub fn opt_or<I, P>(mut parser: P, default: P::Output) -> impl Parser<I, Output = P::Output, Error = P::Error> 
where
    I: Input,
    P: Parser<I>,
    P::Output: Clone
{
    move |input: I| {

        match parser.parse(input.clone()) {
            Ok(t) => {
                Ok(t)
            },
            Err(_) => Ok((default.clone(), input))
        }
    }
}


pub fn between<I, L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) -> impl Parser<I, Output = M::Output, Error = L::Error> 
where
    I: Input,
    L: Parser<I>,
    M: Parser<I, Error = L::Error>,
    R: Parser<I, Error = L::Error>
{
    move |input: I| {

        let (_, i) = lparser.parse(input)?;
        let (o, i)= mparser.parse(i)?;
        let (_, i) = rparser.parse(i)?;

        Ok((o, i))
    }
}

pub fn pair<I, L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) ->  impl Parser<I, Output = (L::Output, R::Output), Error = L::Error>
where 
    I: Input,
    L: Parser<I>,
    M: Parser<I, Error = L::Error>,
    R: Parser<I, Error = L::Error>
{
    move |input: I| {
        let (o1, i) = lparser.parse(input)?;
        let (_, i) = mparser.parse(i)?;
        let (o2, i) = rparser.parse(i)?;
        Ok(((o1, o2), i))
    }
}


pub fn many<I, P>(mut parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>
{
    
    move |input: I| {
        
        let mut it = iterator(input, refmut(&mut parser));

        let o = it.collect::<Vec<_>>();
        
        Ok((o, it.into_input()))
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

        let mut it = iterator(i, refmut(&mut parser));

        it.for_each(|o| os.push(o));

        Ok((os, it.into_input()))
        
    }
}


pub fn skip_many<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |input: I| {
     
        let mut it = iterator(input, refmut(&mut parser));

        it.for_each(|_| ());

        Ok(((), it.into_input()))
    }
}

pub fn skip_many1<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        let (_, i) = parser.parse(input)?;

        let mut it = iterator(i, refmut(&mut parser));

        it.for_each(|_| ());

        Ok(((), it.into_input()))
    }
}

pub fn sepby<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |mut input: I| {
        let mut os = vec![];

        match parser.parse(input.clone()) {
            Ok((o, i)) => {
                input = i;
                os.push(o);
            }
            Err(_) => return Ok((os, input))
        }

        let mut it = iterator(input, refmut(&mut sep).and(refmut(&mut parser)));

        it.for_each(|o| os.push(o));

        Ok((os, it.into_input()))
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

        let mut it = iterator(i, refmut(&mut sep).and(refmut(&mut parser)));

        it.for_each(|o| os.push(o));

        Ok((os, it.into_input()))
    }
}


pub fn count<I, P>(mut parser: P, n: usize) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |mut input: I| {
        let mut os = vec![];
        for _ in 0..n {
            let (o, i) = parser.parse(input)?; 
            input = i;
            os.push(o);
        }
        Ok((os, input))
    }
}

pub fn pure<I, T, E>(t: T) -> impl Parser<I, Output = T, Error = E> 
where
    I: Input,
    T: Clone
{
    move|input: I| {
        Ok((t.clone(), input))
    }
}


mod test {

    use crate::primitive::digit;

    use super::*;

    #[test]
    fn test() {

        let mut parser = between(
            '[', 
            sepby(digit, ','), 
            ']'
          );
          
        // assert_eq!(parser.parse("[1,2,3,4,5,6]").unwrap(), (vec!['1','2','3','4','5','6'], ""));

          println!("{:?}", parser.parse("[1,2,3]"))
    }
}