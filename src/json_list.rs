use std::io;
use super::json_value::JSONValue;

#[inline(always)]
fn write_json_iterator<J, I, W>(iter: &mut I, w: &mut W) -> io::Result<()>
    where I: Iterator<Item=J>,
          J: JSONValue,
          W: io::Write {
    w.write_all(b"[")?;
    if let Some(first) = iter.next() {
        first.write_json(w)?;
        for x in iter {
            w.write_all(b",")?;
            x.write_json(w)?;
        }
    }
    w.write_all(b"]")
}

impl<T: JSONValue> JSONValue for Vec<T> {
    #[inline(always)]
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write_json_iterator(&mut self.iter(), w)
    }
}

/// A struct used to wrap another type and make it serializable as a json list.
///
/// The other type has to be able to yield values by implementing IntoIterator.
///
/// # Examples
/// ### Serialize a slice as JSON
/// ```
/// use json_in_type::json_list::ToJSONList;
/// use json_in_type::JSONValue;
///
/// let slice : [u8; 3] = [42,42,42];
///
/// assert_eq!("[42,42,42]", ToJSONList(slice).to_json_string());
/// ```
pub struct ToJSONList<T: JSONValue, U>(pub U)
    where for<'a> &'a U: IntoIterator<Item=&'a T>;

impl<T: JSONValue, U> JSONValue for ToJSONList<T, U>
    where for<'a> &'a U: IntoIterator<Item=&'a T> {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write_json_iterator(&mut (&self.0).into_iter(), w)
    }
}

pub trait JSONList: JSONValue {
    fn write_json_ending<W: io::Write>(&self, w: &mut W) -> io::Result<()>;
}

pub struct JSONListElem<T: JSONValue, U: JSONList> {
    elem: T,
    next: U,
}

impl<T: JSONValue, U: JSONList> JSONListElem<T, U> {
    pub fn new(elem: T, next: U) -> JSONListElem<T, U> {
        JSONListElem { elem, next }
    }
}

impl<T: JSONValue, U: JSONList> JSONList for JSONListElem<T, U> {
    #[inline(always)]
    fn write_json_ending<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b",")?;
        self.elem.write_json(w)?;
        self.next.write_json_ending(w)
    }
}

impl<T: JSONValue, U: JSONList> JSONValue for JSONListElem<T, U> {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"[")?;
        self.elem.write_json(w)?;
        self.next.write_json_ending(w)
    }
}

pub struct JSONListEnd;

impl JSONList for JSONListEnd {
    fn write_json_ending<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"]")
    }
}

impl JSONValue for JSONListEnd {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"[]")
    }
}

/// Creates a static json list that can be serialized very fast.
/// Returns an object implementing JSONValue
///
/// # Examples
/// Create a list containing objects of different types.
/// ```
/// use json_in_type::*;
///
/// let my_list = json_list![1,true,"hello"];
///
/// assert_eq!("[1,true,\"hello\"]", my_list.to_json_string());
/// ```
#[macro_export]
macro_rules! json_list {
    ($elem:expr $(, $rest:expr )* ) => {
        $crate::json_list::JSONListElem::new(
            $elem,
            json_list!($($rest),*)
        )
    };
    () => { $crate::json_list::JSONListEnd{} };
}
