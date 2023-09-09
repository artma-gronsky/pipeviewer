use std::io::Result;

use pipeviewer::args::Args;
use pipeviewer::read::read;
use pipeviewer::stats::stats;
use pipeviewer::write::write;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut total_bytes = 0;

    loop {
        let buffer = match read(&args.infile) {
            Ok(v) if v.is_empty() => break,
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        stats(args.silent, buffer.len(), &mut total_bytes, false);

        if !write(&args.outfile, &buffer)? {
            break;
        }
    }

    stats(args.silent, 0, &mut total_bytes, true);
    Ok(())
}
