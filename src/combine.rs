use crate::{
    Parser, 
    State
};


pub fn opt<P>(mut parser: P) -> impl Parser<Output = Option<P::Output>> where 
    P: Parser
{
    move |state: &mut State| {
        let mut state_cloned = state.clone();

        match parser.parse(&mut state_cloned) {
            Ok(v) => {
                *state = state_cloned;
                Ok(Some(v))
            },
            Err(_) => Ok(None)
        }
    }
}

pub fn opt_or<P>(mut parser: P, default: P::Output) -> impl Parser<Output = P::Output> where 
    P: Parser,
    P::Output: Clone
{
    move |state: &mut State| {
        let mut state_cloned = state.clone();

        match parser.parse(&mut state_cloned) {
            Ok(t) => {
                *state = state_cloned;
                Ok(t)
            },
            Err(_) => Ok(default.clone())
        }
    }
}

pub fn between<L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) -> impl Parser<Output = M::Output> where 
    L: Parser,
    M: Parser,
    R: Parser
{
    move |state: &mut State| {

        let _ = lparser.parse(state)?;
        let res= mparser.parse(state)?;
        let _ = rparser.parse(state)?;

        Ok(res)
    }
}

pub fn pair<L, M, R>(mut lparser: L, mut mparser: M, mut rparser: R) ->  impl Parser<Output = (L::Output, R::Output)>
    where L: Parser,
        M: Parser,
        R: Parser
{
    move |state: &mut State| {
        let left = lparser.parse(state)?;
        mparser.parse(state)?;
        let right = rparser.parse(state)?;
        Ok((left, right))
    }
}


pub fn many<P>(mut parser: P) -> impl Parser<Output = Vec<P::Output>> where
    P: Parser
{
    move |state: &mut State| {
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

pub fn many1<P>(mut parser: P) -> impl Parser<Output = Vec<P::Output>> where
    P: Parser
{
    move |state: &mut State| {
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


pub fn skip_many<P>(mut parser: P) -> impl Parser<Output = ()> where 
    P: Parser
{
    move |state: &mut State| {
        loop {
            let mut state_cloned = state.clone();

            match parser.parse(&mut state_cloned) {
                Ok(_) => *state = state_cloned,
                Err(_) => break
            }
        }

        Ok(())
    }
}

pub fn skip_many1<P>(mut parser: P) -> impl Parser<Output = ()> where 
    P: Parser  
{
    move |state: &mut State| {
        parser.parse(state)?;

        loop {
            let mut state_cloned = state.clone();

            match parser.parse(&mut state_cloned) {
                Ok(_) => *state = state_cloned,
                Err(_) => break
            }
        }

        Ok(())
    }
}



pub fn sepby<P, S>(mut parser: P, mut sep: S) -> impl Parser<Output = Vec<P::Output>> where
        P: Parser, 
        S: Parser
{
    move |state: &mut State| {
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

pub fn sepby1<P, S>(mut parser: P, mut sep: S) -> impl Parser<Output = Vec<P::Output>> where
        P: Parser, 
        S: Parser 
{
    move |state: &mut State| {
        let mut result = vec![];
       
        result.push(parser.parse(state)?);

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


pub fn count<P>(mut parser: P, n: usize) -> impl Parser<Output = Vec<P::Output>> 
    where P: Parser
{
    move |state: &mut State| {
        let mut result = vec![];
        for _ in 0..n {
            result.push(parser.parse(state)?);
        }
        Ok(result)
    }
}

pub fn pure<T: Clone>(t: T) -> impl Parser<Output = T> {
    move|_state: &mut State| {
        Ok(t.clone())
    }
}

pub fn attempt<P>(mut parser: P) -> impl Parser<Output = P::Output> where
    P: Parser
{
    move |state: &mut State| {
        let mut state_cloned = state.clone();
        match parser.parse(&mut state_cloned) {
            Ok(t) => {
                *state = state_cloned;
                Ok(t)
            }
            Err(e) => Err(e)
        }
    }
}



mod test {

    use crate::primitive::{char, digit};

    use super::*;

    #[test]
    fn test() {
        let mut state = State::new("[1,2,3,4,5,6]");

        let mut p = between(
            char('['), 
            sepby(digit(), char(',')), 
            char(']')
        );

        println!("{:?}", p.parse(&mut state));

        println!("{:?}", state);

    }
}