use clap::Arg;
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = clap::Command::new("pipeviewer")
            .bin_name("pipeviewer")
            .args([
                Arg::new("infile")
                    .long("infile")
                    .help("Read from a file instead of stdin"),
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .help("Whrite output to a file instead of stdout"),
            ])
            .arg(Arg::new("silent").short('s').long("silent"))
            .get_matches();

        let infile = matches
            .get_one::<String>("infile")
            .map(|s| s.to_owned())
            .unwrap_or_default();
        let outfile = matches
            .get_one::<String>("outfile")
            .map(|s| s.to_owned())
            .unwrap_or_default();

        let silent = if matches.contains_id("silent") {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or(String::new()).is_empty()
        };

        Self {
            infile,
            outfile,
            silent,
        }
    }
}
