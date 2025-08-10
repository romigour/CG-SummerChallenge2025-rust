use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub enum InputSource {
    Stdin(io::Stdin),
    File(BufReader<File>),
}

impl InputSource {
    pub fn from_stdin() -> Self {
        InputSource::Stdin(io::stdin())
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(InputSource::File(BufReader::new(file)))
    }

    pub fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        buf.clear();
        match self {
            InputSource::Stdin(stdin) => stdin.read_line(buf),
            InputSource::File(reader) => reader.read_line(buf),
        }
    }
}
