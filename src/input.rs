use std::io::{self, BufRead, BufReader, Read};

pub struct Input<R> {
    reader: BufReader<R>,
    buffer: Vec<u8>,
}

impl<R> Iterator for Input<R>
where
    R: Read,
{
    type Item = Result<String, io::Error>;

    fn next(&mut self) -> Option<Result<String, io::Error>> {
        if let Err(e) = self.reader.read_until(b'"', &mut self.buffer) {
            return Some(Err(e));
        }

        self.buffer.clear();

        if let Err(e) = self.reader.read_until(b'"', &mut self.buffer) {
            return Some(Err(e));
        }

        if self.buffer.is_empty() {
            return None;
        }

        match String::from_utf8(self.buffer[..(self.buffer.len() - 1)].to_vec()) {
            Ok(s) => Some(Ok(s)),
            Err(e) => Some(Err(io::Error::new(io::ErrorKind::Other, e))),
        }
    }
}

pub fn parse<R: Read>(reader: R) -> Input<R> {
    Input {
        reader: BufReader::new(reader),
        buffer: vec![],
    }
}
