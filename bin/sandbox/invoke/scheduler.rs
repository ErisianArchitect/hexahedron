use std::{collections::{BTreeMap, BinaryHeap}, marker::PhantomData, sync::Arc, time::{Duration, Instant}};
use paste::paste;
use super::context::InvokeContext;
use super::time_key::*;

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

impl<Args, R, F> From<F> for ContextInjector<(), Args, R, F>
where
Self: Callback {
    fn from(value: F) -> Self {
        ContextInjector {
            phantom: PhantomData,
            data: (),
            callback: value,
        }
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

impl<Data0, Arg0, R, F> Callback for ContextInjector<(Data0,), (Arg0,), R, F>
where
R: Into<SchedulerResponse>,
Data0: Send + Sync + 'static,
Arg0: Send + Sync + 'static,
F: Fn(
    &mut Data0,
    Arc<Arg0>
) -> R + 'static {
    fn invoke(
            &mut self,
            context: &InvokeContext,
            scheduler: &mut Scheduler
        ) -> SchedulerResponse {
        let (
            data0,
        ) = &mut self.data;
        (self.callback)(
            data0,
            context.get::<Arg0>().expect("Failed to get field from context."),
        ).into()
    }
}

impl<Data0, F> Callback for ContextInjector<(Data0,), (), (), F>
where
Data0: Send + Sync + 'static,
F: Fn(
    &mut Data0,
) + 'static {
    fn invoke(
            &mut self,
            context: &InvokeContext,
            scheduler: &mut Scheduler
        ) -> SchedulerResponse {
        let (
            data0,
        ) = &mut self.data;
        (self.callback)(
            data0,
        );
        SchedulerResponse::Finish
    }
}

impl<Arg0, R, F> Callback for ContextInjector<(), (Arg0,), R, F>
where
R: Into<SchedulerResponse>,
Arg0: Send + Sync + 'static,
F: Fn(Arc<Arg0>) -> R + 'static {
    fn invoke(
            &mut self,
            context: &InvokeContext,
            scheduler: &mut Scheduler
        ) -> SchedulerResponse {
        (self.callback)(
            context.get::<Arg0>().expect("Failed to get field from context."),
        ).into()
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

#[cfg(test)]
mod testing_sandbox {

    use std::time::Duration;

    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;

    #[test]
    fn sandbox() {
        let mut context = InvokeContext::new();
        context.insert(vec![
            String::from("Hello, world!"),
            String::from("The quick brown fox jumps over the lazy dog."),
            String::from("This is a test."),
        ]);
        let mut n = 0i32;
        let mut injector = ContextInjector::from(|strings: Arc<Vec<String>>| {
            println!("Basic scheduled event.");
            Duration::from_secs(3)
        });
        let mut scheduler = Scheduler::new();
        scheduler.after_secs(3, inject_with((0i32, ), |num: &mut i32, string: Arc<Vec<String>>| {
            println!("Basic scheduled event.");
            Duration::from_secs(32)
        }));
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