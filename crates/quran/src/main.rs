use std::io::{self, BufWriter, ErrorKind, Write};

use clap::Parser;
use miette::{IntoDiagnostic, Report};
use quran::{Quran, Text};

/// Print Quranic text for a reference query.
///
/// Examples: `quran 2:255`, `quran 2:12-16`, `quran 2:{3,255}`, `quran '*:1'`.
///
/// Text: Tanzil Quran Text (Version 1.1), Tanzil Project, https://tanzil.net,
/// licensed under CC BY 3.0 and embedded verbatim. See `quran --license`.
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    /// References such as `2:255`, joined by spaces into one query.
    #[arg(required_unless_present = "license")]
    query: Vec<String>,

    /// Text to print.
    #[arg(long, value_enum, default_value_t)]
    text: Text,

    /// Print the Tanzil licence notice of the selected text.
    #[arg(long)]
    license: bool,
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let mut out = BufWriter::new(io::stdout().lock());
    let written = if cli.license {
        writeln!(
            out,
            "{}\n\nSource: Tanzil Project, https://tanzil.net",
            cli.text.notice()
        )
    } else {
        let query = cli.query.join(" ");
        let quran = Quran::load(cli.text)?;
        let lines = quran::parse(&query)
            .and_then(|parsed| quran::resolve(&parsed, &quran))
            .map_err(|error| Report::new(error).with_source_code(query))?;
        quran::write_lines(&mut out, &lines, &quran)
    };
    match written.and_then(|()| out.flush()) {
        Err(error) if error.kind() == ErrorKind::BrokenPipe => Ok(()),
        result => result.into_diagnostic(),
    }
}
