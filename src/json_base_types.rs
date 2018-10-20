//! Serialization to from numbers, booleans, and nil

use std::io;
use super::json_value::JSONValue;
extern crate ryu_ecmascript;

macro_rules! display_is_json {
    ( $( $json_type:ty ),* ) => {
        $(
            impl JSONValue for $json_type {
                #[inline(always)]
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
    bool
);

macro_rules! impl_json_for_float {
    ( $( $json_type:ty ),* ) => {
        $(
            impl JSONValue for $json_type {
                #[inline(always)]
                fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
                    if self.is_finite() {
                        w.write_all(ryu_ecmascript::Buffer::new().format(*self).as_bytes())
                    } else {
                        ().write_json(w) // null
                    }
                }
            }
        )*
    };
}

impl_json_for_float!(f32, f64);

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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_int() {
        assert_eq!("-1234567890", (-1234567890 as i32).to_json_string());
        assert_eq!("1234567890123456789", 1234567890123456789u64.to_json_string());
    }

    #[test]
    fn test_float() {
        use std::f64;
        assert_eq!("-1234567890", (-1234567890 as f64).to_json_string());
        assert_eq!("null", (f64::NAN).to_json_string());
        assert_eq!("null", (f64::NEG_INFINITY).to_json_string());
    }
}
