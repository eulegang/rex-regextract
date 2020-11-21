use eyre::WrapErr;
use regex::Regex;
use structopt::StructOpt;

mod extracter;
mod io;

use extracter::Extractor;

#[derive(StructOpt)]
#[structopt(name = "rex", about = "regex extractor")]
pub struct RexCli {
    /// Regex to search input
    regex: String,

    /// File to use as input (- for stdin)
    #[structopt(short, long, default_value = "-")]
    input: String,

    /// File to use as output (- for stdout)
    #[structopt(short, long, default_value = "-")]
    output: String,

    /// Format to export
    #[structopt(short, long, default_value = "csv")]
    format: String,
}

fn main() -> eyre::Result<()> {
    let cli = RexCli::from_args();

    let regex = Regex::new(&cli.regex).wrap_err("Invalid regex")?;
    let extracter = Extractor::new(regex);

    let input = io::Input::new(&cli.input).wrap_err("failed to setup input")?;
    let mut output =
        io::Output::new(&cli.output, &cli.format).wrap_err("failed to setup output")?;

    output
        .headers(extracter.headers())
        .wrap_err("failed to set headers")?;

    for line in input {
        if let Some(extraction) = extracter.extract(&line) {
            output.entry(&extraction).expect("todo");
        }
    }

    Ok(())
}
