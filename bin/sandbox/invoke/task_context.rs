use std::time::{Duration, Instant};

use super::{callback::{Callback, IntoCallback}, context::SharedState, scheduler::Scheduler, scheduler_context::SchedulerContext};

pub struct TaskContext<'a, Ctx: SchedulerContext> {
    pub time: Instant,
    pub shared: &'a mut Ctx,
    pub scheduler: &'a mut Scheduler<Ctx>,
}

impl<'a, Ctx: SchedulerContext> TaskContext<'a, Ctx> {

    pub fn at<I, F>(&mut self, time: Instant, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.scheduler.at(time, callback);
    }

    /// Schedules callback to be invoked immediately.
    #[inline]
    pub fn now<I, F>(&mut self, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.scheduler.now(callback);
    }

    /// Schedules callback to be invoked after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after<I, F>(&mut self, duration: Duration, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.scheduler.at(self.time + duration, callback);
    }

    /// Schedules callback to be invoked `micros` microseconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_micros<I, F>(&mut self, micros: u64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_micros(micros), callback);
    }

    /// Schedules callback to be invoked `millis` milliseconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_millis<I, F>(&mut self, millis: u64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_millis(millis), callback);
    }

    /// Schedules callback to be invoked `nanos` nanoseconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_nanos<I, F>(&mut self, nanos: u64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_nanos(nanos), callback);
    }

    /// Schedules callback to be invoked `secs` seconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_secs<I, F>(&mut self, secs: u64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs(secs), callback)
    }

    /// Schedules callback to be invoked `secs` seconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_secs_f32<I, F>(&mut self, secs: f32, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs_f32(secs), callback);
    }

    /// Schedules callback to be invoked `secs` seconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_secs_f64<I, F>(&mut self, secs: f64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs_f64(secs), callback);
    }

    /// Schedules callback to be invoked `mins` minutes after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_mins<I, F>(&mut self, mins: u64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs(mins * 60), callback);
    }

    /// Schedules callback to be invoked `mins` minuntes after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_mins_f32<I, F>(&mut self, mins: f32, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs_f32(mins * 60.0), callback);
    }

    /// Schedules callback to be invoked `mins` minutes after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_mins_f64<I, F>(&mut self, mins: f64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs_f64(mins * 60.0), callback);
    }

    /// Schedules callback to be invoked `hours` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_hours<I, F>(&mut self, hours: u64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs(hours * 3600), callback);
    }

    /// Schedules callback to be invoked `hours` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_hours_f32<I, F>(&mut self, hours: f32, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs_f32(hours * 3600.0), callback);
    }

    /// Schedules callback to be invoked `hours` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_hours_f64<I, F>(&mut self, hours: f64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs_f64(hours * 3600.0), callback);
    }

    /// Schedules callback to be invoked `days` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_days<I, F>(&mut self, days: u64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs(days * 86400), callback);
    }

    /// Schedules callback to be invoked `days` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_days_f32<I, F>(&mut self, days: f32, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs_f32(days * 86400.0), callback);
    }

    /// Schedules callback to be invoked `days` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_days_f64<I, F>(&mut self, days: f64, callback: F)
    where F: IntoCallback<Ctx, I> {
        self.after(Duration::from_secs_f64(days * 86400.0), callback);
    }
}