extern crate json_in_type;

use json_in_type::*;

fn main() {
    let dynamic_key = "hello";
    let json_val = JSON(json_object!{
        void: (),
        list: json_list![1,2,3],
        [dynamic_key]: "world"
    });
    println!("{}", json_val);
}
