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
    /* The type of json_val is:
    
    json_in_type::JSON<
        json_in_type::JSONObjectEntry<
            &str, (),
        json_in_type::JSONObjectEntry<
            &str,
                json_in_type::JSONListElem<{integer},
                json_in_type::JSONListElem<{integer},
                json_in_type::JSONListElem<{integer},
                json_in_type::JSONListEnd>>>,
        json_in_type::JSONObjectEntry<
            &str, &str,
        json_in_type::JSONObjectEnd>>>>
    */
    println!("{}", json_val);
}
```