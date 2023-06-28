use rtor::{
    ParseResult,
    ParseError,
    Parser,
    primitive::{
        ascii::digit,
        sat,
        char
    }, 
    combine::{
        skip_many,
        skip_many1,
        recognize,
        opt,
        token,
        between
    }
};

fn main() {
    let v = expr(0).map(|e| e.eval()).parse("1+2*(3+4)+5*6");
    println!("{:?}", v)
}

#[derive(Debug)]
enum Expr {
    Value(f64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Self::Value(v) => *v,
            Self::Plus(x, y) => x.eval() + y.eval(),
            Self::Minus(x, y) => x.eval() - y.eval(),
            Self::Divide(x, y) => x.eval() / y.eval(),
            Self::Multiply(x, y) => x.eval() * y.eval(),
        }
    }
}

fn operator(input: &str) -> ParseResult<(char, u8), &str> {
    char('+').map(|c| (c, 1))
        .or(char('-').map(|c| (c, 1)))
        .or(char('*').map(|c| (c, 2)))
        .or(char('/').map(|c| (c, 2)))
        .parse(input)
}

fn number(input: &str) -> ParseResult<f64, &str> {
    let integer = char('0').or(sat(|ch| matches!(ch, '1'..='9')).andl(skip_many(digit)));
    let fraction = char('.').andr(skip_many1(digit));
    recognize(opt(char('-')).andl(integer).andl(opt(fraction)))
        .map(|i: &str| i.parse::<f64>().unwrap())
        .parse(input)
}

//pratt parser
fn expr<'a>(precedence: u8) -> impl Parser<&'a str, Output = Expr, Error = ParseError<&'a str>> {
    move |input: &'a str| {
        let (mut left, mut input) = token(number.map(Expr::Value)
            .or(between(char('('), expr(0), char(')'))))
            .parse(input)?;

        while let Ok(((op, next_precedence), i)) = token(operator).parse(input.clone()) {
            if next_precedence <= precedence { break }
            let (right, i) = expr(next_precedence).parse(i)?;
            left = match op {
                '+' => Expr::Plus(Box::new(left), Box::new(right)),
                '-' => Expr::Minus(Box::new(left), Box::new(right)),
                '*' => Expr::Multiply(Box::new(left), Box::new(right)),
                '/' => Expr::Divide(Box::new(left), Box::new(right)),
                _ => unreachable!()
            };
            input = i;
        }

        Ok((left, input))
    }
}