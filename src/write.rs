use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
    sync::mpsc::Receiver,
};

pub fn write_loop(outfile: &str, receiver: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    while let Ok(buffer) = receiver.recv() {
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
