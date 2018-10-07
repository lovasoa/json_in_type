use std::fmt;
use std::io;

pub struct FormatterWriter<'a, 'b>(pub &'a mut fmt::Formatter<'b>);

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