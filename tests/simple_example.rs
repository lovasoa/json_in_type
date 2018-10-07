extern crate json_in_type;

use json_in_type::*;

#[test]
fn test_simple_json() {
    let obj = json_object!{
        void: (),
        list: json_list![1,2,3],
        hello: "world"
    };
    let mut buf: Vec<u8> = vec![];
    obj.write_json(&mut buf).unwrap();
    assert_eq!(
        br#"{"void":null,"list":[1,2,3],"hello":"world"}"#.to_vec(),
        buf
    );
}
