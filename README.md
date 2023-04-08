# rtor
Parser Combinator Library

# Example

```rust
use rtor::{
    combine::{
        sep_by,
        between
    },
    primitive::{
        digit
    }
};

fn main() {
    
    let mut parser = between('[', sep_by(digit, ','), ']');
    
    assert_eq!(
        parser.parse("[1,2,3,4,5,6]"), 
        Ok((vec!['1','2','3','4','5','6'], ""))
    );
}
```
