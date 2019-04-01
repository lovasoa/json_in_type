extern crate json_in_type;

use json_in_type::*;

fn main() {
    let is_awesome = true;
    let heterogeneous_list = json_list![42u8, true];
    let dynamic_key = "hello";

    let json_val = json_object! {
        is_awesome, heterogeneous_list,
        [dynamic_key]: "world"
    };
    println!("json: {}", json_val.to_json_string());
    println!("size: {}", ::std::mem::size_of_val(&json_val));
}
