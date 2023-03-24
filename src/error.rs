#[derive(Debug)]
pub enum ParseError {
    Multi(Vec<ParseError>),
    Expect {
        expect: Option<String>,
        found: Option<String>
    }
}

impl ParseError {

    pub fn merge(self, other: Self) -> Self {
        match self {
            Self::Multi(mut errs) => match other {
                Self::Multi(mut other_errs) => {
                    errs.append(&mut other_errs);
                    ParseError::Multi(errs)
                }
                other_err@Self::Expect { expect: _, found: _ }  => {
                    errs.push(other_err);
                    ParseError::Multi(errs)
                }
            },
            err@Self::Expect { expect: _, found: _ } => match other {
                Self::Multi(mut other_errs) => {
                    let mut errs = vec![err];
                    errs.append(&mut other_errs);
                    ParseError::Multi(errs)
                }
                other_err@Self::Expect { expect:_, found:_} => {
                    let mut errs = vec![err];
                    errs.push(other_err);
                    ParseError::Multi(errs)
                }
            }
        }
    }

    fn expect(self, msg: &str) -> Self {
        todo!()
    }
}


mod err {
    use crate::Pos;


    #[derive(Debug)]
    pub struct ParseError {
        pos: Pos,
        // ex
    }
}