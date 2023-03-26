use rtor::{
    ParseError,
    State
};

fn main() {
    let state = State::new("abcdf");

    // let s = satisfy(|t| *t == 'a');
    satisfy(|t| *t == 'c').clone();

    let (o, s) = string("ab").parse(state).unwrap();

    println!("{:?}", o);

    println!("{:?}", s)
}

type ParseResult<'a, O, U> = Result<(O, State<'a, U>), ParseError>;
pub trait Parser<'a, U>: Clone {

    type Output;

    fn parse(self, state: State<'a, U>) -> ParseResult<'a, Self::Output, U>;
}

impl<'a, F, U, O> Parser<'a, U> for F where 
    F: FnOnce(State<'a, U>) -> ParseResult<'a, O, U> + Clone,
{
    type Output = O;

    fn parse(self, state: State<'a, U>) -> ParseResult<'a, Self::Output, U> {
        (self)(state)
    }
}

pub fn satisfy<'a, F, U>(f: F) -> impl Parser<'a, U, Output = char> where 
    F: FnOnce(&char) -> bool + Clone
{
    move |mut state: State<'a, U>| {
        let pos = state.pos();
        match state.next() {
            None => Err(ParseError{pos, expect: vec![], unexpect: None}),
            Some(ch) if f(&ch) => Ok((ch, state)),
            Some(ch) => Err(ParseError{pos, expect: vec![], unexpect: Some(ch.into())})
        }
    }
}

pub fn string<'a, U>(string: &'a str) -> impl Parser<U, Output = String> + 'a {
    move |mut state: State<'a, U>| {
        for ch in string.chars() {
            let pos = state.pos();
            match state.next() {
                None => return Err(ParseError{pos, expect: vec![ch.into()], unexpect: None}),
                Some(t) if t == ch => continue,
                Some(t) => return Err(ParseError{pos, expect: vec![ch.into()], unexpect: Some(t.into())})
            }
        }
        return Ok((string.to_owned(), state))
    }
}