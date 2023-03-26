use crate::{
    Parser, 
    State
};


pub fn opt<U, P>(mut parser: P) -> impl Parser<U, Output = Option<P::Output>> where 
    U: Clone,
    P: Parser<U>
{
    move |state: &mut State<U>| {
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

pub fn opt_or<U, P>(mut parser: P, default: P::Output) -> impl Parser<U, Output = P::Output> where 
    U: Clone,
    P: Parser<U>,
    P::Output: Clone
{
    move |state: &mut State<U>| {
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


pub fn skip_many<U, P>(mut parser: P) -> impl Parser<U, Output = ()> where 
    U: Clone,
    P: Parser<U>  
{
    move |state: &mut State<U>| {
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

pub fn skip_many1<U, P>(mut parser: P) -> impl Parser<U, Output = ()> where 
    U: Clone,
    P: Parser<U>  
{
    move |state: &mut State<U>| {
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

pub fn sepby1<U, P, S>(mut parser: P, mut sep: S) -> impl Parser<U, Output = Vec<P::Output>> where
        U: Clone,
        P: Parser<U>, 
        S: Parser<U> 
{
    move |state: &mut State<U>| {
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


pub fn count<U, P>(mut parser: P, n: usize) -> impl Parser<U, Output = Vec<P::Output>> 
    where P: Parser<U>
{
    move |state: &mut State<U>| {
        let mut result = vec![];
        for _ in 0..n {
            result.push(parser.parse(state)?);
        }
        Ok(result)
    }
}

pub fn pure<U, T: Clone>(t: T) -> impl Parser<U, Output = T> {
    move|_state: &mut State<U>| {
        Ok(t.clone())
    }
}

pub fn attempt<U, P>(mut parser: P) -> impl Parser<U, Output = P::Output> where
    U: Clone,
    P: Parser<U>
{
    move |state: &mut State<U>| {
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