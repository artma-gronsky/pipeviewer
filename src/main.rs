use std::env;
use std::io::{self, Result, Read, Write, ErrorKind};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()>{
    let silent = !env::var("PV_SILENT").unwrap_or(String::new()).is_empty();

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

        if let Err(e) = io::stdout().write_all(&buffer[..num_read]){
            if e.kind() == ErrorKind::BrokenPipe{
                break;
            }
            eprintln!("Oh no, an error! {}", e);
            std::process::exit(1);
        };
    }
 
    Ok(())
}
