use std::io;
use super::json_value::JSONValue;

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
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write_json_iterator(&mut self.iter(), w)
    }
}

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

#[macro_export]
macro_rules! json_list {
    ($elem:expr $(, $rest:expr )* ) =>
        { JSONListElem::new($elem, json_list!($($rest),*)) };
    () => { JSONListEnd{} };
}
