use std::time::{Duration, Instant};

pub struct TimeKey<T> {
    pub(crate) time: Instant,
    pub(crate) value: T,
}

impl<T> TimeKey<T> {
    #[inline]
    pub fn new(time: Instant, value: T) -> Self {
        Self {
            time,
            value,
        }
    }
    #[inline]
    pub fn now(value: T) -> Self {
        Self::new(Instant::now(), value)
    }

    #[inline]
    pub fn after(duration: Duration, value: T) -> Self {
        Self::new(Instant::now() + duration, value)
    }

    #[inline]
    pub fn after_micros(micros: u64, value: T) -> Self {
        Self::after(Duration::from_micros(micros), value)
    }

    #[inline]
    pub fn after_millis(millis: u64, value: T) -> Self {
        Self::after(Duration::from_millis(millis), value)
    }

    #[inline]
    pub fn after_nanos(nanos: u64, value: T) -> Self {
        Self::after(Duration::from_nanos(nanos), value)
    }

    #[inline]
    pub fn after_secs(secs: u64, value: T) -> Self {
        Self::after(Duration::from_secs(secs), value)
    }

    #[inline]
    pub fn after_secs_f32(secs_f32: f32, value: T) -> Self {
        Self::after(Duration::from_secs_f32(secs_f32), value)
    }

    #[inline]
    pub fn after_secs_f64(secs_f64: f64, value: T) -> Self {
        Self::after(Duration::from_secs_f64(secs_f64), value)
    }

    #[inline]
    pub fn after_mins(mins: u64, value: T) -> Self {
        Self::after(Duration::from_secs(mins * 60), value)
    }

    #[inline]
    pub fn after_mins_f32(mins: f32, value: T) -> Self {
        Self::after(Duration::from_secs_f32(mins * 60.0), value)
    }

    #[inline]
    pub fn after_mins_f64(mins: f64, value: T) -> Self {
        Self::after(Duration::from_secs_f64(mins * 60.0), value)
    }

    #[inline]
    pub fn after_hours(hours: u64, value: T) -> Self {
        Self::after(Duration::from_secs(hours * 3600), value)
    }

    #[inline]
    pub fn after_hours_f32(hours: f32, value: T) -> Self {
        Self::after(Duration::from_secs_f32(hours * 3600.0), value)
    }

    #[inline]
    pub fn after_hours_f64(hours: f64, value: T) -> Self {
        Self::after(Duration::from_secs_f64(hours * 3600.0), value)
    }

    #[inline]
    pub fn after_days(days: u64, value: T) -> Self {
        Self::after(Duration::from_secs(days * 86400), value)
    }

    #[inline]
    pub fn after_days_f32(days: f32, value: T) -> Self {
        Self::after(Duration::from_secs_f32(days * 86400.0), value)
    }

    #[inline]
    pub fn after_days_f64(days: f64, value: T) -> Self {
        Self::after(Duration::from_secs_f64(days * 86400.0), value)
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        Instant::now() >= self.time
    }
}

impl<T> PartialEq<TimeKey<T>> for TimeKey<T> {
    fn eq(&self, other: &TimeKey<T>) -> bool {
        self.time == other.time
    }
}

impl<T> Eq for TimeKey<T> {}

impl<T> Ord for TimeKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl<T> PartialOrd<TimeKey<T>> for TimeKey<T> {
    fn partial_cmp(&self, other: &TimeKey<T>) -> Option<std::cmp::Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

impl<T> PartialEq<Instant> for TimeKey<T> {
    fn eq(&self, other: &Instant) -> bool {
        self.time.eq(other)
    }
}

impl<T> PartialOrd<Instant> for TimeKey<T> {
    fn partial_cmp(&self, other: &Instant) -> Option<std::cmp::Ordering> {
        other.partial_cmp(&self.time)
    }
}