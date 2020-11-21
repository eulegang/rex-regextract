use std::fs::File;
use std::io::{self, Write};
use std::str::FromStr;

pub enum Output {
    File(File),
    Stdout,
}

#[derive(thiserror::Error, Debug)]
pub enum OutputError {
    #[error("io error: {0}")]
    IO(#[from] io::Error),

    #[error("unable to create \"{0}\": {1}")]
    FileCreate(String, io::Error),
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::File(f) => f.write(buf),
            Output::Stdout => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                handle.write(buf)
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::File(f) => f.flush(),
            Output::Stdout => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                handle.flush()
            }
        }
    }
}

impl FromStr for Output {
    type Err = OutputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Output::Stdout),
            filename => {
                Ok(Output::File(File::create(filename).map_err(|e| {
                    OutputError::FileCreate(filename.to_string(), e)
                })?))
            }
        }
    }
}
