use std::io;
use super::json_value::JSONValue;

#[inline(always)]
fn json_escaped_char(c: char) -> Option<[u8; 2]> {
    match c {
        '"' => Some([b'\\', b'"']),
        '\\' => Some([b'\\', b'\\']),
        '\n' => Some([b'\\', b'n']),
        '\r' => Some([b'\\', b'r']),
        '\t' => Some([b'\\', b't']),
        '\u{0008}' => Some([b'\\', b'b']),
        _ => None
    }
}

pub trait JSONString: JSONValue {}

impl JSONValue for char {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"\"")?;
        if let Some(s) = json_escaped_char(*self) {
            w.write_all(&s)?;
        } else {
            write!(w, "{}", self)?;
        }
        w.write_all(b"\"")
    }
}

impl JSONString for char {}

impl JSONValue for &str {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"\"")?;
        let mut char_index_to_write = 0;
        for (i, c) in self.char_indices() {
            if let Some(s) = json_escaped_char(c) {
                w.write_all(&self[char_index_to_write..i].as_bytes())?;
                w.write_all(&s)?;
                char_index_to_write = i + c.len_utf8();
            }
        }
        w.write_all(&self[char_index_to_write..self.len()].as_bytes())?;
        w.write_all(b"\"")
    }
}

impl JSONString for &str {}


impl JSONValue for String {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        (self as &str).write_json(w)
    }
}

impl JSONString for String {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_chars() {
        assert_eq!("\"x\"", 'x'.to_json_string());
        assert_eq!("\"\\n\"", '\n'.to_json_string());
    }

    #[test]
    fn test_string() {
        assert_eq!(
            r#""I ❤️ \"pépé\" \n backslash: \\!!!\n""#,
            "I ❤️ \"pépé\" \n backslash: \\!!!\n".to_json_string()
        );
    }
}