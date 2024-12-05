use std::{collections::{BTreeMap, BinaryHeap}, io::Write, marker::PhantomData, sync::Arc, time::{Duration, Instant}};
use paste::paste;
use super::context::SchedulerContext;
use super::time_key::*;

pub enum SchedulerResponse {
    Finish,
    RescheduleAfter(Duration),
    RescheduleAt(Instant),
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

impl<T: Into<SchedulerResponse>> From<Option<T>> for SchedulerResponse {
    fn from(value: Option<T>) -> Self {
        if let Some(value) = value {
            value.into()
        } else {
            SchedulerResponse::Finish
        }
    }
}

impl From<Instant> for SchedulerResponse {
    fn from(value: Instant) -> Self {
        SchedulerResponse::RescheduleAt(value)
    }
}

pub struct TaskContext<'a> {
    time: Instant,
    context: &'a mut SchedulerContext,
    scheduler: &'a mut Scheduler,
}

impl<'a> TaskContext<'a> {
    pub fn time(&self) -> Instant {
        self.time
    }

    pub fn elapsed(&self) -> Duration {
        self.time.elapsed()
    }

    pub fn scheduler(&'a mut self) -> &'a mut Scheduler {
        self.scheduler
    }

    pub fn context(&'a mut self) -> &'a mut SchedulerContext {
        self.context
    }

    pub fn after<F>(&mut self, duration: Duration, callback: F)
    where F: Callback {
        self.scheduler.schedule_heap.push(TimeKey::new(self.time + duration, Box::new(callback)));
    }

    #[inline]
    pub fn now<F>(&mut self, callback: F)
    where F: Callback {
        self.scheduler.schedule_heap.push(TimeKey::now(Box::new(callback)));
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

pub trait Callback: 'static {
    fn invoke(
        &mut self,
        // context: &mut SchedulerContext,
        // scheduler: &mut Scheduler,
        task_ctx: TaskContext<'_>,
    ) -> SchedulerResponse;
}

pub struct ContextInjector<Data, Args, Output, Context, F>
where
Data: 'static,
Context: 'static,
Args: 'static,
Output: 'static,
F: 'static,
Self: Callback {
    phantom: PhantomData<(Args, Output, Context)>,
    data: Data,
    callback: F,
}

impl<Data, Args, Output, Context, F> ContextInjector<Data, Args, Output, Context, F>
where
Data: 'static,
Args: 'static,
Output: 'static,
Context: 'static,
F: 'static,
Self: Callback {
    pub fn new<NArgs, NOutput, NContext, NF>(callback: NF) -> ContextInjector<(), NArgs, NOutput, NContext, NF>
    where
    // NArgs: 'static,
    // NOutput: 'static,
    ContextInjector<(), NArgs, NOutput, NContext, NF>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data: (),
            callback,
        }
    }

    pub fn with_data<NData, NArgs, NOutput, NContext, NF>(data: NData, callback: NF) -> ContextInjector<NData, NArgs, NOutput, NContext, NF>
    where
    NF: Fn() -> NOutput + 'static,
    ContextInjector<NData, NArgs, NOutput, NContext, NF>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data,
            callback,
        }
    }
}

impl<Args, R, Context, F> From<F> for ContextInjector<(), Args, R, Context, F>
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

pub fn inject<Args, Output, Context, F>(callback: F) -> ContextInjector<(), Args, Output, Context, F>
where
ContextInjector<(), Args, Output, Context, F>: Callback {
    ContextInjector {
        phantom: PhantomData,
        data: (),
        callback,
    }
}

pub fn inject_with<Data, Args, Output, Context, F>(data: Data, callback: F) -> ContextInjector<Data, Args, Output, Context, F>
where
ContextInjector<Data, Args, Output, Context, F>: Callback {
    ContextInjector {
        phantom: PhantomData,
        data,
        callback,
    }
}

impl<R: Into<SchedulerResponse>, F> Callback for F
where F: Fn() -> R + 'static {
    fn invoke(
            &mut self,
            context: TaskContext<'_>,
        ) -> SchedulerResponse {
        (self)().into()
    }
}

// impl<F> Callback for ContextInjector<(), (), (), F>
// where
// F: Fn() + 'static {
//     fn invoke(
//             &mut self,
//             context: &SchedulerContext,
//             scheduler: &mut Scheduler
//         ) -> SchedulerResponse {
//         (self.callback)();
//         SchedulerResponse::Finish
//     }
// }

// impl<Data0, Arg0, R, F> Callback for ContextInjector<(Data0,), (Arg0,), R, F>
// where
// R: Into<SchedulerResponse>,
// Data0: Send + Sync + 'static,
// Arg0: Send + Sync + 'static,
// F: Fn(
//     &mut Data0,
//     Arc<Arg0>
// ) -> R + 'static {
//     fn invoke(
//             &mut self,
//             context: &SchedulerContext,
//             scheduler: &mut Scheduler
//         ) -> SchedulerResponse {
//         let (
//             data0,
//         ) = &mut self.data;
//         (self.callback)(
//             data0,
//             context.get::<Arg0>().expect("Failed to get field from context."),
//         ).into()
//     }
// }

macro_rules! context_injector_impls {
    (@ctx_arg; Scheduler, $context:ident) => {
        $context.scheduler
    };
    (@ctx_arg; SchedulerContext, $context:ident) => {
        $context.context
    };
    (@ctx_arg; TaskContext, $context:ident) => {
        $context
    };
    (@ctx_type; Scheduler) => {
        &mut Scheduler
    };
    (@ctx_type; SchedulerContext) => {
        &mut SchedulerContext
    };
    (@ctx_type; TaskContext) => {
        TaskContext<'_>
    };
    (@right_context; ( $($data_type:ident),* ), ( $($arg_type:ident),* ), ($($ctx:ident),*)) => {
        paste!{
            impl<$($data_type,)* $($arg_type,)* R, F> Callback for ContextInjector<($($data_type,)*), ($($arg_type,)*), R, ( ($($data_type,)*), ($($arg_type,)*), ($(context_injector_impls!(@ctx_type; $ctx),)*) ), F>
            where
            R: Into<SchedulerResponse>,
            $(
                $data_type: 'static,
            )*
            $(
                $arg_type: Send + Sync + 'static,
            )*
            F: FnMut(
                $(
                    &mut $data_type,
                )*
                $(
                    Arc<$arg_type>,
                )*
                $(
                    context_injector_impls!(@ctx_type; $ctx),
                )*
            ) -> R + 'static {
                #[allow(non_snake_case)]
                fn invoke(
                    &mut self,
                    context: TaskContext<'_>,
                ) -> SchedulerResponse {
                    let (
                        $(
                            [<_ $data_type>],
                        )*
                    ) = &mut self.data;
                    (self.callback)(
                        $(
                            [<_ $data_type>],
                        )*
                        $(
                            context.context.get::<$arg_type>().expect(concat!("Failed to get ", stringify!($arg_type), " field.")),
                        )*
                        $(
                            context_injector_impls!(@ctx_arg; $ctx, context),
                        )*
                    ).into()
                }
            }
        }
    };
    (($($data_type:ident),*), ($($arg_type:ident),*)) => {
        context_injector_impls!{@right_context; ( $($data_type),* ), ( $($arg_type),* ), ()}
        context_injector_impls!{@right_context; ( $($data_type),* ), ( $($arg_type),* ), (TaskContext)}
    };
    ($([($($data_type:ident),*), ($($arg_type:ident),*)])+) => {
        $(
            context_injector_impls!(($($data_type),*), ($($arg_type),*));
        )+
    };
}

include!("injector_impls.rs");

// context_injector_impls!(
//     [(), ()]
//     [(), (T0)]
//     [(), (T0, T1)]
//     [(), (T0, T1, T2)]
//     [(), (T0, T1, T2, T3)]
//     [(), (T0, T1, T2, T3, T4)]
//     [(D0), ()]
//     [(D0), (T0)]
//     [(D0), (T0, T1)]
//     [(D0), (T0, T1, T2)]
//     [(D0), (T0, T1, T2, T3)]
//     [(D0), (T0, T1, T2, T3, T4)]
//     [(D0, D1), ()]
//     [(D0, D1), (T0)]
//     [(D0, D1), (T0, T1)]
//     [(D0, D1), (T0, T1, T2)]
//     [(D0, D1), (T0, T1, T2, T3)]
//     [(D0, D1), (T0, T1, T2, T3, T4)]
//     [(D0, D1), (T0, T1, T2, T3, T4, T5)]
//     [(D0, D1), (T0, T1, T2, T3, T4, T5, T6)]
//     [(D0, D1), (T0, T1, T2, T3, T4, T5, T6, T7)]
// );

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
        
    }
}

// impl<Data0, F> Callback for ContextInjector<(Data0,), (), (), F>
// where
// Data0: Send + Sync + 'static,
// F: Fn(
//     &mut Data0,
// ) + 'static {
//     fn invoke(
//             &mut self,
//             context: &SchedulerContext,
//             scheduler: &mut Scheduler
//         ) -> SchedulerResponse {
//         let (
//             data0,
//         ) = &mut self.data;
//         (self.callback)(
//             data0,
//         );
//         SchedulerResponse::Finish
//     }
// }

// impl<Arg0, R, F> Callback for ContextInjector<(), (Arg0,), R, F>
// where
// R: Into<SchedulerResponse>,
// Arg0: Send + Sync + 'static,
// F: Fn(Arc<Arg0>) -> R + 'static {
//     fn invoke(
//             &mut self,
//             context: &SchedulerContext,
//             scheduler: &mut Scheduler
//         ) -> SchedulerResponse {
//         (self.callback)(
//             context.get::<Arg0>().expect("Failed to get field from context."),
//         ).into()
//     }
// }

// impl<F> Callback for ContextInjector<(), (), SchedulerResponse, F>
// where
// F: Fn() -> SchedulerResponse + 'static {
//     fn invoke(
//             &mut self,
//             context: &SchedulerContext,
//             scheduler: &mut Scheduler
//         ) -> SchedulerResponse {
//         (self.callback)()
//     }
// }

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
    pub fn now<F>(&mut self, callback: F)
    where F: Callback {
        self.schedule_heap.push(TimeKey::now(Box::new(callback)));
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

    fn process_next(&mut self, context: &mut SchedulerContext) {
        let Some(TimeKey { time, mut value }) = self.schedule_heap.pop() else {
            panic!("No task in heap.");
        };
        let task_context = TaskContext {
            time,
            context,
            scheduler: self,
        };
        match value.invoke(task_context) {
            SchedulerResponse::Finish => (),
            SchedulerResponse::RescheduleAfter(duration) => {
                self.schedule_heap.push(TimeKey::new(time + duration, value));
            },
            SchedulerResponse::RescheduleAt(instant) => {
                self.schedule_heap.push(TimeKey::new(instant, value));
            },
        }
    }

    pub fn process_until(&mut self, instant: Instant, context: &mut SchedulerContext) {
        while let Some(TimeKey { time, value }) = self.schedule_heap.peek() {
            if instant < *time {
                break;
            }
            self.process_next(context);
        }
    }

    #[inline]
    pub fn process_until_now(&mut self, context: &mut SchedulerContext) {
        self.process_until(Instant::now(), context);
    }

    #[inline]
    fn next_task_time(&self) -> Option<Instant> {
        let Some(TimeKey { time, .. }) = self.schedule_heap.peek() else {
            return None;
        };
        Some(*time)
    }

    #[inline]
    pub fn process_blocking(&mut self, context: &mut SchedulerContext) {
        const ONE_MS: Duration = Duration::from_millis(1);
        while let Some(time) = self.next_task_time() {
            let now = Instant::now();
            if now < time {
                let diff = time - now;
                if diff.as_millis() > ONE_MS.as_millis() {
                    std::thread::sleep(diff - ONE_MS);
                }
                while Instant::now() < time {
                    std::hint::spin_loop();
                }
            }
            self.process_until_now(context);
        }
    }
    
}

pub use experiment::experiment;

pub trait SchedulerContextArg {

}

mod experiment {
    use super::*;
    use chrono::Timelike;
    use hexahedron::prelude::Increment;
    pub fn experiment() {
        let mut context = SchedulerContext::new();
        context.insert(vec![
            String::from("Hello, world!"),
            String::from("The quick brown fox jumps over the lazy dog."),
            String::from("This is a test."),
        ]);
        let mut scheduler = Scheduler::new();
        println!("Before schedule.");
        scheduler.now(inject(|context: TaskContext<'_>| {
            println!("Starting...");
            context.scheduler.after_secs(5, inject_with((0i32, ), |num: &mut i32, string: Arc<Vec<String>>, context: TaskContext<'_>| {
                let chron = chrono::Local::now();
                println!("Frame {:>2} {:>2} {:>25}", num.increment(), chron.second(), chron.timestamp_millis());
                std::io::stdout().flush().unwrap();
                if *num < 61 {
                    Some(Duration::from_secs(1) / 60)
                } else {
                    context.scheduler.after_secs(3, || {
                        let chron = chrono::Local::now();
                        println!("Finished! {}", chron.timestamp_millis());
                    });
                    None
                }
            }));
        }));
        scheduler.process_blocking(&mut context);
    }
}

/*
00: (data,)*, (args,)*
01: S, (data,)*, (args,)*
02: C, (data,)*, (args,)*
03: SC, (data,)*, (args,)*
04: CS, (data,)*, (args,)*
05: (data,)*, S, (args,)*
06: (data,)*, C, (args,)*
07: (data,)*, SC, (args,)*
08: (data,)*, CS, (args,)*
09: (data,)*, (args,)*, S
10: (data,)*, (args,)*, C
11: (data,)*, (args,)*, SC
12: (data,)*, (args,)*, CS
*/
