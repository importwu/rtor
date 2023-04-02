use crate::Input;

pub trait Parser<I: Input> {
    type Output;
    type Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error>;

    fn map<F, R>(self, f: F) -> Map<Self, F>
    where 
        F: FnMut(Self::Output) -> R,
        Self: Sized
    {
        Map { parser: self, f }
    }

    fn or<P>(self, bparser: P) -> Or<Self, P> 
    where
        P: Parser<I>,
        Self: Sized
    {
        Or { aparser: self, bparser }
    }

    fn and<P>(self, bparser: P) -> And<Self, P> 
    where
        P: Parser<I>,
        Self: Sized
    {
        And { aparser: self, bparser }
    }

    fn and_then<F, P>(self, f: F) -> AndThen<Self, F> 
    where
        P: Parser<I>,
        F: FnOnce(Self::Output) -> P + Clone,
        Self: Sized
    {
        AndThen { parser: self, f }
    }
}

impl<F, O, I: Input, E> Parser<I> for F where F: FnMut(I) -> Result<(O, I), E> {
    type Output = O;
    type Error = E;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        (self)(input)
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F
}


impl<I, P, F, R> Parser<I> for Map<P, F> 
where
    I: Input,
    P: Parser<I>,
    F: FnMut(P::Output) -> R
{
    type Output = R;
    type Error = P::Error;

    fn parse(&mut self, input: I) ->  Result<(Self::Output, I), Self::Error> {
        let (o, i) = self.parser.parse(input)?;
        Ok(((self.f)(o), i))
    }
}

pub struct Or<A, B> {
    aparser: A,
    bparser: B
}

impl<I, A, B> Parser<I> for Or<A, B> 
where
    I: Input,
    A: Parser<I>,
    B: Parser<I, Output = A::Output, Error = A::Error>
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        match self.aparser.parse(input.clone()) {
            Ok((o, i)) => Ok((o, i)),
            Err(_) => match self.bparser.parse(input) {
                Ok((o, i)) => Ok((o, i)),
                Err(e) => Err(e)
            }
        }
    }
}

pub struct And<A, B> {
    aparser: A,
    bparser: B
}

impl<I, A, B> Parser<I> for And<A, B> 
where
    I: Input,
    A: Parser<I>,
    B: Parser<I, Error = A::Error>
{
    type Output = B::Output;
    type Error = B::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (_, i) = self.aparser.parse(input)?;
        self.bparser.parse(i)
    }
}

pub struct AndThen<P, F> {
    parser: P,
    f: F
}

impl<I, A, B, F> Parser<I> for AndThen<A, F> 
where
    I: Input,
    A: Parser<I>,
    B: Parser<I, Error = A::Error>,
    F: FnOnce(A::Output) -> B + Clone
{
    type Output = B::Output;
    type Error = B::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (o, i) = self.parser.parse(input)?;
        (self.f.clone())(o).parse(i)
    }
}

mod test {


    use crate::primitive::char;

    use super::*;

    #[test]
    fn test() {
        let p = char('a')
            .or(char('b'))
            .or(char('c'))
            .parse("bsd");

        println!("{:?}", p)
    }
}