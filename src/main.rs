use clap::{crate_version, value_t, App, Arg};
use regex::Regex;

mod extracter;
mod io;

use extracter::Extractor;

fn main() -> Result<(), String> {
    let app = App::new("rex")
        .version(crate_version!())
        .arg(Arg::with_name("regex").takes_value(true).required(true))
        .arg(
            Arg::with_name("input")
                .takes_value(true)
                .short("i")
                .long("input")
                .help("file to read")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("output")
                .takes_value(true)
                .short("o")
                .long("output")
                .help("file to write to")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("format")
                .takes_value(true)
                .short("f")
                .long("format")
                .possible_values(&["csv"])
                .default_value("csv"),
        );

    let matches = app.get_matches();

    let regex_param = value_t!(matches, "regex", String).unwrap();

    let regex = Regex::new(&regex_param).map_err(|_| format!("Invalid regex: {}", &regex_param))?;

    let extracter = Extractor::new(regex);

    let input_param = value_t!(matches, "input", String).unwrap();
    let input = io::Input::new(&input_param)
        .map_err(|_| format!("Unable to find input: {}", &input_param))?;

    let output_param = value_t!(matches, "output", String).unwrap();
    let format_param = value_t!(matches, "format", String).unwrap();

    let mut output = io::Output::new(&output_param, &format_param)
        .map_err(|_| format!("Unable to find output: {}", &output_param))?;

    output.headers(extracter.headers()).expect("todo");

    for line in input {
        if let Some(extraction) = extracter.extract(&line) {
            output.entry(&extraction).expect("todo");
        }
    }

    Ok(())
}
