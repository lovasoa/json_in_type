extern crate json_in_type;

use json_in_type::*;

#[test]
fn test() {
    let json_val = JSON(json_object!{
        void: (),
        list: json_list![1,2,3],
        hello: "world"
    });
    assert_eq!(
        r#"{"void":null,"list":[1,2,3],"hello":"world"}"#,
        format!("{}", json_val)
    );
}
