use super::{Fout, OutputDriver};
use crate::extracter::Extraction;

pub struct CSV {
    writer: csv::Writer<Fout>,
}

impl OutputDriver for CSV {
    type Error = csv::Error;

    fn create(fout: Fout) -> Result<Self, Self::Error> {
        let writer = csv::Writer::from_writer(fout);

        Ok(CSV { writer })
    }

    fn headers(&mut self, header: &[String]) -> Result<(), Self::Error> {
        self.writer.write_record(header)
    }

    fn entry(&mut self, extraction: &Extraction) -> Result<(), Self::Error> {
        self.writer.write_record(&extraction.values)
    }
}
