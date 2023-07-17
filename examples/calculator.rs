use rtor::{
    ParseResult,
    Parser,
    char::{
        char,
        unicode,
        ascii
    }, 
    combinator::{
        terminated,
        recognize,
        preceded,
        between,
        opt,
        alt,
        eof,
    }
};

fn main() {
    let result = terminated(
        expr.map(|e| e.eval()), 
        preceded(unicode::multi_space, eof)
    )("1 + 2 * ( 3 + 4 ) + 5 * 6");
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

fn expr(input: &str) -> ParseResult<Expr, &str> {
    let atom = token(alt((
        number.map(Expr::Value),
        parens(token(expr))
    )));

    let atom = preceded(
        unicode::multi_space, 
        alt((
            number.map(Expr::Value),
            between(char('('), expr, preceded(unicode::multi_space, char(')')))
        ))
    );

    atom
        .chainl1(|i| {
            let (op, i) = preceded(
                unicode::multi_space, 
                alt((char('*'), char('/')))
            )(i)?;
            Ok((move |l: Expr, r: Expr| Expr::Binary { op, left: Box::new(l), right: Box::new(r) }, i))
        })
        .chainl1(|i| {
            let (op, i) = preceded(
                unicode::multi_space, 
                alt((char('+'), char('-')))
            )(i)?;
            Ok((move |l: Expr, r: Expr| Expr::Binary { op, left: Box::new(l), right: Box::new(r) }, i))
        })
        .parse(input)
}

pub fn number(input: &str) -> ParseResult<f64, &str> {
    let exponent = (alt((char('e'), char('E'))), opt(alt((char('+'), char('-')))), ascii::multi_digit1);
    let fraction = (char('.'), ascii::multi_digit1);
    recognize((opt(char('-')), ascii::multi_digit1, opt(fraction), opt(exponent)))
        .map(|i: &str| i.parse().unwrap())
        .parse(input)
}