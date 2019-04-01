#[macro_use]
extern crate json_in_type_derive;
#[macro_use]
extern crate serde_derive;

use criterion::{AxisScale, Criterion, Fun, ParameterizedBenchmark, PlotConfiguration, Throughput};
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
    let obj = serde_json::json!({
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
    let obj = serde_json::json!({
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
            Fun::new("json_in_type", |b, i| {
                b.iter(|| simple_json_in_type_derive(*i))
            }),
            Fun::new("serde_json", |b, i| b.iter(|| simple_serde_derive(*i))),
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

    struct BenchStr(String);
    impl BenchStr {
        fn new(magnitude: usize) -> BenchStr {
            BenchStr(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                 In vel erat rutrum, tincidunt lorem nullam\n"
                    .repeat(1 << magnitude),
            )
        }
        fn serde(&self) -> Vec<u8> {
            serde_json::to_vec(&self.0).unwrap()
        }
        fn json_in_type(&self) -> Vec<u8> {
            self.0.to_json_buffer()
        }
        fn bytes_len(&self) -> usize {
            self.0.as_bytes().len()
        }
    }
    impl std::fmt::Debug for BenchStr {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}-bytes string", self.bytes_len())
        }
    }

    c.bench(
        "string encoding throughput",
        ParameterizedBenchmark::new(
            "json_in_type",
            |b, i| b.iter(|| i.json_in_type()),
            (0..12).step_by(4).map(BenchStr::new),
        )
            .with_function("serde", |b, i| b.iter(|| i.serde()))
            .throughput(|i| Throughput::Bytes(i.bytes_len() as u32))
            .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic)),
    );
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default().noise_threshold(0.05);
    targets = criterion_benchmark
}
criterion::criterion_main!(benches);
