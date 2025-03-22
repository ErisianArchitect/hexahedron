use std::time::{Duration, Instant};

use hexcore::extensions::Replace;
use hexmath::average::{AverageBuffer, AvgBuffer};

use crate::engine::Engine;

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
    // Fixed Update
    pub last_fixed_time: Instant,
    pub fixed_timestep: Duration,
    // Frame Spacing (Update/Render)
    pub last_frame_time: Instant,
    pub frame_spacing: Spacing,
    pub average_frame_time: AverageBuffer<Duration>,
}

pub struct FramespaceResult<T> {
    pub result: T,
    pub duration: Duration,
}

impl FrameSpace {

    pub fn fixed_updates<F: FnMut(Duration) -> ()>(&mut self, mut fixed_update: F) {
        let mut last_fixed_time = self.last_fixed_time;
        while last_fixed_time.elapsed() >= self.fixed_timestep {
            fixed_update(self.fixed_timestep);
            last_fixed_time += self.fixed_timestep;
        }
        self.last_fixed_time = last_fixed_time;
    }

    pub fn frame<R, F: FnOnce() -> R>(&mut self, frame: F) -> Option<FramespaceResult<R>> {
        if self.frame_spacing.is_ready(self.last_frame_time, Some(self.average_frame_time.average())) {
            let hexcore::macros::MeasureTimeResult {
                start_time,
                result,
                duration,
            } = hexcore::measure_time!{
                frame()
            };
            self.last_frame_time = start_time + duration;
            self.average_frame_time.push(duration);
            Some(FramespaceResult { result, duration })
        } else {
            None
        }
    }
}