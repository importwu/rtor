use super::{
    Parser,
    Error,
    Alt
};

macro_rules! tuple_parser_impl {
    ($a: ident, $($rest: ident),+) => {
        impl<Input, $a, $($rest),+> Parser<Input> for ($a, $($rest),+) 
        where
            $a: Parser<Input>,
            $($rest: Parser<Input, Error = $a::Error>),+
        {
            type Output = ($a::Output, $($rest::Output),+);
            type Error = $a::Error;

            fn parse(&mut self, input: Input) -> Result<(Self::Output, Input), Self::Error> {
                tuple_parser_inner!(o1, 0, self, input, (), $a, $($rest),+);
            }
        }
    };
}

macro_rules! tuple_parser_inner {
    ($o:ident, $field:tt, $self:expr, $input:expr, (), $a:ident, $($rest:ident),*) => {
        let ($o, i) = $self.$field.parse($input)?;
        succ_tuple_parer_inner!($field, $self, i, ($o), $($rest),*)
    };
    ($o:ident, $field:tt, $self:expr, $input:expr, ($($os:tt)*), $a:ident, $($rest:ident),*) => {
        let ($o, i) = $self.$field.parse($input)?;
        succ_tuple_parer_inner!($field, $self, i, ($($os)*, $o), $($rest),*)
    };
    ($o:ident, $field:tt, $self:expr, $input:expr, ($($os:tt)*), $a:ident) => {
        let ($o, i) = $self.$field.parse($input)?;
        return Ok((($($os)*, $o), i));
    };
}

macro_rules! succ_tuple_parer_inner {
    (0, $($p:tt),*) => (tuple_parser_inner!(o2, 1, $($p),*));
    (1, $($p:tt),*) => (tuple_parser_inner!(o3, 2, $($p),*));
    (2, $($p:tt),*) => (tuple_parser_inner!(o4, 3, $($p),*));
    (3, $($p:tt),*) => (tuple_parser_inner!(o5, 4, $($p),*));
    (4, $($p:tt),*) => (tuple_parser_inner!(o6, 5, $($p),*));
    (5, $($p:tt),*) => (tuple_parser_inner!(o7, 6, $($p),*));
    (6, $($p:tt),*) => (tuple_parser_inner!(o8, 7, $($p),*));
    (7, $($p:tt),*) => (tuple_parser_inner!(o9, 8, $($p),*));
    (8, $($p:tt),*) => (tuple_parser_inner!(o10, 9, $($p),*));
    (9, $($p:tt),*) => (tuple_parser_inner!(o11, 10, $($p),*));
    (10, $($p:tt),*) => (tuple_parser_inner!(o12, 11, $($p),*));
    (11, $($p:tt),*) => (tuple_parser_inner!(o13, 12, $($p),*));
    (12, $($p:tt),*) => (tuple_parser_inner!(o14, 13, $($p),*));
    (13, $($p:tt),*) => (tuple_parser_inner!(o15, 14, $($p),*));
    (14, $($p:tt),*) => (tuple_parser_inner!(o16, 15, $($p),*));
    (15, $($p:tt),*) => (tuple_parser_inner!(o17, 16, $($p),*));
    (16, $($p:tt),*) => (tuple_parser_inner!(o18, 17, $($p),*));
    (17, $($p:tt),*) => (tuple_parser_inner!(o19, 18, $($p),*));
    (18, $($p:tt),*) => (tuple_parser_inner!(o20, 19, $($p),*));
    (19, $($p:tt),*) => (tuple_parser_inner!(o21, 20, $($p),*));
    (20, $($p:tt),*) => (tuple_parser_inner!(o22, 21, $($p),*));
}

macro_rules! tuple_parser {
    ($a:ident, $b:ident, $($rest:ident),*) => {
        tuple_parser_impl!($a, $b, $($rest),*);
        tuple_parser!($b, $($rest),*);
    };
    ($a:ident, $b:ident) => {
        tuple_parser_impl!($a, $b);
    }
}

tuple_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);

macro_rules! alt_parser_impl {
    ($a: ident, $($rest: ident),+) => {
        impl<Input, $a, $($rest),+> Alt<Input> for ($a, $($rest),+) 
        where
            Input: super::Input,
            $a: Parser<Input>,
            $a::Error: Error<Input>,
            $($rest: Parser<Input, Error = $a::Error, Output = $a::Output>),+
        {   
            type Output = $a::Output;
            type Error = $a::Error;

            fn choice(&mut self, input: Input) -> Result<(Self::Output, Input), Self::Error> {
                alt_parser_inner!(0, (), self, input, $a, $($rest),+)
            }
        }
    };
}

macro_rules! alt_parser_inner {
    ($field:tt, (), $self:expr, $input:expr, $a:ident, $($rest:ident),*) => {
        match $self.$field.parse($input.clone()) {
            Ok(t) => Ok(t),
            Err(e1) => succ_alt_parser_inner!($field, (e1), $self, $input, $($rest),*)
        }
    };
    ($field:tt, ($err:expr), $self:expr, $input:expr, $a:ident, $($rest:ident),*) => {
        match $self.$field.parse($input.clone()) {
            Ok(t) => Ok(t),
            Err(e2) => {
                let e1 = $err.merge(e2);
                succ_alt_parser_inner!($field, (e1), $self, $input, $($rest),*)
            }
        }
    };
    ($field:tt, ($err:expr), $self:expr, $input:expr, $a:ident) => { 
        match $self.$field.parse($input.clone()) {
            Ok(t) => Ok(t),
            Err(e2) => Err($err.merge(e2))
        }
    }
}

macro_rules! succ_alt_parser_inner {
    (0, $($p:tt),*) => (alt_parser_inner!(1, $($p),*));
    (1, $($p:tt),*) => (alt_parser_inner!(2, $($p),*));
    (2, $($p:tt),*) => (alt_parser_inner!(3, $($p),*));
    (3, $($p:tt),*) => (alt_parser_inner!(4, $($p),*));
    (4, $($p:tt),*) => (alt_parser_inner!(5, $($p),*));
    (5, $($p:tt),*) => (alt_parser_inner!(6, $($p),*));
    (6, $($p:tt),*) => (alt_parser_inner!(7, $($p),*));
    (7, $($p:tt),*) => (alt_parser_inner!(8, $($p),*));
    (8, $($p:tt),*) => (alt_parser_inner!(9, $($p),*));
    (9, $($p:tt),*) => (alt_parser_inner!(10, $($p),*));
    (10, $($p:tt),*) => (alt_parser_inner!(11, $($p),*));
    (11, $($p:tt),*) => (alt_parser_inner!(12, $($p),*));
    (12, $($p:tt),*) => (alt_parser_inner!(13, $($p),*));
    (13, $($p:tt),*) => (alt_parser_inner!(14, $($p),*));
    (14, $($p:tt),*) => (alt_parser_inner!(15, $($p),*));
    (15, $($p:tt),*) => (alt_parser_inner!(16, $($p),*));
    (16, $($p:tt),*) => (alt_parser_inner!(17, $($p),*));
    (17, $($p:tt),*) => (alt_parser_inner!(18, $($p),*));
    (18, $($p:tt),*) => (alt_parser_inner!(19, $($p),*));
    (19, $($p:tt),*) => (alt_parser_inner!(20, $($p),*));
    (20, $($p:tt),*) => (alt_parser_inner!(21, $($p),*));
}

macro_rules! alt_parser {
    ($a:ident, $b:ident, $($rest:ident),*) => {
        alt_parser_impl!($a, $b, $($rest),*);
        alt_parser!($b, $($rest),*);
    };
    ($a:ident, $b:ident) => {
        alt_parser_impl!($a, $b);
    }
}

alt_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);