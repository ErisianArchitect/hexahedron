use std::time::{Duration, Instant};

use hexcore::extensions::Replace;

#[derive(Debug, Clone)]
pub struct FrameLimiter {
    last_frame: Instant,
    min_time: Option<Duration>,
}

impl FrameLimiter {
    pub fn start_new<M: Into<Option<Duration>>>(min_time: M) -> Self {
        Self {
            last_frame: Instant::now(),
            min_time: min_time.into(),
        }
    }
    
    pub fn set_last_frame(&mut self, time: Instant) -> Instant {
        self.last_frame.replace(time)
    }

    pub fn frame_start(&mut self) -> Instant {
        self.last_frame.replace(Instant::now())
    }

    pub fn should_begin_frame(&self) -> bool {
        match &self.min_time {
            None => true,
            Some(time) => {
                self.last_frame.elapsed().ge(time)
            }
        }
    }
}

pub struct FrameSpace {
    begin_frame_time: Instant,
    last_frame_end_time: Instant,
}

impl FrameSpace {

    pub fn duration_since_last_frame(&self) -> Duration {
        self.last_frame_end_time.elapsed()
    }

    /// Call this function at the beginning of the frame to put the thread
    /// to sleep until the target time. If your game loop is running too fast,
    /// this will slow it down to the target rate.
    pub fn rate_limit(&self, target_fps: u32) {

    }
}