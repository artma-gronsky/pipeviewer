use clap::Arg;
use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = clap::Command::new("pipeviewer")
        .bin_name("pipeviewer")
        //.arg_required_else_help(true)
        .args([
            Arg::new("infile").long("infile").help("Read from a file instead of stdin"),
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

    dbg!(infile, outfile, silent);

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read: usize = match io::stdin().read(&mut buffer) {
            Ok(num) if num == 0 => break,
            Ok(num) => num,
            Err(e) => return Err(e),
        };

        total_bytes += num_read;

        if !silent {
            eprint!("\r total_bytes: {}", total_bytes);
        }

        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            eprintln!("\n Oh no, an error! {}", e);
            std::process::exit(1);
        };
    }

    Ok(())
}
