use rtor::{
    ParseResult,
    Parser,
    token::symbol,
    character::{
        ascii::digit,
        sat,
        char
    }, 
    combinator::{
        skip_many,
        skip_many1,
        recognize,
        opt,
        between
    }
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

fn number(input: &str) -> ParseResult<f64, &str> {
    let integer = char('0').or(sat(|ch| matches!(ch, '1'..='9')).andl(skip_many(digit)));
    let fraction = char('.').andr(skip_many1(digit));
    recognize(opt(char('-')).andl(integer).andl(opt(fraction)))
        .map(|i: &str| i.parse::<f64>().unwrap())
        .parse(input)
}

fn expr(input: &str) -> ParseResult<Expr, &str> {
    let atom = symbol(number.map(Expr::Value)
        .or(between(char('('), expr, char(')'))));

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