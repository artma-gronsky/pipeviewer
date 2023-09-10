use std::io::Result;
use crossbeam::channel::Receiver;

pub fn stats_loop(silent: bool, stat_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;

    while let Ok(num_read) = stat_rx.recv() {
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
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
