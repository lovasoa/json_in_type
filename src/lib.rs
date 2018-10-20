//! **json_in_type** is a library for *very fast* [JSON](http://json.org/) serialization.
//! It does only serialization, not parsing.
//!
//! # Principles
//! This library is fast at runtime because it tries to do more at compile time,
//! using rust's powerful type system, and macros.
//!
//! The base idea when starting to write this library was that a json object
//! with a number of properties that is known at compile-time can be efficiently
//! stored in an ad-hoc type, that will have a compact representation in memory,
//! and will be serialized much faster than a HashMap.
//!
//! # How to use
//! This crate has two main macros, [`json_object!`](macro.json_object.html),
//! and [`json_list!`](macro.json_list.html).
//! Use them to create [json values](trait.JSONValue.html), that you can then serialize.
//!
//!
pub use self::json_value::JSONValue;
pub use self::json_value::JSON;

pub mod json_base_types;
pub mod json_list;
pub mod json_object;
pub mod json_string;
pub mod json_value;
pub mod utils;