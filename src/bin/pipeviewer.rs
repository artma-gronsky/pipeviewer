use crossbeam::channel::{bounded, unbounded};

use std::io::Result;

use pipeviewer::args::Args;
use pipeviewer::read;
use pipeviewer::stats;
use pipeviewer::write;
use std::thread;

fn main() -> Result<()> {
    let Args {
        infile,
        outfile,
        silent,
    } = Args::parse();

    let (write_tx, write_rx) = bounded(1024);
    let (stat_tx, stat_rx) = unbounded();

    let infile1 = infile.to_owned();
    let read_handle = thread::spawn(move || read::read_loop(&infile1, write_tx, stat_tx));

    let stats_handler = thread::spawn(move || stats::stats_loop(silent, stat_rx));

    let outfile1 = outfile.to_owned();
    let write_handler = thread::spawn(move || write::write_loop(&outfile1, write_rx));

    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handler.join().unwrap();
    let write_io_result = write_handler.join().unwrap();
    read_io_result?;
    stats_io_result?;
    write_io_result?;
    Ok(())
}
