use std::{
    io::Result,
    sync::mpsc::{Receiver, Sender},
};
pub fn stats_loop(
    silent: bool,
    receiver: Receiver<Vec<u8>>,
    write_sender: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;

    while let Ok(buffer) = receiver.recv() {
        let num_read = buffer.len();
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if write_sender.send(buffer).is_err() {
            break;
        }

        if num_read == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }
    Ok(())
}
