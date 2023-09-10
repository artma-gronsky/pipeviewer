use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
};

use crossbeam::channel::Receiver;

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    while let Ok(buffer) = write_rx.recv() {
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }

            eprintln!("\n Oh no, an error! {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
