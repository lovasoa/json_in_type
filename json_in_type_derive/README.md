# Procedural macros for json_in_type

This crate defines a derive mode macro allowing the
automatic derivation of the
[JSONValue](https://docs.rs/json_in_type/latest/json_in_type/trait.JSONValue.html)
trait for custom data structures.

# Example

```rust
extern crate json_in_type;
#[macro_use]
extern crate json_in_type_derive;
use json_in_type::JSONValue;

#[derive(JSONValue)]
struct MyObject { // This will be encoded as a JSON object
    void: (),
    list: Vec<u8>,
    hello: String,
}

#[derive(JSONValue)]
struct WrapperStruct(u8, u8); // This will be encoded as a JSON list
```