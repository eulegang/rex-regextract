use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

pub enum Input {
    File(BufReader<File>),
    Stdio,
}

#[derive(thiserror::Error, Debug)]
pub enum InputError {
    #[error("Failed to read \"#{0}\": {1}")]
    FileRead(String, io::Error),
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, InputError> {
        match s {
            "-" => Ok(Input::Stdio),
            filename => {
                let file = File::open(filename)
                    .map_err(|e| InputError::FileRead(filename.to_string(), e))?;
                Ok(Input::File(BufReader::new(file)))
            }
        }
    }
}

impl Iterator for Input {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut buf = String::new();

        let size = match self {
            Input::File(f) => f.read_line(&mut buf),
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
