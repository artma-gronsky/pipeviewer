use crossbeam::channel::Receiver;
use std::io::Result;
use std::time::{Duration, Instant};

pub fn stats_loop(silent: bool, stat_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;

    let start = Instant::now();
    let mut timer = Timer::new();
    let mut rate_per_second = 0.0;
    while let Ok(num_read) = stat_rx.recv() {
        timer.update();
        rate_per_second = num_read as f64 / timer.delta.as_secs_f64();
        total_bytes += num_read;
        if !silent && timer.ready {
            timer.ready = false;
            eprint!(
                "\r{} {} [{:.0} b/s]",
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second
            );
        }

        if num_read == 0 {
            break;
        }
    }

    if !silent {
        eprintln!(
            "\r{} {} [{:.0} b/s]",
            total_bytes,
            start.elapsed().as_secs().as_time(),
            rate_per_second
        );
    }
    Ok(())
}

trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

struct Timer {
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    countdown: Duration,
    ready: bool,
}

impl Timer {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;

        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        })
    }
}
