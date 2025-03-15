use std::time::Duration;
use chrono::DateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameTime {
    duration: Duration,
}

impl std::ops::Deref for FrameTime {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.duration
    }
}

impl FrameTime {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    pub fn fps(&self) -> f64 {
        1.0 / self.duration.as_secs_f64()
    }
}

#[derive(Debug, Clone)]
pub struct FrameInfo {
    pub index: u64,
    pub average_frame_time: FrameTime,
    pub delta_time: FrameTime,
    pub run_time: Duration,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dur_test() {
        let dur = (Duration::from_secs(3) + Duration::from_secs(3) + Duration::from_secs(3)) / 111u32;
        println!("{}", dur.as_micros());
    }
}