use std::time::{Duration, Instant};

use super::{
    callback::{Callback, IntoCallback}, scheduler_context::SchedulerContext
};

pub struct TimeKey<Ctx: SchedulerContext> {
    pub(crate) time: Instant,
    pub(crate) callback: Box<dyn Callback<Ctx>>,
}

impl<Ctx: SchedulerContext> TimeKey<Ctx> {
    #[inline]
    pub fn new(time: Instant, callback: Box<dyn Callback<Ctx>>) -> Self {
        Self {
            time,
            callback,
        }
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        Instant::now() >= self.time
    }
}

impl<Ctx: SchedulerContext> PartialEq<TimeKey<Ctx>> for TimeKey<Ctx> {
    fn eq(&self, other: &TimeKey<Ctx>) -> bool {
        self.time == other.time
    }
}

impl<Ctx: SchedulerContext> Eq for TimeKey<Ctx> {}

impl<Ctx: SchedulerContext> Ord for TimeKey<Ctx> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl<Ctx: SchedulerContext> PartialOrd<TimeKey<Ctx>> for TimeKey<Ctx> {
    fn partial_cmp(&self, other: &TimeKey<Ctx>) -> Option<std::cmp::Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

impl<Ctx: SchedulerContext> PartialEq<Instant> for TimeKey<Ctx> {
    fn eq(&self, other: &Instant) -> bool {
        self.time.eq(other)
    }
}

impl<Ctx: SchedulerContext> PartialOrd<Instant> for TimeKey<Ctx> {
    fn partial_cmp(&self, other: &Instant) -> Option<std::cmp::Ordering> {
        other.partial_cmp(&self.time)
    }
}