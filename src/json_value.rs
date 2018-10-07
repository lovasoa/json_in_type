use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use super::fmt_io_compat;

pub trait JSONValue {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()>;
    fn to_json_string(&self) -> String {
        format!("{}", JSON(self))
    }
    fn to_json_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(512);
        self.write_json(&mut buffer).unwrap();
        buffer
    }
}


impl<'a, S: JSONValue + ?Sized> JSONValue for &'a S {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        (**self).write_json(w)
    }
}

impl<'a, S: JSONValue + ?Sized> JSONValue for Box<S> {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        (**self).write_json(w)
    }
}

pub struct JSON<T: JSONValue>(pub T);

impl<T: JSONValue> Display for JSON<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut writer = fmt_io_compat::FormatterWriter(f);
        self.0.write_json(&mut writer)
            .map(|_size| ())
            .map_err(|_err| fmt::Error {})
    }
}

impl<T: JSONValue> From<JSON<T>> for Vec<u8> {
    fn from(json: JSON<T>) -> Self {
        json.0.to_json_buffer()
    }
}