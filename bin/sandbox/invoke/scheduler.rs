use std::{collections::{BTreeMap, BinaryHeap}, marker::PhantomData, sync::Arc, time::{Duration, Instant}};

use super::context::InvokeContext;

pub enum SchedulerResponse {
    Finish,
    RescheduleAfter(Duration),
}

impl From<()> for SchedulerResponse {
    fn from(value: ()) -> Self {
        SchedulerResponse::Finish
    }
}

impl From<Duration> for SchedulerResponse {
    fn from(value: Duration) -> Self {
        SchedulerResponse::RescheduleAfter(value)
    }
}

pub trait Callback: 'static {
    fn invoke(
        &mut self,
        context: &InvokeContext,
        scheduler: &mut Scheduler
    ) -> SchedulerResponse;
}

pub struct ContextInjector<Data, Args, Output, F>
where
Data: 'static,
Args: 'static,
Output: 'static,
F: 'static,
Self: Callback {
    phantom: PhantomData<(Args, Output)>,
    data: Data,
    callback: F,
}

pub fn inject<Args, Output, F>(callback: F) -> ContextInjector<(), Args, Output, F>
where
Args: 'static,
Output: 'static,
F: 'static,
ContextInjector<(), Args, Output, F>: Callback {
    ContextInjector {
        phantom: PhantomData,
        data: (),
        callback,
    }
}

pub fn inject_with<Data, Args, Output, F>(data: Data, callback: F) -> ContextInjector<Data, Args, Output, F>
where
ContextInjector<Data, Args, Output, F>: Callback {
    ContextInjector {
        phantom: PhantomData,
        data,
        callback,
    }
}

impl<Data, Args, Output, F> ContextInjector<Data, Args, Output, F>
where
Data: 'static,
Args: 'static,
Output: 'static,
F: 'static,
Self: Callback {
    pub fn new<NArgs, NOutput, NF>(callback: NF) -> ContextInjector<(), NArgs, NOutput, NF>
    where
    NArgs: 'static,
    NOutput: 'static,
    ContextInjector<(), NArgs, NOutput, NF>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data: (),
            callback,
        }
    }

    pub fn with_data<NData, NArgs, NOutput, NF>(data: NData, callback: NF) -> ContextInjector<NData, NArgs, NOutput, NF>
    where
    NF: Fn() -> NOutput + 'static,
    ContextInjector<NData, NArgs, NOutput, NF>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data,
            callback,
        }
    }
}

impl<F, Output> From<F> for ContextInjector<(), (), Output, F>
where
Output: 'static,
F: Fn() -> Output + 'static,
Self: Callback {
    fn from(value: F) -> Self {
        ContextInjector {
            phantom: PhantomData,
            data: (),
            callback: value,
        }
    }
}

impl<R: Into<SchedulerResponse>, F> Callback for F
where F: Fn() -> R + 'static {
    fn invoke(
            &mut self,
            context: &InvokeContext,
            scheduler: &mut Scheduler
        ) -> SchedulerResponse {
        (self)().into()
    }
}

impl<F> Callback for ContextInjector<(), (), (), F>
where
F: Fn() + 'static {
    fn invoke(
            &mut self,
            context: &InvokeContext,
            scheduler: &mut Scheduler
        ) -> SchedulerResponse {
        (self.callback)();
        SchedulerResponse::Finish
    }
}

impl<F> Callback for ContextInjector<(), (), SchedulerResponse, F>
where
F: Fn() -> SchedulerResponse + 'static {
    fn invoke(
            &mut self,
            context: &InvokeContext,
            scheduler: &mut Scheduler
        ) -> SchedulerResponse {
        (self.callback)()
    }
}

#[derive(Default)]
pub struct Scheduler {
    schedule_heap: BinaryHeap<TimeKey<Box<dyn Callback>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            schedule_heap: BinaryHeap::new(),
        }
    }

    #[inline]
    pub fn after<F>(&mut self, duration: Duration, callback: F)
    where F: Callback {
        self.schedule_heap.push(TimeKey::after(duration, Box::new(callback)));
    }

    #[inline]
    pub fn after_micros<F>(&mut self, micros: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_micros(micros), callback);
    }

    #[inline]
    pub fn after_millis<F>(&mut self, millis: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_millis(millis), callback);
    }

    #[inline]
    pub fn after_nanos<F>(&mut self, nanos: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_nanos(nanos), callback);
    }

    #[inline]
    pub fn after_secs<F>(&mut self, secs: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs(secs), callback)
    }

    #[inline]
    pub fn after_secs_f32<F>(&mut self, secs: f32, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f32(secs), callback);
    }

    #[inline]
    pub fn after_secs_f64<F>(&mut self, secs: f64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f64(secs), callback);
    }

    #[inline]
    pub fn after_mins<F>(&mut self, mins: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs(mins * 60), callback);
    }

    #[inline]
    pub fn after_mins_f32<F>(&mut self, mins: f32, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f32(mins * 60.0), callback);
    }

    #[inline]
    pub fn after_mins_f64<F>(&mut self, mins: f64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f64(mins * 60.0), callback);
    }

    #[inline]
    pub fn after_hours<F>(&mut self, hours: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs(hours * 3600), callback);
    }

    #[inline]
    pub fn after_hours_f32<F>(&mut self, hours: f32, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f32(hours * 3600.0), callback);
    }

    #[inline]
    pub fn after_hours_f64<F>(&mut self, hours: f64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f64(hours * 3600.0), callback);
    }

    #[inline]
    pub fn after_days<F>(&mut self, days: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs(days * 86400), callback);
    }

    #[inline]
    pub fn after_days_f32<F>(&mut self, days: f32, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f32(days * 86400.0), callback);
    }

    #[inline]
    pub fn after_days_f64<F>(&mut self, days: f64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f64(days * 86400.0), callback);
    }
    
}

// TODO: I would prefer to use chrono for time.
struct TimeKey<T> {
    time: Instant,
    value: T,
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
        Instant::now() <= self.time
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

#[cfg(test)]
mod testing_sandbox {

    use std::time::Duration;

    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;

    #[test]
    fn sandbox() {
        let mut context = InvokeContext::new();
        let mut n = 0i32;
        let mut injector = inject(|| {
            println!("Basic scheduled event.");
            SchedulerResponse::Finish
        });
        let mut scheduler = Scheduler::new();
        scheduler.after_secs(3, || {
            println!("Basic scheduled event.");
            // Duration::from_secs(32)
            SchedulerResponse::Finish
        });
        // scheduler.after_secs(3, inject(|| {

        // }));
        injector.invoke(&context, &mut scheduler);
        injector.invoke(&context, &mut scheduler);
        use std::cmp::Reverse;
        let mut heap = BinaryHeap::new();
        heap.push(TimeKey::after_days_f64(0.10, "This is first maybe?"));
        heap.push(TimeKey::after_days_f64(1.0,  "Hello, world!"));

        if let Some(TimeKey { time, value }) = heap.peek() {
            println!("value: {value}");
            heap.pop();
        }
    }
}