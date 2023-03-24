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
    let mut state = State::new("[1,2,3,4,5,6]");
    
    let mut parser = between(
      char('['), 
      sepby(digit, char(',')), 
      char(']')
    );
    
    assert_eq!(parser.parse(&mut state), Ok(vec![1,2,3,4,5,6]));
}
```
