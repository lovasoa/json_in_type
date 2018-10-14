use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use super::utils;

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

/// Encapsulates a [JSONValue](trait.JSONValue.html) and implements useful traits.
///
/// # Examples
///
/// ```
/// use json_in_type::JSON;
///
/// let x_json = JSON(vec![(), (), ()]);
/// assert_eq!("[null,null,null]", x_json.to_string()); // JSON format
///
/// println!("{}", x_json); // just works. Displays [null,null,null]
///
/// let my_buffer : Vec<u8> = x_json.into();
/// assert_eq!(b"[null,null,null]".to_vec(), my_buffer);
/// ```
pub struct JSON<T: JSONValue>(pub T);

impl<T: JSONValue> Display for JSON<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut writer = utils::FormatterWriter(f);
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