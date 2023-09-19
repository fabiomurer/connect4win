use std::fmt;

#[derive(Debug, Clone)]
pub struct Timer {
    counter: u64,
    duration: u64,
}

#[derive(Debug)]
pub struct TimeoutError;

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "time is up :(")
    }
}

impl Timer {
    pub fn new(cycles: u64) -> Timer {
        Timer {
            duration: cycles,
            counter: 0,
        }
    }

    pub fn start(&mut self) {
        self.counter = 0;
    }

    pub fn check(&mut self) -> Result<u64, TimeoutError> {
        self.counter += 1;
        if self.counter >= self.duration {
            Err(TimeoutError)
        } else {
            Ok(self.counter)
        }
    }
}