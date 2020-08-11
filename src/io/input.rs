use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub enum Input<'b> {
    File(BufReader<File>),
    Stdio,
    Bytes(&'b [u8]),
}

impl Input<'_> {
    pub fn new(path: &str) -> io::Result<Input> {
        match path {
            "-" => Ok(Input::Stdio),
            otherwise => File::open(otherwise).map(BufReader::new).map(Input::File),
        }
    }
}

impl Iterator for Input<'_> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut buf = String::new();

        let size = match self {
            Input::File(f) => f.read_line(&mut buf),
            Input::Bytes(b) => b.read_line(&mut buf),
            Input::Stdio => {
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                handle.read_line(&mut buf)
            }
        };

        match size {
            Ok(0) | Err(_) => None,
            Ok(_) => {
                chomp(&mut buf);
                Some(buf)
            }
        }
    }
}

fn chomp(buf: &mut String) {
    if buf.ends_with('\n') {
        buf.pop();
    }

    if buf.ends_with('\r') {
        buf.pop();
    }
}
