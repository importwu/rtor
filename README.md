# rtor
Parser Combinator Library

# Example

```rust
use rtor::{
    State,
    combine::{
        sepby,
        between
    },
    primitive::{
        digit,
        char
    }
};

fn main() {
    
    let mut parser = between(
      char('['), 
      sepby(digit, char(',')), 
      char(']')
    );
    
    assert_eq!(parser.parse("[1,2,3,4,5,6]"), Ok((vec![1,2,3,4,5,6], "")));
}
```
