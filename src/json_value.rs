//! The main module of the json_in_type crate.
//!
//! It contains its core abstraction, [JSONValue](trait.JSONValue.html), that represents
//! a value that can be serialized to json.

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use super::utils;

/// A trait implemented by types that can be serialized to JSON
///
/// This trait can be derived for custom structs using
/// [json_in_type_derive](https://docs.rs/json_in_type_derive/)
pub trait JSONValue {
    /// Write the object as json to the given writer
    ///
    /// # Examples
    ///
    /// Write a JSON object to a file
    ///
    /// ```
    /// # let mut my_file = ::std::io::sink();
    /// use json_in_type::JSONValue;
    ///
    /// vec![1, 2, 3].write_json(&mut my_file);
    /// ```
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()>;

    /// Returns the object formatted as a json string
    ///
    /// # Panics
    /// If you implement JSONValue on your own types and emit invalid UTF-8
    /// in write_json. If you use the implementations of JSONValue provided
    /// in this library, this function will never panic.
    fn to_json_string(&self) -> String {
        // This is safe because the bytes we emit are all valid UTF-8
        String::from_utf8(self.to_json_buffer()).unwrap()
    }

    /// Returns a buffer containing the bytes of a json representation of the object
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