use std::io;
use std::io::{Error, ErrorKind, Write};

pub struct Writer<'w> {
    out: &'w mut dyn Write,
    buffer: String,
    extra: String,
}

impl<'w> Writer<'w> {
    pub fn new (out: &'w mut dyn Write) -> Self {
        Writer {
            out,
            buffer : "".to_string(),
            extra: "".to_string(),
        }
    }

    pub fn write(&mut self, code: u8, raw: bool) -> io::Result<()> {
        let _code = match raw {
            true => {format!("{code:08b}")}
            false => {format!("{code:b}")}
        };

        _code.chars().for_each(|f| {
            if self.buffer.len() < 8 {
                self.buffer.push(f);
            } else {
                self.extra.push(f);
            }
        });

        if self.buffer.len() == 8 {
            let bin = u8::from_str_radix(self.buffer.as_str(), 2).unwrap();
            println!("{0} ({1}) was written in the buffer.", self.buffer, bin);

            self.out.write(&[bin])
                .expect("Error while writing on the buffer");

            self.buffer = self.extra.clone();
            self.extra = "".to_owned();
        }

        if !self.extra.is_empty() {
            println!("shit {1:?} {}", self.extra.len(), self.extra);
            return Err(Error::new(ErrorKind::Unsupported, "Should not get here"));
        }

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if !self.buffer.is_empty() {
            self.out.write(&[u8::from_str_radix(self.buffer.as_str(), 2).unwrap()])?;
            let bin = u8::from_str_radix(self.buffer.as_str(), 2).unwrap();
            println!("{0} ({1}) was flushed into the buffer.", self.buffer, bin);
        }
        Ok(())
    }
}