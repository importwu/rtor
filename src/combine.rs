use crate::{
    Input, 
    Parser, 
    iter::Many
};

#[inline]
pub fn ref_mut<I, P>(parser: &mut P) -> impl Parser<I, Output = P::Output, Error = P::Error> + '_ 
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
    
    move |mut input: I| {
        
        let it = input.many(ref_mut(&mut parser));

        let o = it.collect::<Vec<_>>();
        
        Ok((o, input))
    }
}


pub fn many1<I, P>(mut parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>
{

    move |input: I| {

        let (o, mut i) = parser.parse(input)?;

        let mut os = vec![o];

        let it = i.many(ref_mut(&mut parser));

        it.for_each(|o| os.push(o));

        Ok((os, i))
        
    }
}


pub fn skip_many<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |mut input: I| {
     
        let it = input.many(ref_mut(&mut parser));

        it.for_each(|_| ());

        Ok(((), input))
    }
}

pub fn skip_many1<I, P>(mut parser: P) -> impl Parser<I, Output = (), Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |input: I| {
        let (_, mut i) = parser.parse(input)?;

        let it = i.many(ref_mut(&mut parser));

        it.for_each(|_| ());

        Ok(((), i))
    }
}

pub fn sep_by<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
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

        let it = input.many(ref_mut(&mut sep).and(ref_mut(&mut parser)));

        it.for_each(|o| os.push(o));

        Ok((os, input))
    }
}

pub fn sep_by1<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |input: I| {
        
        let (o, mut i) = parser.parse(input)?;
        
        let mut os = vec![o];

        let it = i.many(ref_mut(&mut sep).and(ref_mut(&mut parser)));

        it.for_each(|o| os.push(o));

        Ok((os, i))
    }
}

pub fn end_by<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |mut input: I| { 
        let mut os = vec![];

        match followed_by(ref_mut(&mut parser), ref_mut(&mut sep)).parse(input.clone()) {
            Ok((o, i)) => {
                input = i;
                os.push(o);
            }
            Err(_) => return Ok((os, input))
        }

        let it = input.many(followed_by(ref_mut(&mut parser), ref_mut(&mut sep)));

        it.for_each(|o| os.push(o));

        Ok((os, input))
    }
}

pub fn end_by1<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |input: I| { 
        let (o, mut i) = followed_by(ref_mut(&mut parser), ref_mut(&mut sep)).parse(input.clone())?;

        let mut os = vec![o];

        let it = i.many(followed_by(ref_mut(&mut parser), ref_mut(&mut sep)));

        it.for_each(|o| os.push(o));

        Ok((os, i))
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

pub fn followed_by<I, A, B>(mut aparser: A, mut bparser: B) -> impl Parser<I, Output = A::Output, Error = A::Error> 
where
    I: Input,
    A: Parser<I>,
    B: Parser<I, Error = A::Error>
{
    move |input: I| {
        let (o, i) = aparser.parse(input)?;
        let (_, i) = bparser.parse(i)?;
        Ok((o, i))
    }
}
