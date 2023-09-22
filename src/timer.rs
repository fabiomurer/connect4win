#[cfg(target_family = "wasm")]
use instant::{Duration, Instant};
#[cfg(not(target_family = "wasm"))]
use std::time::{Duration, Instant};

use std::fmt;

#[derive(Debug, Clone)]
pub struct Timer {
    duration: Duration,
    start: Instant,
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
        Timer {
            duration: Duration::new(seconds, 0),
            start: Instant::now(),
        }
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

    pub fn set_duration(&mut self, seconds: u64) {
        self.duration = Duration::new(seconds, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    #[should_panic]
    fn elapsed() {
        let mut t = Timer::new(1);
        let second = time::Duration::from_secs(1);
        t.start();
        thread::sleep(second);
        t.check().unwrap();
    }
}
