use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::sync::mpsc::Sender;

pub fn read_loop(infile: &str, sender: Sender<Vec<u8>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(size) if size == 0 => break,
            Ok(v) => v,
            Err(_) => break,
        };

        if sender.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }

    sender.send(Vec::new()).unwrap();
    Ok(())
}
