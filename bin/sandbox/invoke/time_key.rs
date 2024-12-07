use std::time::{Duration, Instant};

use super::{
    scheduler::{BoxableCallback},
    callback::Callback,
};

pub struct TimeKey {
    pub(crate) time: Instant,
    pub(crate) callback: Box<dyn Callback>,
}

impl TimeKey {
    #[inline]
    pub fn new(time: Instant, callback: Box<dyn Callback>) -> Self {
        Self {
            time,
            callback,
        }
    }

    #[inline]
    pub fn now<I, F: BoxableCallback<I>>(callback: F) -> Self {
        Self::new(Instant::now(), callback.into_box())
    }

    #[inline]
    pub fn after<I, F: BoxableCallback<I>>(duration: Duration, callback: F) -> Self {
        Self::new(Instant::now() + duration, callback.into_box())
    }

    #[inline]
    pub fn after_micros<I, F: BoxableCallback<I>>(micros: u64, callback: F) -> Self {
        Self::after(Duration::from_micros(micros), callback)
    }

    #[inline]
    pub fn after_millis<I, F: BoxableCallback<I>>(millis: u64, callback: F) -> Self {
        Self::after(Duration::from_millis(millis), callback)
    }

    #[inline]
    pub fn after_nanos<I, F: BoxableCallback<I>>(nanos: u64, callback: F) -> Self {
        Self::after(Duration::from_nanos(nanos), callback)
    }

    #[inline]
    pub fn after_secs<I, F: BoxableCallback<I>>(secs: u64, callback: F) -> Self {
        Self::after(Duration::from_secs(secs), callback)
    }

    #[inline]
    pub fn after_secs_f32<I, F: BoxableCallback<I>>(secs_f32: f32, callback: F) -> Self {
        Self::after(Duration::from_secs_f32(secs_f32), callback)
    }

    #[inline]
    pub fn after_secs_f64<I, F: BoxableCallback<I>>(secs_f64: f64, callback: F) -> Self {
        Self::after(Duration::from_secs_f64(secs_f64), callback)
    }

    #[inline]
    pub fn after_mins<I, F: BoxableCallback<I>>(mins: u64, callback: F) -> Self {
        Self::after(Duration::from_secs(mins * 60), callback)
    }

    #[inline]
    pub fn after_mins_f32<I, F: BoxableCallback<I>>(mins: f32, callback: F) -> Self {
        Self::after(Duration::from_secs_f32(mins * 60.0), callback)
    }

    #[inline]
    pub fn after_mins_f64<I, F: BoxableCallback<I>>(mins: f64, callback: F) -> Self {
        Self::after(Duration::from_secs_f64(mins * 60.0), callback)
    }

    #[inline]
    pub fn after_hours<I, F: BoxableCallback<I>>(hours: u64, callback: F) -> Self {
        Self::after(Duration::from_secs(hours * 3600), callback)
    }

    #[inline]
    pub fn after_hours_f32<I, F: BoxableCallback<I>>(hours: f32, callback: F) -> Self {
        Self::after(Duration::from_secs_f32(hours * 3600.0), callback)
    }

    #[inline]
    pub fn after_hours_f64<I, F: BoxableCallback<I>>(hours: f64, callback: F) -> Self {
        Self::after(Duration::from_secs_f64(hours * 3600.0), callback)
    }

    #[inline]
    pub fn after_days<I, F: BoxableCallback<I>>(days: u64, callback: F) -> Self {
        Self::after(Duration::from_secs(days * 86400), callback)
    }

    #[inline]
    pub fn after_days_f32<I, F: BoxableCallback<I>>(days: f32, callback: F) -> Self {
        Self::after(Duration::from_secs_f32(days * 86400.0), callback)
    }

    #[inline]
    pub fn after_days_f64<I, F: BoxableCallback<I>>(days: f64, callback: F) -> Self {
        Self::after(Duration::from_secs_f64(days * 86400.0), callback)
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        Instant::now() >= self.time
    }
}

impl PartialEq<TimeKey> for TimeKey {
    fn eq(&self, other: &TimeKey) -> bool {
        self.time == other.time
    }
}

impl Eq for TimeKey {}

impl Ord for TimeKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd<TimeKey> for TimeKey {
    fn partial_cmp(&self, other: &TimeKey) -> Option<std::cmp::Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

impl PartialEq<Instant> for TimeKey {
    fn eq(&self, other: &Instant) -> bool {
        self.time.eq(other)
    }
}

impl PartialOrd<Instant> for TimeKey {
    fn partial_cmp(&self, other: &Instant) -> Option<std::cmp::Ordering> {
        other.partial_cmp(&self.time)
    }
}