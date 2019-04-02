//! Serialization to JSON strings like `"hello world \n"`
use super::JSONValue;
use std::io;

static ESCAPE_CHARS: [&'static [u8]; 0x20] = [
    b"\\u0000",
    b"\\u0001",
    b"\\u0002",
    b"\\u0003",
    b"\\u0004",
    b"\\u0005",
    b"\\u0006",
    b"\\u0007",
    b"\\b",
    b"\\t",
    b"\\n",
    b"\\u000b",
    b"\\f",
    b"\\r",
    b"\\u000e",
    b"\\u000f",
    b"\\u0010",
    b"\\u0011",
    b"\\u0012",
    b"\\u0013",
    b"\\u0014",
    b"\\u0015",
    b"\\u0016",
    b"\\u0017",
    b"\\u0018",
    b"\\u0019",
    b"\\u001a",
    b"\\u001b",
    b"\\u001c",
    b"\\u001d",
    b"\\u001e",
    b"\\u001f"
];

#[inline(always)]
fn json_escaped_char(c: u8) -> Option<&'static [u8]> {
    match c {
        x if x < 0x20 => Some(ESCAPE_CHARS[c as usize]),
        b'\\' => Some(&b"\\\\"[..]),
        b'\"' => Some(&b"\\\""[..]),
        _ => None
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
            w.write_all(escaped)?;
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
        write_json_common(self, w)?;
        w.write_all(b"\"")
    }
}

fn write_json_common<W: io::Write>(s: &str, w: &mut W) -> io::Result<()> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("sse4.2") {
                return unsafe { write_json_simd(s, w) };
            }
        }
    write_json_nosimd(s.as_bytes(), w)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse4.2")]
unsafe fn write_json_simd<W: io::Write>(s: &str, w: &mut W) -> io::Result<()> {
    use std::arch::x86_64::*;
    use std::mem::size_of;

    const VECTOR_SIZE: usize = size_of::<__m128i>();

    let bytes = s.as_bytes();
    let control_chars = _mm_setr_epi8(0, 0x1f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    let special_chars = _mm_setr_epi8(
        b'\\' as i8,
        b'"' as i8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    );

    let mut char_index_to_write = 0;
    let mut current_index = 0;
    for chunk_bytes in bytes.chunks(VECTOR_SIZE) {
        let current_chunk_len = chunk_bytes.len();
        let needs_write_at = if current_chunk_len != VECTOR_SIZE {
            0
        } else {
            let chunk = _mm_loadu_si128(chunk_bytes.as_ptr() as *const _);
            let idx_control_chars = _mm_cmpestri(
                control_chars,
                2,
                chunk,
                current_chunk_len as i32,
                _SIDD_CMP_RANGES,
            );
            let idx_special_chars = _mm_cmpestri(
                special_chars,
                2,
                chunk,
                current_chunk_len as i32,
                _SIDD_CMP_EQUAL_ANY,
            );
            idx_special_chars.min(idx_control_chars) as usize
        };
        if needs_write_at != VECTOR_SIZE {
            let end_idx = current_index + needs_write_at;
            w.write_all(&bytes[char_index_to_write..end_idx])?;
            write_json_nosimd(&chunk_bytes[needs_write_at..], w)?;
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
            w.write_all(escaped)?;
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
    fn test_string_with_control_chars() {
        assert_eq!(
            r#""0123456789\u001fabcde""#,
            "0123456789\x1Fabcde".to_json_string()
        );
        assert_eq!(
            r#""0123456789\u001eabcde""#,
            "0123456789\x1Eabcde".to_json_string()
        );
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
            assert_eq!(
                format!("\"{}\"", newlines.replace('\n', "\\n")),
                newlines.to_json_string()
            );
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
