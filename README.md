# json-in-types
Fast json encoder in rust, that encodes the structure of JSON values in their types 

## Exemple use

```rust
extern crate json_in_type;

use json_in_type::*;

fn main() {
    let json_val = JSON(json_object!{
        void: (),
        list: json_list![1,2,3],
        hello: "world"
    });
    println!("{}", json_val);
}
```