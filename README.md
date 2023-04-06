# rtor
Parser Combinator Library

# Example

```rust
use rtor::{
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
    
    let mut parser = between('[', sepby(digit, ','), ']');
    
    assert_eq!(
        parser.parse("[1,2,3,4,5,6]"), 
        Ok((vec!['1','2','3','4','5','6'], ""))
    );
}
```
