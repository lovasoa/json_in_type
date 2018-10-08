extern crate json_in_type;
#[macro_use]
extern crate json_in_type_derive;

use json_in_type::JSONValue;

#[allow(dead_code)]
#[derive(JSONValue)]
struct MyObject {
    void: (),
    list: Vec<u8>,
    hello: String,
}

#[test]
fn test_simple_json() {
    let obj = MyObject {
        void: (),
        list: vec![1, 2, 3],
        hello: String::from("world"),
    };
    assert_eq!(
        r#"{"void":null,"list":[1,2,3],"hello":"world"}"#,
        obj.to_json_string()
    );
}

#[allow(dead_code)]
#[derive(JSONValue)]
struct WrapperStruct(u8, u8);

#[test]
fn test_wrapperstruct() {
    let obj = WrapperStruct(9, 4);
    assert_eq!("[9,4]", obj.to_json_string());
}

#[allow(dead_code)]
#[derive(JSONValue)]
enum Val { A, B, C }

#[test]
fn test_enum() {
    let obj = Val::C;
    assert_eq!(r#"{"C":true}"#, obj.to_json_string());
}
