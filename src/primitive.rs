use crate::{
    Parser,
    ParseError, State
};

pub fn satisfy<F, U>(mut f: F) -> impl Parser<U, Output = char> where 
    F: FnMut(&char) -> bool 
{
    move |state: &mut State<U>| {
        let pos = state.pos();
        match state.next() {
            None => Err(ParseError{pos, expect: vec![], unexpect: None}),
            Some(ch) if f(&ch) => Ok(ch),
            Some(ch) => Err(ParseError{pos, expect: vec![], unexpect: Some(ch.into())})
        }
    }
}

pub fn char<U>(ch: char) -> impl Parser<U, Output = char> {
    satisfy(move |t| *t == ch).expect(&ch.to_string())
}

pub fn digit<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_digit).expect("digit")
}

mod test {

    use super::*;

    #[test]
    fn test() {

        let mut state = State::new("bdf");

        let mut a = digit()
            .or(char('v'))
            .or(char('a'));

        println!("{}", a.parse(&mut state).err().unwrap());
        println!("{:?}", state)

    }
}