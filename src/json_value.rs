use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;

pub trait JSONValue {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()>;
    fn to_json_string(&self) -> String {
        format!("{}", JSON(self))
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

struct FormatterWriter<'a, 'b>(&'a mut Formatter<'b>);

impl<'a, 'b> io::Write for FormatterWriter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match core::str::from_utf8(buf) {
            Ok(buf_str) => {
                self.0.write_str(buf_str)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
                    .map(|()| buf.len())
            }
            Err(err) => {
                Err(io::Error::new(io::ErrorKind::InvalidData, err))
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

impl<T: JSONValue> Display for JSON<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut writer = FormatterWriter(f);
        self.0.write_json(&mut writer)
            .map(|_size| ())
            .map_err(|_err| fmt::Error {})
    }
}