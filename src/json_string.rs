use std::io;
use super::json_value::JSONValue;

#[inline(always)]
fn json_escaped_char(c: u8) -> Option<Vec<u8>> {
    if c > 0x20 && c != b'"' && c != b'\\' {
        None
    } else {
        match c {
            b'"' => Some(b"\\\"".to_vec()),
            b'\\' => Some(b"\\\\".to_vec()),
            b'\n' => Some(b"\\n".to_vec()),
            b'\r' => Some(b"\\r".to_vec()),
            b'\t' => Some(b"\\t".to_vec()),
            x if x < b' ' => Some(format!("\\u{:04x}", x as u32).as_bytes().to_vec()),
            _ => None
        }
    }
}

pub trait JSONString: JSONValue {}

impl JSONValue for char {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"\"")?;
        if let Some(s) = json_escaped_char(*self as u8) {
            w.write_all(&s)?;
        } else {
            write!(w, "{}", self)?;
        }
        w.write_all(b"\"")
    }
}

impl JSONString for char {}

impl<'a> JSONValue for &'a str {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"\"")?;
        let bytes = self.as_bytes();
        let mut char_index_to_write = 0;
        for i in 0..bytes.len() {
            if let Some(s) = json_escaped_char(bytes[i]) {
                w.write_all(&bytes[char_index_to_write..i])?;
                w.write_all(&s)?;
                char_index_to_write = i + 1;
            }
        }
        w.write_all(&bytes[char_index_to_write..])?;
        w.write_all(b"\"")
    }
}

impl<'a> JSONString for &'a str {}


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
        assert_eq!("\"\\\\\"", '\\'.to_json_string());
        assert_eq!("\"\\u0000\"", '\0'.to_json_string());
        assert_eq!("\"❤\"", '❤'.to_json_string());
    }

    #[test]
    fn test_simple_string() {
        assert_eq!(r#""""#, "".to_json_string());
        assert_eq!(r#""Hello, world!""#, "Hello, world!".to_json_string());
        assert_eq!(r#""\t\t\n""#, "\t\t\n".to_json_string());
    }

    #[test]
    fn test_complex_string() {
        assert_eq!(
            r#""I ❤️ \"pépé\" \n backslash: \\!!!\n""#,
            "I ❤️ \"pépé\" \n backslash: \\!!!\n".to_json_string()
        );
    }
}
