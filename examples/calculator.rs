use rtor::{
    ParseResult,
    Parser,
    token::{
        token,
        number,
        parens
    },
    char::char, 
    combinator::{
        alt,
        terminated,
        eof
    }
};

fn main() {
    let result = terminated(expr.map(|e| e.eval()), token(eof))("1 + 2 * ( 3 + 4 ) + 5 * 6");
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
                _ => panic!("invalid binary operator")
            }
        }
    }
}

fn expr(input: &str) -> ParseResult<Expr, &str> {
    let atom = token(alt((
        number.map(|i: &str| Expr::Value(i.parse::<f64>().unwrap())),
        parens(token(expr))
    )));

    atom
        .chainl1(|i| {
            let (op, i) = token(alt((char('*'), char('/'))))(i)?;
            Ok((move |l: Expr, r: Expr| Expr::Binary { op, left: Box::new(l), right: Box::new(r) }, i))
        })
        .chainl1(|i| {
            let (op, i) = token(alt((char('+'), char('-'))))(i)?;
            Ok((move |l: Expr, r: Expr| Expr::Binary { op, left: Box::new(l), right: Box::new(r) }, i))
        })
        .parse(input)
}