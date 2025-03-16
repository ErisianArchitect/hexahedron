use std::time::{Duration, Instant};

use hexcore::extensions::Replace;
use hexmath::average::{AverageBuffer, AvgBuffer};

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Spacing {
    #[default]
    Immediate,
    Fixed(Duration),
}

impl Spacing {
    pub fn is_ready(&self, last_time: Instant, early: Option<Duration>) -> bool {
        match self {
            Spacing::Immediate => true,
            Spacing::Fixed(duration) => {
                match early {
                    Some(early) => (last_time - early).elapsed().ge(duration),
                    None => last_time.elapsed().ge(duration),
                }
            },
        }
    }

    pub fn fixed_step(&self) -> Option<Duration> {
        match self {
            Spacing::Immediate => None,
            &Spacing::Fixed(duration) => Some(duration),
        }
    }
}

pub struct FrameSpace {
    begin_frame_time: Instant,
    last_frame_end_time: Instant,
    // Fixed Update
    fixed_time: Instant,
    fixed_timestep: Duration,
    average_fixed_update_time: AverageBuffer<Duration>,
    // Update

    // Render
    render_spacing: Spacing,
    average_render_time: AverageBuffer<Duration>,
    last_render_time: Instant,


}

impl FrameSpace {

    pub fn duration_since_last_frame(&self) -> Duration {
        self.last_frame_end_time.elapsed()
    }

    pub fn fixed_updates<T, F: Fn(&mut Self, T) -> ()>(&mut self, arg: T, fixed_update: F) {
        todo!()
    }

    pub fn update<T, R, F: FnOnce(&mut Self, T) -> R>(&mut self, arg: T, update: F) -> R{
        todo!()
    }

    pub fn render<T, R, F: FnOnce(&mut Self, T) -> R>(&mut self, arg: T, render: F) -> Option<R> {
        if self.render_spacing.is_ready(self.last_render_time, Some(self.average_render_time.average())) {
            Some(render(self, arg))
        } else {
            None
        }
    }

    /// Call this function at the beginning of the frame to put the thread
    /// to sleep until the target time. If your game loop is running too fast,
    /// this will slow it down to the target rate.
    pub fn rate_limit(&self, target_fps: u32) {
        todo!()
    }
}