use std::io;
use super::json_value::JSONValue;

macro_rules! display_is_json {
    ( $( $json_type:ty ),* ) => {
        $(
            impl JSONValue for $json_type {
                fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
                    write!(w, "{}", self)
                }
            }
        )*
    };
}

display_is_json!(
    i8,i16,i32,i64,i128,isize,
    u8,u16,u32,u64,u128,usize,
    f32,f64,
    bool
);

impl JSONValue for () {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"null")
    }
}

impl<T:JSONValue> JSONValue for Option<T> {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        if let Some(val) = self {
            val.write_json(w)
        } else {
            ().write_json(w)
        }
    }
}