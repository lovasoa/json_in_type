# json_in_type

Fast json encoder in rust, that does more at compile time, and less at run time.
One notable feature is the ability to encode the structure of JSON objects in their type.

This allows for a very compact representation of objects in memory, and up to an order of magnitude better performance
than the traditional approach (used by serde's `json!` marco, for instance) where JSON objects are stored in maps.

The goal of this library is to be as close as possible to the performance
and memory footprint you would get by writing the json by hand in your source code
and using string formatting to insert you dynamic values.

```rust
fn write_obj_bad(value: i32) -> String { 
    format!("{\"value\":{}}", value)
}

// Safer, more readable, but equivalent and not less efficient :
fn write_obj_good(value: i32) -> String {
    (json_object!{value}).to_json_string()
}
```

## Exemple use

```rust
extern crate json_in_type;

use json_in_type::*;

fn main() {
    let void = ();
    let list = json_list![42u8, true];
    let dynamic_key = "hello";
    
    let json_val = json_object!{
        void, list,
        [dynamic_key]: "world"
    };
    /* The type of json_val is:
    
        InlinedJSONObjectEntry<
            (),
        InlinedJSONObjectEntry<
            JSONListElem<u8,
            JSONListElem<JSONtrue,
            JSONListEnd>>>,
        JSONObjectEntry<
            &str, &str,
        JSONObjectEnd>>>
    */

    assert_eq!(
        r#"{"void":null,"list":[42,true],"hello":"world"}"#,
        json_val.to_json_string()
    );
}
```

## Memory use
The generated types have a very small memory footprint at runtime.
You don't pay for the json structure, only for what you put in it !

In the next example, we store the following json structure on only two bytes:
```json
{
  "result_count" : 1,
  "errors" : null,
  "results" : [
    {"answer":42, "ok":true}
   ]
}
```

```rust
fn test_memory_size() {
    let (result_count, answer) = (1u8, 42u8);
    let my_val = json_object! {
        result_count,
        errors: null,
        results: json_list![
            json_object!{answer, ok: true}
        ]
    };
    // my_val weighs only two bytes, because we stored only 2 u8 in it
    assert_eq!(2, ::std::mem::size_of_val(&my_val));
}
```
## Performance

This library is generally faster than SERDE.
Here are detailed comparison results on different json serialization tasks realized on an  AMD Ryzen 5 1600X.
[See detailed benchmark results.](https://lovasoa.github.io/json_in_type/docs/criterion/report/)

### Encoding 8 nested json objects using a rust macro

We use serde's
[`json!`](https://docs.serde.rs/serde_json/macro.json.html)
and json_in_type's
[`json_object!`](https://docs.rs/json_in_type/0.1.2/json_in_type/macro.json_object.html)
macro to encode a nested json object.

#### Encoded object
We encode a JSON structure composed of 8 nested objects, each of 
which contains a single key, that is known at compile time.
The last nested object contains an integer *n* that is not known at compile time.
```json
{"nested":{"nested":{"nested":{"nested":{"nested":{"nested":{"nested":{"nested":{"value":n}}}}}}}}}
```

#### Benchmark result
![nested json objects comparison](https://lovasoa.github.io/json_in_type/docs/criterion/encode%20nested%20objects/report/violin.svg)

### Encoding a very simple json object using a rust macro

#### Encoded object
```json
{
        "void": null,
        "list": [1, 2, 3, 3],
        "hello": "world"
}
```

#### Benchmark result
![simple object](https://lovasoa.github.io/json_in_type/docs/criterion/encode%20simple%20object%20with%20macro/report/violin.svg)

### Encoding a very simple json object using `#[derive(...)]`

#### Encoded object
```json
{
        "void": null,
        "list": [1, 2, 3, 3],
        "hello": "world"
}
```

created from the following rust struct

```rust
#[derive(Serialize, JSONValue)]
struct MyObject {
    void: (),
    list: Vec<f64>,
    hello: String,
}
```

#### Benchmark result
![simple object](https://lovasoa.github.io/json_in_type/docs/criterion/encode%20simple%20object%20with%20derive/report/violin.svg)

## External links

 * docs.rs hosts this crate's [api documentation](https://docs.rs/json_in_type).
    * documentation for [the `json_object!` macro](https://docs.rs/json_in_type/0.1.2/json_in_type/macro.json_object.html)
    * documentation for [the `JSONValue` trait](https://docs.rs/json_in_type/0.1.2/json_in_type/json_value/trait.JSONValue.html)
 * You can automatically derive the `JSONValue` trait for your type using the [json_in_type_derive crate](https://docs.rs/json_in_type_derive)
 * You can see [json_in_type on crates.io](https://crates.io/crates/json_in_type).
