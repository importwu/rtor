use super::Parser;

macro_rules! tuple_parser {
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
    ($o: ident, $field: tt, $self: expr, $input: expr, (), $a: ident, $($rest: ident),*) => {
        let ($o, i) = $self.$field.parse($input)?;
        succ!($field, $self, i, ($o), $($rest),*)
    };
    ($o: ident, $field: tt, $self: expr, $input: expr, ($($os:tt)*), $a: ident, $($rest: ident),*) => {
        let ($o, i) = $self.$field.parse($input)?;
        succ!($field, $self, i, ($($os)*, $o), $($rest),*)
    };
    ($o: ident, $field: tt, $self: expr, $input: expr, ($($os:tt)*), $a: ident) => {
        let ($o, i) = $self.$field.parse($input)?;
        return Ok((($($os)*, $o), i));
    };
}

macro_rules! succ {
    (0, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o2, 1, $self, $input, ($($os)*), $($name),*));
    (1, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o3, 2, $self, $input, ($($os)*), $($name),*));
    (2, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o4, 3, $self, $input, ($($os)*), $($name),*));
    (3, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o5, 4, $self, $input, ($($os)*), $($name),*));
    (4, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o6, 5, $self, $input, ($($os)*), $($name),*));
    (5, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o7, 6, $self, $input, ($($os)*), $($name),*));
    (6, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o8, 7, $self, $input, ($($os)*), $($name),*));
    (7, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o9, 8, $self, $input, ($($os)*), $($name),*));
    (8, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o10, 9, $self, $input, ($($os)*), $($name),*));
    (9, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o11, 10, $self, $input, ($($os)*), $($name),*));
    (10, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o12, 11, $self, $input, ($($os)*), $($name),*));
    (11, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o13, 12, $self, $input, ($($os)*), $($name),*));
    (12, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o14, 13, $self, $input, ($($os)*), $($name),*));
    (13, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o15, 14, $self, $input, ($($os)*), $($name),*));
    (14, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o16, 15, $self, $input, ($($os)*), $($name),*));
    (15, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o17, 16, $self, $input, ($($os)*), $($name),*));
    (16, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o18, 17, $self, $input, ($($os)*), $($name),*));
    (17, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o19, 18, $self, $input, ($($os)*), $($name),*));
    (18, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o20, 19, $self, $input, ($($os)*), $($name),*));
    (19, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o21, 20, $self, $input, ($($os)*), $($name),*));
    (20, $self: expr, $input: expr, ($($os:tt)*), $($name: ident),*) => (tuple_parser_inner!(o22, 21, $self, $input, ($($os)*), $($name),*));
}

tuple_parser!(A, B);
tuple_parser!(A, B, C);
tuple_parser!(A, B, C, D);
tuple_parser!(A, B, C, D, E);
tuple_parser!(A, B, C, D, E, F);
tuple_parser!(A, B, C, D, E, F, G);
tuple_parser!(A, B, C, D, E, F, G, H);
tuple_parser!(A, B, C, D, E, F, G, H, I);
tuple_parser!(A, B, C, D, E, F, G, H, I, J);
tuple_parser!(A, B, C, D, E, F, G, H, I, J, K);
tuple_parser!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M);
tuple_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
tuple_parser!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);