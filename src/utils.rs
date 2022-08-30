//! Useful tools when working with the types in this crate

use std::fmt;
use std::io;
use std::str;

/// Converts a Formatter to a Writer
pub struct FormatterWriter<'a, 'b: 'a>(pub &'a mut fmt::Formatter<'b>);

impl<'a, 'b: 'a> io::Write for FormatterWriter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf_str = match str::from_utf8(buf) {
            Ok(buf_str) => buf_str,
            // SAFETY: str::from_utf8() certifies that a part of the string is valid UTF-8.
            Err(err) => unsafe { str::from_utf8_unchecked(&buf[0..err.valid_up_to()]) },
        };
        match self.0.write_str(buf_str) {
            Ok(()) => Ok(buf_str.len()),
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::JSON;
    use std::fmt::Write;

    #[test]
    fn format_unicode() {
        let mut output = String::new();
        write!(&mut output, "{}", JSON("🤨🤨\n😮😮\n🤨🤨\n😮😮OMG"))
            .expect("no formatting errors");
        assert_eq!(output, r#""🤨🤨\n😮😮\n🤨🤨\n😮😮OMG""#);
    }
}
