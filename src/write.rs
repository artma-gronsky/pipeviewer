use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
};

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    if let Err(e) = writer.write_all(buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(false);
        }

        eprintln!("\n Oh no, an error! {}", e);
        std::process::exit(1);
    };

    Ok(true)
}
