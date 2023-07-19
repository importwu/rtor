use rtor::{
    Input,
    ParseResult,
    Parser,
    ParseError,
    AsChar,
    char::{
        char,
        ascii
    }, 
    combinator::{
        terminated,
        recognize,
        between,
        opt,
        alt,
        eof,
    },
};

fn main() {
    let result = calc("1 + 2 * ( 3 + 4 ) + 5 * 6");
    assert_eq!(result, Ok((45.0, "")));
}

#[derive(Debug)]
enum Expr {
    Value(f64),
    Binary {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>
    }
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Self::Value(v) => *v,
            Self::Binary { op, left, right } => match op {
                '+' => left.eval() + right.eval(),
                '-' => left.eval() - right.eval(),
                '*' => left.eval() * right.eval(),
                '/' => left.eval() / right.eval(),
                _ => panic!("unknown binary operator")
            }
        }
    }
}

fn calc(input: &str) -> ParseResult<f64, &str> {
    between(
        ascii::multi_space, 
        expr.map(|e| e.eval()), 
        eof
    )(input)
}

fn expr(input: &str) -> ParseResult<Expr, &str> {
    let atom = alt((
        number.map(Expr::Value),
        between(lexeme(char('(')), expr, lexeme(char(')')))
    ));

    atom.chainl1(|i| {
            let (op, i) = alt((lexeme(char('*')), lexeme(char('/'))))(i)?;
            Ok((move |l: Expr, r: Expr| Expr::Binary { op, left: Box::new(l), right: Box::new(r) }, i))
        })
        .chainl1(|i| {
            let (op, i) = alt((lexeme(char('+')), lexeme(char('-'))))(i)?;
            Ok((move |l: Expr, r: Expr| Expr::Binary { op, left: Box::new(l), right: Box::new(r) }, i))
        })
        .parse(input)
}

pub fn number(input: &str) -> ParseResult<f64, &str> {
    let exponent = (alt((char('e'), char('E'))), opt(alt((char('+'), char('-')))), ascii::multi_digit1);
    let fraction = (char('.'), ascii::multi_digit1);
    lexeme(recognize((opt(char('-')), ascii::multi_digit1, opt(fraction), opt(exponent)))
        .map(|i: &str| i.parse().unwrap()))(input)
}

pub fn lexeme<P, I, E>(parser: P) -> impl FnMut(I) -> ParseResult<P::Output, I, E> 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>,
    P: Parser<I, E>,
{
    let mut parser = terminated(parser, ascii::multi_space);
    move |input| parser(input)
}