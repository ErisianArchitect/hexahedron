use std::time::{Instant, Duration};
use crate::extensions::*;

#[inline]
pub fn now() -> Instant {
    Instant::now()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timer {
    time: Instant,
}

impl Timer {
    /// Starts the timer.
    #[inline]
    pub fn start() -> Self {
        Self { time: Instant::now() }
    }

    pub fn start_time(self) -> Instant {
        self.time
    }

    /// Returns the elapsed time.
    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.time.elapsed()
    }

    /// Returns the elapsed time and resets the timer (starting from this point).
    #[inline]
    pub fn time(&mut self) -> Duration {
        self.time.replace(Instant::now()).elapsed()
    }

    #[inline]
    pub fn reset(&mut self) {
        self.time = Instant::now();
    }
    
    #[inline]
    pub fn measure_time<R, F: FnOnce() -> R>(f: F) -> (Duration, R) {
        let timer = Timer::start();
        let result = f();
        (
            timer.elapsed(),
            result,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timer_test() {
        let (time, _) = Timer::measure_time(|| {
            std::thread::sleep(Duration::from_millis(16));
        });
        println!("{time:?}");
    }
}