use std::time::{Duration, Instant};

use super::{
    callback::Callback, scheduler::IntoCallback, scheduler_context::SchedulerContext
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
    pub fn now<I, F: IntoCallback<Ctx, I>>(callback: F) -> Self {
        Self::new(Instant::now(), Box::new(callback.into_callback()))
    }

    #[inline]
    pub fn after<I, F: IntoCallback<Ctx, I>>(duration: Duration, callback: F) -> Self {
        Self::new(Instant::now() + duration, Box::new(callback.into_callback()))
    }

    #[inline]
    pub fn after_micros<I, F: IntoCallback<Ctx, I>>(micros: u64, callback: F) -> Self {
        Self::after(Duration::from_micros(micros), callback)
    }

    #[inline]
    pub fn after_millis<I, F: IntoCallback<Ctx, I>>(millis: u64, callback: F) -> Self {
        Self::after(Duration::from_millis(millis), callback)
    }

    #[inline]
    pub fn after_nanos<I, F: IntoCallback<Ctx, I>>(nanos: u64, callback: F) -> Self {
        Self::after(Duration::from_nanos(nanos), callback)
    }

    #[inline]
    pub fn after_secs<I, F: IntoCallback<Ctx, I>>(secs: u64, callback: F) -> Self {
        Self::after(Duration::from_secs(secs), callback)
    }

    #[inline]
    pub fn after_secs_f32<I, F: IntoCallback<Ctx, I>>(secs_f32: f32, callback: F) -> Self {
        Self::after(Duration::from_secs_f32(secs_f32), callback)
    }

    #[inline]
    pub fn after_secs_f64<I, F: IntoCallback<Ctx, I>>(secs_f64: f64, callback: F) -> Self {
        Self::after(Duration::from_secs_f64(secs_f64), callback)
    }

    #[inline]
    pub fn after_mins<I, F: IntoCallback<Ctx, I>>(mins: u64, callback: F) -> Self {
        Self::after(Duration::from_secs(mins * 60), callback)
    }

    #[inline]
    pub fn after_mins_f32<I, F: IntoCallback<Ctx, I>>(mins: f32, callback: F) -> Self {
        Self::after(Duration::from_secs_f32(mins * 60.0), callback)
    }

    #[inline]
    pub fn after_mins_f64<I, F: IntoCallback<Ctx, I>>(mins: f64, callback: F) -> Self {
        Self::after(Duration::from_secs_f64(mins * 60.0), callback)
    }

    #[inline]
    pub fn after_hours<I, F: IntoCallback<Ctx, I>>(hours: u64, callback: F) -> Self {
        Self::after(Duration::from_secs(hours * 3600), callback)
    }

    #[inline]
    pub fn after_hours_f32<I, F: IntoCallback<Ctx, I>>(hours: f32, callback: F) -> Self {
        Self::after(Duration::from_secs_f32(hours * 3600.0), callback)
    }

    #[inline]
    pub fn after_hours_f64<I, F: IntoCallback<Ctx, I>>(hours: f64, callback: F) -> Self {
        Self::after(Duration::from_secs_f64(hours * 3600.0), callback)
    }

    #[inline]
    pub fn after_days<I, F: IntoCallback<Ctx, I>>(days: u64, callback: F) -> Self {
        Self::after(Duration::from_secs(days * 86400), callback)
    }

    #[inline]
    pub fn after_days_f32<I, F: IntoCallback<Ctx, I>>(days: f32, callback: F) -> Self {
        Self::after(Duration::from_secs_f32(days * 86400.0), callback)
    }

    #[inline]
    pub fn after_days_f64<I, F: IntoCallback<Ctx, I>>(days: f64, callback: F) -> Self {
        Self::after(Duration::from_secs_f64(days * 86400.0), callback)
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