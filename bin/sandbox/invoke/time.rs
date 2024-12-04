use std::time::{Instant, Duration};

#[inline]
pub fn now() -> Instant {
    Instant::now()
}

#[inline]
pub const fn micros(micros: u64) -> Duration {
    Duration::from_micros(micros)
}

#[inline]
pub const fn millis(millis: u64) -> Duration {
    Duration::from_millis(millis)
}

#[inline]
pub const fn nanos(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}

#[inline]
pub const fn secs(secs: u64) -> Duration {
    Duration::from_secs(secs)
}

#[inline]
pub fn secs_f32(secs: f32) -> Duration  {
    Duration::from_secs_f32(secs)
}

#[inline]
pub fn secs_f64(secs: f64) -> Duration  {
    Duration::from_secs_f64(secs)
}

#[inline]
pub const fn mins(mins: u64) -> Duration {
    Duration::from_secs(mins * 60)
}

#[inline]
pub fn mins_f32(mins: f32) -> Duration {
    Duration::from_secs_f32(mins * 60.0)
}

#[inline]
pub fn mins_f64(mins: f64) -> Duration {
    Duration::from_secs_f64(mins * 60.0)
}

#[inline]
pub const fn hours(hours: u64) -> Duration {
    Duration::from_secs(hours * 3600)
}

#[inline]
pub fn hours_f32(hours: f32) -> Duration {
    Duration::from_secs_f32(hours * 3600.0)
}

#[inline]
pub fn hours_f64(hours: f64) -> Duration {
    Duration::from_secs_f64(hours * 3600.0)
}

#[inline]
pub const fn days(days: u64) -> Duration {
    Duration::from_secs(days * 86400)
}

#[inline]
pub fn days_f32(days: f32) -> Duration {
    Duration::from_secs_f32(days * 86400.0)
}

#[inline]
pub fn days_f64(days: f64) -> Duration {
    Duration::from_secs_f64(days * 86400.0)
}

// After

#[inline]
pub fn after(duration: Duration) -> Instant {
    Instant::now() + duration
}

#[inline]
pub fn after_micros(micros: u64) -> Instant {
    after(Duration::from_micros(micros))
}

#[inline]
pub fn after_millis(millis: u64) -> Instant {
    after(Duration::from_millis(millis))
}

#[inline]
pub fn after_nanos(nanos: u64) -> Instant {
    after(Duration::from_nanos(nanos))
}

#[inline]
pub fn after_secs(secs: u64) -> Instant {
    after(Duration::from_secs(secs))
}

#[inline]
pub fn after_secs_f32(secs: f32) -> Instant {
    after(Duration::from_secs_f32(secs))
}

#[inline]
pub fn after_secs_f64(secs: f64) -> Instant {
    after(Duration::from_secs_f64(secs))
}

#[inline]
pub fn after_mins(mins: u64) -> Instant {
    after(Duration::from_secs(mins * 60))
}

#[inline]
pub fn after_mins_f32(mins: f32) -> Instant {
    after(Duration::from_secs_f32(mins * 60.0))
}

#[inline]
pub fn after_mins_f64(mins: f64) -> Instant {
    after(Duration::from_secs_f64(mins * 60.0))
}

#[inline]
pub fn after_hours(hours: u64) -> Instant {
    after(Duration::from_secs(hours * 3600))
}

#[inline]
pub fn after_hours_f32(hours: f32) -> Instant {
    after(Duration::from_secs_f32(hours * 3600.0))
}

#[inline]
pub fn after_hours_f64(hours: f64) -> Instant {
    after(Duration::from_secs_f64(hours * 3600.0))
}

#[inline]
pub fn after_days(days: u64) -> Instant {
    after(Duration::from_secs(days * 86400))
}

#[inline]
pub fn after_days_f32(days: f32) -> Instant {
    after(Duration::from_secs_f32(days * 86400.0))
}

#[inline]
pub fn after_days_f64(days: f64) -> Instant {
    after(Duration::from_secs_f64(days * 86400.0))
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

}