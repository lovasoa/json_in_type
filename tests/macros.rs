extern crate json_in_type;

use json_in_type::*;

#[test]
fn test_nested_macro() {
    assert_eq!("[{}]", json_list![json_object!{}].to_json_string());
    assert_eq!("[{}]", json_list![json_object!{},].to_json_string());
    assert_eq!(r#"[{"ok":true}]"#, json_list![json_object!{ok:true}].to_json_string());
    assert_eq!(r#"[{"a":0,"b":0}]"#, json_list![json_object!{a:0,b:0}].to_json_string());
    assert_eq!(r#"[{"a":0,"b":0},{"a":0,"b":0}]"#,
               json_list![
                    json_object!{a:0,b:0},
                    json_object!{a:0,b:0},
                   ].to_json_string());
}

#[test]
fn test_mem_size() {
    let value_count: u8 = 3;
    let (val1, val2, val3): (i8, i8, i8) = (42, -42, 66);

    let small_object = json_object! {
            value_count,
            values: json_list![
                json_object! {ok:true, value: val1},
                json_object! {ok:true, value: val2},
                json_object! {ok:true, value: val3},
            ]
        };
    assert_eq!(4, ::std::mem::size_of_val(&small_object));
}

#[test]
fn mem_size_example() {
    let (result_count, answer) = (1u8, 42u8);
    let my_val = json_object! {
        result_count,
        errors: null,
        results: json_list![
            json_object!{answer, ok: true}
        ]
    };
    // my_val weighs only two bytes, because we stored only 2 u8 in it
    assert_eq!(2, ::std::mem::size_of_val(&my_val));
    assert_eq!(r#"{"result_count":1,"errors":null,"results":[{"answer":42,"ok":true}]}"#,
               my_val.to_json_string());
}

#[test]
fn handwritten_equivalent() {
    fn write_obj_bad(value: i32) -> String { format!("{{\"value\":{}}}", value) }
    fn write_obj_good(value: i32) -> String { (json_object! {value}).to_json_string() }
    assert_eq!(write_obj_bad(42), write_obj_good(42));
}