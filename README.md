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
    
    let json_val = JSON(json_object!{
        void, list,
        [dynamic_key]: "world"
    });
    /* The type of json_val is:
    
    json_in_type::JSON<
        main::InlinedJSONObjectEntry<
            (),
        main::InlinedJSONObjectEntry<
            json_in_type::JSONListElem<{integer},
                json_in_type::JSONListElem<{integer},
                json_in_type::JSONListElem<{integer},
                json_in_type::JSONListEnd>>>,
        json_in_type::JSONObjectEntry<
            &str, &str,
        json_in_type::JSONObjectEnd>>>>
    */

    assert_eq!(
        r#"{"void":null,"list":[1,2,3],"hello":"world"}"#,
        format!("{}", json_val)
    );
}
```

## Performance

This library is generally faster than SERDE.
Here are detailed comparison results on different json serialization tasks realized on an  AMD Ryzen 5 1600X.
[See detailed benchmark results.](https://lovasoa.github.io/json_in_type/docs/criterion/report/)

### Encoding 8 nested json objects using a rust macro

#### Encoded object
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
