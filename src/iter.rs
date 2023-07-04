use crate::{
    Input, 
    Parser
};

#[derive(Debug)]
pub struct ParserIter<I, E, P> {
    input: I,
    parser: P,
    error: Option<E>,
}

impl<I, E, P> ParserIter<I, E, P> {
    pub fn new(input: I, parser: P) -> Self {
        Self { 
            input, 
            parser, 
            error: None,
        }
    }

    pub fn get(self) -> I {
        self.input
    }

    pub fn try_get(self) -> Result<I, E> {
        match self.error {
            None => Ok(self.input),
            Some(e) => Err(e)
        }
    }
}

impl<I, E, P> Iterator for &mut ParserIter<I, E, P> 
where
    I: Input,
    P: Parser<I, E>
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error.is_some() { return None }

        match self.parser.parse(self.input.clone()) {
            Ok((o, i)) => {
                self.input = i;
                Some(o)
            }
            Err(e) => {
                self.error = Some(e);
                None
            }
        }
    }
}