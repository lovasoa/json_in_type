#[macro_use]
extern crate criterion;
extern crate json_in_type;
#[macro_use]
extern crate json_in_type_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use criterion::{Criterion, Fun};
use json_in_type::*;

fn simple_json_in_type(n: f64) -> Vec<u8> {
    let obj = json_object! {
        void: (),
        list: json_list![1.,2.,3.,n],
        hello: "world"
    };
    obj.to_json_buffer()
}

fn simple_serde(n: f64) -> Vec<u8> {
    let obj = json!({
        "void": null,
        "list": [1., 2., 3., n],
        "hello": "world"
    });
    serde_json::to_vec(&obj).unwrap()
}

#[derive(Serialize, JSONValue)]
struct MyObject {
    void: (),
    list: Vec<f64>,
    hello: String,
}

fn simple_serde_derive(n: f64) -> Vec<u8> {
    let obj = MyObject {
        void: (),
        list: vec![1., 2., 3., n],
        hello: String::from("world"),
    };
    serde_json::to_vec(&obj).unwrap()
}

fn simple_json_in_type_derive(n: f64) -> Vec<u8> {
    let obj = MyObject {
        void: (),
        list: vec![1., 2., 3., n],
        hello: String::from("world"),
    };
    obj.to_json_buffer()
}

fn nested_json_in_type(n: u8) -> Vec<u8> {
    // 8 levels of nested objects
    let obj = json_object! {
        nested: json_object! {
            nested: json_object! {
                nested: json_object! {
                    nested: json_object! {
                        nested: json_object! {
                            nested: json_object! {
                                nested: json_object! {
                                    nested: json_object! {
                                        value: n
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    obj.to_json_buffer()
}

fn nested_serde(n: u8) -> Vec<u8> {
    let obj = json!({
        "nested": {
            "nested": {
                "nested": {
                    "nested": {
                        "nested": {
                            "nested": {
                                "nested": {
                                    "nested": {
                                        "value": n
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    serde_json::to_vec(&obj).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_functions(
        "encode simple object with macro",
        vec![
            Fun::new("json_in_type", |b, i| b.iter(|| simple_json_in_type(*i))),
            Fun::new("serde_json", |b, i| b.iter(|| simple_serde(*i))),
        ],
        999_999_999.999f64,
    );
    c.bench_functions(
        "encode simple object with derive",
        vec![
            Fun::new("json_in_type with derive", |b, i| b.iter(|| simple_json_in_type_derive(*i))),
            Fun::new("serde_json with derive", |b, i| b.iter(|| simple_serde_derive(*i))),
        ],
        999_999_999.999f64,
    );
    c.bench_functions(
        "encode nested objects",
        vec![
            Fun::new("json_in_type", |b, i| b.iter(|| nested_json_in_type(*i))),
            Fun::new("serde_json", |b, i| b.iter(|| nested_serde(*i))),
        ],
        0,
    );
}

criterion_group!{
    name = benches;
    config = Criterion::default().noise_threshold(0.05);
    targets = criterion_benchmark
}
criterion_main!(benches);
