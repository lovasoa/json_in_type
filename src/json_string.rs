//! Serialization to JSON strings like `"hello world \n"`

extern crate simd;

use self::simd::u8x16;
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
/// Implemented by types that can be serialized to a json string.
///
/// Implement this trait for your type if you want to be able to use it as a
/// key in a json object.
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
        write_json_simd(self, w)?;
        w.write_all(b"\"")
    }
}

fn write_json_simd<W: io::Write>(s: &str, w: &mut W) -> io::Result<()> {
    let bytes = s.as_bytes();
    let space = u8x16::splat(b' ');
    let quote = u8x16::splat(b'"');
    let slash = u8x16::splat(b'\\');
    let chunk_size = 16;
    let mut char_index_to_write = 0;
    let mut current_index = 0;
    for chunk_bytes in bytes.chunks(chunk_size) {
        let current_chunk_len = chunk_bytes.len();
        let needs_write = current_chunk_len != chunk_size || {
            let chunk = u8x16::load(chunk_bytes, 0);
            !(chunk.ge(space) & chunk.ne(quote) & chunk.ne(slash)).all()
        };
        if needs_write {
            w.write_all(&bytes[char_index_to_write..current_index])?;
            write_json_nosimd(chunk_bytes, w)?;
            char_index_to_write = current_index + current_chunk_len;
        }
        current_index += current_chunk_len;
    }
    w.write_all(&bytes[char_index_to_write..])
}

fn write_json_nosimd<W: io::Write>(bytes: &[u8], w: &mut W) -> io::Result<()> {
    let mut char_index_to_write = 0;
    for (i, &c) in bytes.iter().enumerate() {
        if let Some(escaped) = json_escaped_char(c) {
            w.write_all(&bytes[char_index_to_write..i])?;
            escaped.write(w)?;
            char_index_to_write = i + 1;
        }
    }
    w.write_all(&bytes[char_index_to_write..])
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

    #[test]
    fn short_strings_of_increasing_length() {
        for i in 0..128 {
            let xs = String::from("x").repeat(i);
            assert_eq!(format!("\"{}\"", xs), xs.to_json_string());

            let newlines = String::from("\n").repeat(i);
            assert_eq!(format!("\"{}\"", newlines.replace('\n', "\\n")), newlines.to_json_string());
        }
    }

    #[test]
    fn long_ascii_string() {
        let s = String::from("x").repeat(7919);
        assert_eq!(format!("\"{}\"", s), s.to_json_string());
    }

    #[test]
    fn long_nonascii_string() {
        let s = String::from("\u{2a6a5}").repeat(7919);
        assert_eq!(format!("\"{}\"", s), s.to_json_string());
    }

    #[test]
    fn long_mixed_string() {
        let source = String::from("0123456789abcdef0123456789abcdef\0").repeat(7919);
        let target = source.replace('\0', "\\u0000");
        assert_eq!(format!("\"{}\"", target), source.to_json_string());
    }

    #[test]
    fn many_backslashes() {
        let n = 7919;
        let s = String::from("\\").repeat(n);
        assert_eq!(format!("\"{}\"", s.repeat(2)), s.to_json_string());
    }
}
