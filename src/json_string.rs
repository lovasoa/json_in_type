use std::io;
use super::json_value::JSONValue;

struct EscapeChar(u8);

impl EscapeChar {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        let c = self.0;
        match c {
            b'"' => w.write_all(b"\\\""),
            b'\\' => w.write_all(b"\\\\"),
            b'\n' => w.write_all(b"\\n"),
            b'\r' => w.write_all(b"\\r"),
            b'\t' => w.write_all(b"\\t"),
            _ => write!(w, "\\u{:04x}", u32::from(c)),
        }
    }
}

#[inline(always)]
fn json_escaped_char(c: u8) -> Option<EscapeChar> {
    if c > 0x1F && c != b'"' && c != b'\\' {
        None
    } else {
        Some(EscapeChar(c))
    }
}

pub trait JSONString: JSONValue {}

impl JSONValue for char {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"\"")?;
        if let Some(escaped) = json_escaped_char(*self as u8) {
            escaped.write(w)?;
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
        for (i, &c) in bytes.iter().enumerate() {
            if let Some(escaped) = json_escaped_char(c) {
                w.write_all(&bytes[char_index_to_write..i])?;
                escaped.write(w)?;
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
