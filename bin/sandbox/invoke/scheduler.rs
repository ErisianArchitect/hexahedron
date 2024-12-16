use std::{any::TypeId, collections::{BTreeMap, BinaryHeap}, io::Write, marker::PhantomData, sync::Arc, time::{Duration, Instant}};
use paste::paste;
use super::{context::SharedState, scheduler_context::*};
use super::time_key::*;
use super::task_context::TaskContext;
use super::task_response::TaskResponse;
use super::callback::{Callback, *};

#[derive(Default)]
pub struct Scheduler<Ctx: SchedulerContext> {
    // phantom: PhantomData<(Ctx)>,
    pub(crate) schedule_heap: BinaryHeap<TimeKey<Ctx>>,
}

impl<Ctx: SchedulerContext> Scheduler<Ctx> {
    pub fn new() -> Self {
        Self {
            // phantom: PhantomData,
            schedule_heap: BinaryHeap::new(),
        }
    }

    #[inline]
    pub fn at<M, F>(&mut self, time: Instant, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.schedule_heap.push(TimeKey::new(time, Box::new(callback.into_callback())));
    }

    #[inline]
    pub fn after<M, F>(&mut self, duration: Duration, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.at(Instant::now() + duration, callback);
    }

    #[inline]
    pub fn now<M, F>(&mut self, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.at(Instant::now(), callback);
    }

    #[inline]
    pub fn after_micros<M, F>(&mut self, micros: u64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_micros(micros), callback);
    }

    #[inline]
    pub fn after_millis<M, F>(&mut self, millis: u64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_millis(millis), callback);
    }

    #[inline]
    pub fn after_nanos<M, F>(&mut self, nanos: u64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_nanos(nanos), callback);
    }

    #[inline]
    pub fn after_secs<M, F>(&mut self, secs: u64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs(secs), callback)
    }

    #[inline]
    pub fn after_secs_f32<M, F>(&mut self, secs: f32, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs_f32(secs), callback);
    }

    #[inline]
    pub fn after_secs_f64<M, F>(&mut self, secs: f64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs_f64(secs), callback);
    }

    #[inline]
    pub fn after_mins<M, F>(&mut self, mins: u64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs(mins * 60), callback);
    }

    #[inline]
    pub fn after_mins_f32<M, F>(&mut self, mins: f32, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs_f32(mins * 60.0), callback);
    }

    #[inline]
    pub fn after_mins_f64<M, F>(&mut self, mins: f64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs_f64(mins * 60.0), callback);
    }

    #[inline]
    pub fn after_hours<M, F>(&mut self, hours: u64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs(hours * 3600), callback);
    }

    #[inline]
    pub fn after_hours_f32<M, F>(&mut self, hours: f32, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs_f32(hours * 3600.0), callback);
    }

    #[inline]
    pub fn after_hours_f64<M, F>(&mut self, hours: f64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs_f64(hours * 3600.0), callback);
    }

    #[inline]
    pub fn after_days<M, F>(&mut self, days: u64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs(days * 86400), callback);
    }

    #[inline]
    pub fn after_days_f32<M, F>(&mut self, days: f32, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs_f32(days * 86400.0), callback);
    }

    #[inline]
    pub fn after_days_f64<M, F>(&mut self, days: f64, callback: F)
    where F: IntoCallback<Ctx, M> {
        self.after(Duration::from_secs_f64(days * 86400.0), callback);
    }

    pub(crate) fn process_next(&mut self, shared: &mut Ctx) {
        let Some(TimeKey { time, callback: mut value }) = self.schedule_heap.pop() else {
            panic!("No task in heap.");
        };
        let task_context = TaskContext {
            time,
            shared,
            scheduler: self,
        };
        let start_time = Instant::now();
        match value.invoke(task_context, &mut ()) {
            TaskResponse::Finish | TaskResponse::Continue => (),
            TaskResponse::AfterTaskBegan(duration) => {
                self.schedule_heap.push(TimeKey::new(start_time + duration, value));
            },
            TaskResponse::AfterTaskEnds(duration) => {
                self.schedule_heap.push(TimeKey::new(Instant::now() + duration, value));
            },
            TaskResponse::AfterScheduled(duration) => {
                self.schedule_heap.push(TimeKey::new(time + duration, value));
            },
            TaskResponse::At(instant) => {
                self.schedule_heap.push(TimeKey::new(instant, value));
            },
            TaskResponse::Immediate => {
                self.schedule_heap.push(TimeKey::new(Instant::now(), value));
            }
        }
    }

    /// Processes tasks that come before `deadline`.
    #[inline]
    pub fn process_until(&mut self, deadline: Instant, shared: &mut Ctx) {
        while let Some(TimeKey { time, .. }) = self.schedule_heap.peek() {
            if deadline < *time {
                break;
            }
            self.process_next(shared);
        }
    }

    /// Process until current time.  
    /// Current time is not updated after each task is processed, so it may be late. Use `process_current()` for more precise timing.
    #[inline]
    pub fn process_until_now(&mut self, shared: &mut Ctx) {
        self.process_until(Instant::now(), shared);
    }

    /// Similar to `process_until_now()`, except this method uses the current
    /// time for each processing chunk rather than the same time for each chunk.  
    /// With `process_until_now()`, you may end up processing nodes late.
    #[inline]
    pub fn process_current(&mut self, context: &mut Ctx) {
        while let Some(TimeKey { time, .. }) = self.schedule_heap.peek() {
            if Instant::now() < *time {
                break;
            }
            self.process_next(context);
        }
    }

    #[inline]
    pub fn next_task_time(&self) -> Option<Instant> {
        let Some(TimeKey { time, .. }) = self.schedule_heap.peek() else {
            return None;
        };
        Some(*time)
    }

    #[inline]
    pub fn duration_until_next_task(&self) -> Option<Duration> {
        let Some(TimeKey { time, .. }) = self.schedule_heap.peek() else {
            return None;
        };
        Some(time.duration_since(Instant::now()))
    }

    /// Process tasks until there are no tasks remaining, blocking the current thread in the process.
    #[inline]
    pub fn process_blocking(&mut self, context: &mut Ctx) {
        while let Some(next_task_time) = self.next_task_time() {
            spin_sleep::sleep_until(next_task_time);
            self.process_current(context);
        }
    }

    #[inline]
    pub fn process_blocking_for(&mut self, duration: Duration, wait_until_deadline: bool, context: &mut Ctx) {
        self.process_blocking_until(Instant::now() + duration, wait_until_deadline, context);
    }

    pub fn process_blocking_until(&mut self, deadline: Instant, wait_until_deadline: bool, context: &mut Ctx) {
        while let Some(next_task_time) = self.next_task_time() {
            // If the next task time is after the deadline, return.
            if next_task_time > deadline {
                // If requested, wait until the deadline before returning.
                if wait_until_deadline {
                    spin_sleep::sleep_until(deadline);
                }
                return;
            }
            spin_sleep::sleep_until(next_task_time);
            self.process_current(context);
        }
    }

    pub fn clear(&mut self) {
        self.schedule_heap.clear();
    }
}

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
    }
}