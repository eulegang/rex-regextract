use crate::extracter::Extraction;
use ::csv::Error as CsvError;
use std::fs::File;
use std::io::{self, Write};

mod csv;

pub enum Output {
    Csv(csv::CSV),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io failed: {0}")]
    IO(#[from] io::Error),

    #[error("failed to write csv: {0}")]
    Csv(#[from] CsvError),

    #[error("unsupported format")]
    UnsupportedFormat,
}

impl Output {
    pub fn new(out: &str, format: &str) -> Result<Output, Error> {
        let fout = match out {
            "-" => Fout::Stdout,
            a => Fout::File(File::create(a)?),
        };

        let output = match format.to_lowercase().as_str() {
            "csv" => Output::Csv(csv::CSV::create(fout)?),
            _ => return Err(Error::UnsupportedFormat),
        };

        Ok(output)
    }

    pub fn headers(&mut self, header: &[String]) -> Result<(), Error> {
        match self {
            Output::Csv(csv) => csv.headers(header).map_err(Into::into),
        }
    }

    pub fn entry(&mut self, extraction: &Extraction) -> Result<(), Error> {
        match self {
            Output::Csv(csv) => csv.entry(extraction).map_err(Into::into),
        }
    }
}

trait OutputDriver: Sized {
    type Error;
    fn create(fout: Fout) -> Result<Self, Self::Error>;
    fn headers(&mut self, header: &[String]) -> Result<(), Self::Error>;
    fn entry(&mut self, extraction: &Extraction) -> Result<(), Self::Error>;
}

enum Fout {
    File(File),
    Stdout,
    Bytes(Vec<u8>),
}

impl Write for Fout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Fout::File(f) => f.write(buf),
            Fout::Bytes(b) => b.write(buf),
            Fout::Stdout => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                handle.write(buf)
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Fout::File(f) => f.flush(),
            Fout::Bytes(b) => b.flush(),
            Fout::Stdout => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                handle.flush()
            }
        }
    }
}
