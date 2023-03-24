use crate::{
    Parser,
    ParseError, State
};

pub fn satisfy<F, U>(mut f: F) -> impl Parser<U, Output = char> where 
    F: FnMut(&char) -> bool 
{
    move |state: &mut State<U>| {
        match state.next() {
            None => Err(ParseError::Expect { expect: None, found: None }),
            Some(ch) if f(&ch) => Ok(ch),
            Some(ch) => Err(ParseError::Expect { expect: None, found: Some(ch.into()) })
        }
    }
}

pub fn char<U>(ch: char) -> impl Parser<U, Output = char> where 
{
    satisfy(move |t| *t == ch)
}

pub fn digit<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_digit)
}

mod test {

    use super::*;

    #[test]
    fn test() {

        let mut state = State::new("bdf");

        let mut a = char('a')
            .or(char('b'));

        println!("{:?}", a.parse(&mut state));
        println!("{:?}", state)

    }
}