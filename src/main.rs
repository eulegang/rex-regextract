use eyre::WrapErr;
use regex::Regex;
use structopt::StructOpt;

mod extracter;
mod format;
mod io;

use extracter::Extractor;

#[derive(StructOpt)]
#[structopt(name = "rex", about = "regex extractor")]
pub struct RexCli {
    /// Regex to search input
    regex: String,

    /// File to use as input (- for stdin)
    #[structopt(short, long, default_value = "-")]
    input: io::Input,

    /// File to use as output (- for stdout)
    #[structopt(short, long, default_value = "-")]
    output: io::Output,

    /// Format to export
    #[structopt(short, long, default_value = "csv", possible_values = format::FORMATS)]
    format: format::Format,
}

fn main() -> eyre::Result<()> {
    let mut cli = RexCli::from_args();

    let regex = Regex::new(&cli.regex).wrap_err("Invalid regex")?;
    let extracter = Extractor::new(regex);

    cli.format
        .begin(&mut cli.output, extracter.headers())
        .wrap_err("Unable to write out header information")?;

    let mut first = true;
    for line in cli.input {
        if let Some(extraction) = extracter.extract(&line) {
            cli.format
                .line_match(&mut cli.output, &extraction, first)
                .wrap_err("Failed to write record information")?;
            first = false;
        }
    }

    cli.format
        .end(&mut cli.output)
        .wrap_err("Unable to write out trailer information")?;

    Ok(())
}
