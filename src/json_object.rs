use std::io;
use super::json_string::JSONString;
use super::json_value::JSONValue;

impl<'a, T: JSONValue> JSONValue for Vec<(&'a str, T)> {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"{")?;
        let len = self.len();
        if len > 0 {
            for (key, value) in self[0..len - 1].iter() {
                key.write_json(w)?;
                w.write_all(b":")?;
                value.write_json(w)?;
                w.write_all(b",")?;
            }
            let (key, value) = &self[len - 1];
            key.write_json(w)?;
            w.write_all(b":")?;
            value.write_json(w)?;
        }
        w.write_all(b"}")
    }
}


pub trait JSONObject: JSONValue {
    fn write_json_ending<W: io::Write>(&self, f: &mut W, prefix: &[u8]) -> io::Result<()>;
}


pub struct JSONObjectEntry<K: JSONString, V: JSONValue, U: JSONObject> {
    pub key: K,
    pub value: V,
    pub next: U,
}

impl<K: JSONString, V: JSONValue, U: JSONObject> JSONObject for JSONObjectEntry<K, V, U> {
    #[inline(always)]
    fn write_json_ending<W: io::Write>(&self, w: &mut W, prefix: &[u8]) -> io::Result<()> {
        w.write_all(prefix)?;
        self.key.write_json(w)?;
        w.write_all(b":")?;
        self.value.write_json(w)?;
        self.next.write_json_ending(w, b",")
    }
}

impl<K: JSONString, V: JSONValue, U: JSONObject> JSONValue for JSONObjectEntry<K, V, U> {
    #[inline(always)]
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        self.write_json_ending(w, b"{")
    }
}

pub struct JSONObjectEnd;

impl JSONObject for JSONObjectEnd {
    #[inline(always)]
    fn write_json_ending<W: io::Write>(&self, w: &mut W, _: &[u8]) -> io::Result<()> {
        w.write_all(b"}")
    }
}

impl JSONValue for JSONObjectEnd {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"{}")
    }
}

#[macro_export]
macro_rules! json_object {
    () => { JSONObjectEnd{} };
    // Literal key
    ($key:ident : $value:expr, $($rest:tt)*) => {
        JSONObjectEntry{
            key: stringify!($key),
            value: $value,
            next: json_object!($($rest)*)
         }
    };
    // Simply adding a trailing colon
    ($key:ident : $value:expr) => { json_object!($key:$value,) };
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!("{}", JSONObjectEnd.to_json_string());
        assert_eq!("{}", json_object!().to_json_string());
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(r#"{"x":{}}"#, json_object!(x : json_object!()).to_json_string());
        // With a trailing comma:
        assert_eq!(r#"{"x":{}}"#, json_object!(x : json_object!(),).to_json_string());
    }

    #[test]
    fn test_two_pairs() {
        assert_eq!(r#"{"x":{},"y":{}}"#, json_object! {
            x : json_object!(),
            y : json_object!()
        }.to_json_string());
    }

    #[test]
    fn test_nested() {
        assert_eq!(r#"{"x":{"y":{}}}"#, json_object! {
            x : json_object! {
                y : json_object!()
            }
        }.to_json_string());
    }
}
