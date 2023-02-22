# rtor
Parser Combinator Library

# Example

```rust
use rtor::text::{StaticInput, digit, char};
use rtor::combinators::{sepby, between};

fn main() {
    let mut input = StaticInput::new("[1,2,3,4,5,6]");
    
    let mut parser = between(
      char('['), 
      sepby(digit, char(',')), 
      char(']')
    );
    
    assert_eq!(parser.parse(&mut input), Ok(vec![1,2,3,4,5,6]));
}
```
