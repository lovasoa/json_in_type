extern crate json_in_type;

use json_in_type::*;

fn main() {
    let is_awesome = true;
    let heterogeneous_list = json_list![1,2,3,(),5];
    let dynamic_key = "hello";

    let json_val = json_object!{
        is_awesome, heterogeneous_list,
        [dynamic_key]: "world"
    };
    println!("{}", JSON(json_val));
}
