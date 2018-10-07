extern crate json_in_type;

use json_in_type::*;

fn main() {
    let v = json_list![
        true,
        '\n',
        json_object!{
            x: 1,
            hello: String::from("world")
        },
        vec![Some("val"), None],
        (),
        Box::new("I ❤️ \"pépé\" \n backslash: \\!!!"),
        Box::new(
            JSONListElem::new(
                1, JSONListElem::new(
                    "lala",
                    JSONListEnd,
                ),
            )
        ),
        Box::new(JSONListEnd),
        Box::new(vec![
            ("a", 1),
            ("b", 1)
        ])
    ];
    println!("{}", JSON(v));
}
