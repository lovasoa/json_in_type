# json_in_type

Fast json encoder in rust, that does more at compile time, and less at run time.
One notable feature is the ability to encode the structure of JSON objects in their type.

This allows for a very compact representation of objects in memory, and up to an order of magnitude better performance
than the traditional approach (used by serde's `json!` marco, for instance) where JSON objects are stored as HashMaps.  

## Exemple use

```rust
extern crate json_in_type;

use json_in_type::*;

fn main() {
    let void = ();
    let list = json_list![1,2,3];
    let dynamic_key = "hello";
    
    let json_val = json_object!{
        void, list,
        [dynamic_key]: "world"
    };
    /* The type of json_val is:
    
        main::InlinedJSONObjectEntry<
            (),
        main::InlinedJSONObjectEntry<
            json_in_type::JSONListElem<{integer},
            json_in_type::JSONListElem<{integer},
            json_in_type::JSONListElem<{integer},
            json_in_type::JSONListEnd>>>,
        json_in_type::JSONObjectEntry<
            &str, &str,
        json_in_type::JSONObjectEnd>>>
    */

    assert_eq!(
        r#"{"void":null,"list":[1,2,3],"hello":"world"}"#,
        json_val.to_json_string()
    );
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
 * You can see [json_in_type on crates.io](https://crates.io/crates/json_in_type).
