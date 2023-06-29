use rtor::{
    ParseResult,
    Parser,
    token::{
        symbol,
        float,
        parens
    },
    character::char, 
};

fn main() {
    let v = expr.map(|e| e.eval()).parse("1+2*(3+4)+5*6");
    println!("{:#?}", v)
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
    let atom = symbol(float.map(|i: &str| Expr::Value(i.parse::<f64>().unwrap()))
        .or(parens(expr)));

    atom.chainl1(|i| {
        let (op, i) = symbol(char('*').or(char('/'))).parse(i)?;
        Ok((move |l: Expr, r: Expr| Expr::Binary { op, left: Box::new(l), right: Box::new(r) }, i))
    })
    .chainl1(|i| {
        let (op, i) = symbol(char('+').or(char('-'))).parse(i)?;
        Ok((move |l: Expr, r: Expr| Expr::Binary { op, left: Box::new(l), right: Box::new(r) }, i))
    })
    .parse(input)
}