use crate::{
    Input, 
    Parser
};

pub struct Iter<I, P> {
    input: I,
    parser: P,
    flag: bool
}

impl<'a, I, P> Iterator for &'a mut Iter<I, P> 
where
    I: Input,
    P: Parser<I>
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {

        if self.flag { return None }

        match self.parser.parse(self.input.clone()) {
            Ok((o, i)) => {
                self.input = i;
                Some(o)
            }
            Err(_) => {
                self.flag = true;
                None
            }
        }

    }
}

pub fn iterator<I, P>(input: I, parser: P) -> Iter<I, P> 
where
    I: Input,
    P: Parser<I>
{
    Iter { 
        input, 
        parser,
        flag: false 
    }
}


impl<I, P> Iter<I, P> {
    pub fn into_input(self) -> I {
        self.input
    }
}

