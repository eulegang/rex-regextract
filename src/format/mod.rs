use crate::extracter::Extraction;
use std::io::{self, Write};
use std::str::FromStr;

#[cfg(test)]
mod test;

pub const FORMATS: &[&str] = &[
    "csv",
    "csv-no-header",
    "tsv",
    "tsv-no-header",
    "json",
    "ldjson",
    "json-seq",
];

#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    Csv,
    CsvNoHeader,
    Tsv,
    TsvNoHeader,
    Json,
    LDJson,
    JsonSeq,
}

impl Format {
    pub fn begin(&self, out: &mut impl Write, headers: &[String]) -> io::Result<()> {
        use Format::*;

        match self {
            Csv => {
                writeln!(out, "{}", headers.join(","))
            }

            Tsv => {
                writeln!(out, "{}", headers.join("\t"))
            }

            Json => {
                writeln!(out, "[")
            }

            _ => Ok(()),
        }
    }

    pub fn line_match(
        &self,
        out: &mut impl Write,
        extraction: &Extraction,
        first: bool,
    ) -> io::Result<()> {
        use Format::*;

        match self {
            Json => {
                if !first {
                    writeln!(out, ",")?;
                }

                write!(out, "  {}", json_format(extraction))
            }

            JsonSeq => writeln!(out, "\x1e{}", json_format(extraction)),
            LDJson => writeln!(out, "{}", json_format(extraction)),
            Csv | CsvNoHeader => writeln!(out, "{}", sv_format(extraction, ",")),
            Tsv | TsvNoHeader => writeln!(out, "{}", sv_format(extraction, "\t")),
        }
    }

    pub fn end(&self, out: &mut impl Write) -> io::Result<()> {
        use Format::*;

        match self {
            Json => writeln!(out, "\n]"),
            _ => Ok(()),
        }
    }
}

fn json_format(extraction: &Extraction) -> String {
    let mut kvs = Vec::new();
    for (header, value) in extraction.pairs() {
        let header = json_str_encode(header);
        let value = json_str_encode(value);

        kvs.push(format!("\"{}\":\"{}\"", header, value));
    }

    format!("{{{}}}", kvs.join(","))
}

fn json_str_encode(input: &str) -> String {
    input
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}

fn sv_format(extraction: &Extraction, sep: &str) -> String {
    let mut buf = Vec::new();

    for val in extraction.values() {
        let has_double = val.contains('"');
        let has_sep = val.contains(sep);
        let line = val.contains('\n');

        let mut rep = if has_double {
            val.replace('"', "\"\"")
        } else {
            val.to_string()
        };

        if has_double || has_sep || line {
            rep.insert(0, '"');
            rep.push('"');
        }

        buf.push(rep);
    }

    buf.join(sep)
}

impl FromStr for Format {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, FormatError> {
        use Format::*;

        match s {
            "csv" => Ok(Csv),
            "csv-no-header" => Ok(CsvNoHeader),
            "tsv" => Ok(Tsv),
            "tsv-no-header" => Ok(TsvNoHeader),
            "json" => Ok(Json),
            "ldjson" => Ok(LDJson),
            "json-seq" => Ok(JsonSeq),

            s => Err(FormatError::DoesNotExist(s.to_string())),
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum FormatError {
    #[error("{0} is not a valid format")]
    DoesNotExist(String),
}
