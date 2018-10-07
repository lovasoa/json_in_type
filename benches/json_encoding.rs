#[macro_use]
extern crate criterion;
extern crate json_in_type;
extern crate serde_json;

use json_in_type::*;

use criterion::{Criterion, Fun};

fn encode_json_in_type(n: f64) -> Vec<u8> {
    let obj = json_object!{
        void: (),
        list: json_list![1,2,3,n],
        hello: "world"
    };
    let mut buf: Vec<u8> = vec![];
    obj.write_json(&mut buf).unwrap();
    buf
}

fn encode_serde(n: f64) -> Vec<u8> {
    let obj = serde_json::json!({
        "void": null,
        "list": [1,2,3,n],
        "hello": "world"
    });
    serde_json::to_vec(&obj).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_functions(
        "encode simple object",
        vec![
            Fun::new("json_in_type", |b, i| b.iter(|| encode_json_in_type(*i))),
            Fun::new("serde_json", |b, i| b.iter(|| encode_serde(*i))),
        ],
        999_999_999.999f64,
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
