use std::{time::{Duration, Instant}, sync::mpsc::RecvTimeoutError, fmt::write};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Timer {
    duration: Duration,
    start: Instant
}

#[derive(Debug)]
pub struct TimeoutError; 

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "time is up :(")
    }
}

impl Timer {
    pub fn new(seconds: u64) -> Timer {
        Timer { duration: Duration::new(seconds, 0), start: Instant::now() }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn check(&self) -> Result<Duration, TimeoutError> {
        let elapsed = self.start.elapsed();
        if elapsed >= self.duration {
            Err(TimeoutError)
        } else {
            Ok(elapsed)
        }
    }
}

