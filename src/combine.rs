use crate::{
    Input, 
    Parser, 
    Error, 
    AsChar, 
    ParseError, 
    iter::Many, 
    primitive::ascii::space, 
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
    P: Parser<I>
{
    move |mut input: I| {
        
        let it = Many::new(&mut input, parser.ref_mut());
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

        let it = Many::new(&mut i, parser.ref_mut());
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

        let it = Many::new(&mut input, parser.ref_mut());
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

        let it = Many::new(&mut i, parser.ref_mut());
        it.for_each(|_| ());

        Ok(((), i))
    }
}

pub fn skip<I, P>(mut parser: P, n: usize) -> impl Parser<I, Output = (), Error = P::Error> 
where 
    I: Input,
    P: Parser<I>
{
    move |mut input: I| {
        for _ in 0..n {
            let (_, i) = parser.parse(input)?; 
            input = i;
        }
        Ok(((), input))
    }
}

pub fn sepby<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |input: I| {
        let (mut os, mut i) = match parser.parse(input.clone()) {
            Ok((o, i)) => (vec![o], i),
            Err(_) => return Ok((vec![], input))
        };

        let it = Many::new(&mut i, sep.ref_mut().andr(parser.ref_mut()));
        it.for_each(|o| os.push(o));

        Ok((os, i))
    }
}


pub fn sepby1<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |input: I| {
        let (o, mut i) = parser.parse(input)?;
        let mut os = vec![o];
       
        let it = Many::new(&mut i, sep.ref_mut().andr(parser.ref_mut()));
        it.for_each(|o| os.push(o));

        Ok((os, i))
    }
}

pub fn endby<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |input: I| { 
        let (mut os, mut i) = match parser.ref_mut().andl(sep.ref_mut()).parse(input.clone()) {
            Ok((o, i)) => (vec![o], i),
            Err(_) => return Ok((vec![], input))
        };

        let it = Many::new(&mut i, parser.ref_mut().andl(sep.ref_mut()));
        it.for_each(|o| os.push(o));

        Ok((os, i))
    }
}

pub fn endby1<I, P, S>(mut parser: P, mut sep: S) -> impl Parser<I, Output = Vec<P::Output>, Error = P::Error> 
where
    I: Input,
    P: Parser<I>, 
    S: Parser<I, Error = P::Error>
{
    move |input: I| { 
        let (o, mut i) = parser.ref_mut().andl(sep.ref_mut()).parse(input.clone())?;
        let mut os = vec![o];

        let it = Many::new(&mut i, parser.ref_mut().andl(sep.ref_mut()));
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
            Ok(_) => Err(Error::from_token(input.next()))
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

pub fn token<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = ParseError<I::Token>> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = ParseError<I::Token>>
{
    move |input: I| {
        let (_, i) = skip_many(space).parse(input)?;
        parser.parse(i)
    }
}
