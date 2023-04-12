use crate::{
    Input, 
    Parser
};

pub struct Many<'a, I, P> {
    input: &'a mut I,
    parser: P,
    flag: bool
}

impl<'a, I, P> Many<'a, I, P> {
    pub fn new(input: &'a mut I, parser: P) -> Self {
        Self { 
            input, 
            parser, 
            flag: false 
        }
    }
}

impl<'a, I, P> Iterator for Many<'a, I, P> 
where
    I: Input,
    P: Parser<I>
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {

        if self.flag { return None }

        match self.parser.parse(self.input.clone()) {
            Ok((o, i)) => {
                *self.input = i;
                Some(o)
            }
            Err(_) => {
                self.flag = true;
                None
            }
        }

    }
}