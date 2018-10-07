use std::io;
use super::json_string::JSONString;
use super::json_value::JSONValue;

impl<T: JSONValue> JSONValue for Vec<(&str, T)> {
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
    fn write_json_ending<W: io::Write>(&self, f: &mut W) -> io::Result<()>;
}


pub struct JSONObjectEntry<K: JSONString, V: JSONValue, U: JSONObject> {
    key: K,
    value: V,
    next: U,
}

impl<K: JSONString, V: JSONValue, U: JSONObject> JSONObjectEntry<K, V, U> {
    pub fn new(key: K, value: V, next: U) -> JSONObjectEntry<K, V, U> {
        JSONObjectEntry { key, value, next }
    }
}

impl<K: JSONString, V: JSONValue, U: JSONObject> JSONObject for JSONObjectEntry<K, V, U> {
    #[inline(always)]
    fn write_json_ending<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b",")?;
        self.key.write_json(w)?;
        w.write_all(b":")?;
        self.value.write_json(w)?;
        self.next.write_json_ending(w)
    }
}

impl<K: JSONString, V: JSONValue, U: JSONObject> JSONValue for JSONObjectEntry<K, V, U> {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"{")?;
        self.key.write_json(w)?;
        w.write_all(b":")?;
        self.value.write_json(w)?;
        self.next.write_json_ending(w)
    }
}

pub struct JSONObjectEnd;

impl JSONObject for JSONObjectEnd {
    fn write_json_ending<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
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
    ($key:ident : $value:expr $(, $keys:ident : $values:expr )* ) => {
        JSONObjectEntry::new(
            stringify!($key),
            $value,
            json_object!($($keys : $values),*)
         )
    };
    () => { JSONObjectEnd{} };
}