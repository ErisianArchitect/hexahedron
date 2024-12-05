use std::{collections::{BTreeMap, BinaryHeap}, io::Write, marker::PhantomData, sync::Arc, time::{Duration, Instant}};
use paste::paste;
use super::context::InvokeContext;
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

pub trait Callback: 'static {
    fn invoke(
        &mut self,
        context: &InvokeContext,
        scheduler: &mut Scheduler
    ) -> SchedulerResponse;
}

pub struct ContextInjector<Data, Args, Output, const NEEDS_SCHEDULER: bool, F>
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

impl<Data, Args, Output, const NEEDS_SCHEDULER: bool, F> ContextInjector<Data, Args, Output, NEEDS_SCHEDULER, F>
where
Data: 'static,
Args: 'static,
Output: 'static,
F: 'static,
Self: Callback {
    pub fn new<NArgs, NOutput, const SCHED: bool, NF>(callback: NF) -> ContextInjector<(), NArgs, NOutput, SCHED, NF>
    where
    NArgs: 'static,
    NOutput: 'static,
    ContextInjector<(), NArgs, NOutput, SCHED, NF>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data: (),
            callback,
        }
    }

    pub fn with_data<NData, NArgs, NOutput, const SCHED: bool, NF>(data: NData, callback: NF) -> ContextInjector<NData, NArgs, NOutput, SCHED, NF>
    where
    NF: Fn() -> NOutput + 'static,
    ContextInjector<NData, NArgs, NOutput, SCHED, NF>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data,
            callback,
        }
    }
}

impl<Args, R, const SCHED: bool, F> From<F> for ContextInjector<(), Args, R, SCHED, F>
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

pub fn inject<Args, Output, const SCHED: bool, F>(callback: F) -> ContextInjector<(), Args, Output, SCHED, F>
where
ContextInjector<(), Args, Output, SCHED, F>: Callback {
    ContextInjector {
        phantom: PhantomData,
        data: (),
        callback,
    }
}

pub fn inject_with<Data, Args, Output, const SCHED: bool, F>(data: Data, callback: F) -> ContextInjector<Data, Args, Output, SCHED, F>
where
ContextInjector<Data, Args, Output, SCHED, F>: Callback {
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
            context: &InvokeContext,
            scheduler: &mut Scheduler
        ) -> SchedulerResponse {
        (self)().into()
    }
}

// impl<F> Callback for ContextInjector<(), (), (), F>
// where
// F: Fn() + 'static {
//     fn invoke(
//             &mut self,
//             context: &InvokeContext,
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
//             context: &InvokeContext,
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
    (($($data_type:ident),*), ($($arg_type:ident),*)) => {
        paste!{
            impl<$($data_type,)* $($arg_type,)* R, F> Callback for ContextInjector<($($data_type,)*), ($($arg_type,)*), R, true, F>
            where
            R: Into<SchedulerResponse>,
            $(
                $data_type: 'static,
            )*
            $(
                $arg_type: Send + Sync + 'static,
            )*
            F: Fn(
                $(
                    &mut $data_type,
                )*
                $(
                    Arc<$arg_type>,
                )*
                &mut Scheduler,
            ) -> R + 'static {
                #[allow(non_snake_case)]
                fn invoke(
                    &mut self,
                    context: &InvokeContext,
                    scheduler: &mut Scheduler,
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
                            context.get::<$arg_type>().expect(concat!("Failed to get ", stringify!($arg_type), " field.")),
                        )*
                        scheduler,
                    ).into()
                }
            }
        }
        paste!{
            impl<$($data_type,)* $($arg_type,)* R, F> Callback for ContextInjector<($($data_type,)*), ($($arg_type,)*), R, false, F>
            where
            R: Into<SchedulerResponse>,
            $(
                $data_type: 'static,
            )*
            $(
                $arg_type: Send + Sync + 'static,
            )*
            F: Fn(
                $(
                    &mut $data_type,
                )*
                $(
                    Arc<$arg_type>,
                )*
            ) -> R + 'static {
                #[allow(non_snake_case)]
                fn invoke(
                    &mut self,
                    context: &InvokeContext,
                    scheduler: &mut Scheduler,
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
                            context.get::<$arg_type>().expect(concat!("Failed to get ", stringify!($arg_type), " field.")),
                        )*
                    ).into()
                }
            }
        }
    };
    ($([($($data_type:ident),*), ($($arg_type:ident),*)])+) => {
        $(
            context_injector_impls!(($($data_type),*), ($($arg_type),*));
        )+
    };
}

context_injector_impls!(
    [(), ()]
    [(), (T0)]
    [(), (T0, T1)]
    [(), (T0, T1, T2)]
    [(), (T0, T1, T2, T3)]
    [(), (T0, T1, T2, T3, T4)]
    [(D0), ()]
    [(D0), (T0)]
    [(D0), (T0, T1)]
    [(D0), (T0, T1, T2)]
    [(D0), (T0, T1, T2, T3)]
    [(D0), (T0, T1, T2, T3, T4)]
    [(D0, D1), ()]
    [(D0, D1), (T0)]
    [(D0, D1), (T0, T1)]
);

// impl<Data0, F> Callback for ContextInjector<(Data0,), (), (), F>
// where
// Data0: Send + Sync + 'static,
// F: Fn(
//     &mut Data0,
// ) + 'static {
//     fn invoke(
//             &mut self,
//             context: &InvokeContext,
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
//             context: &InvokeContext,
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
//             context: &InvokeContext,
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

    fn process_next(&mut self, context: &InvokeContext) {
        let Some(TimeKey { time, mut value }) = self.schedule_heap.pop() else {
            panic!("No task in heap.");
        };
        match value.invoke(context, self) {
            SchedulerResponse::Finish => (),
            SchedulerResponse::RescheduleAfter(duration) => {
                self.schedule_heap.push(TimeKey::new(time + duration, value));
            },
            SchedulerResponse::RescheduleAt(instant) => {
                self.schedule_heap.push(TimeKey::new(instant, value));
            },
        }
    }

    pub fn process_until(&mut self, instant: Instant, context: &InvokeContext) {
        while let Some(TimeKey { time, value }) = self.schedule_heap.peek() {
            if instant < *time {
                break;
            }
            self.process_next(context);
        }
    }

    #[inline]
    pub fn process_until_now(&mut self, context: &InvokeContext) {
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
    pub fn process_blocking(&mut self, context: &InvokeContext) {
        const TEN_MS: Duration = Duration::from_millis(10);
        while let Some(time) = self.next_task_time() {
            let now = Instant::now();
            if now < time {
                let diff = time - now;
                if diff.as_millis() > TEN_MS.as_millis() {
                    std::thread::sleep(diff - TEN_MS);
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

mod experiment {
    use super::*;
    use chrono::Timelike;
    use hexahedron::prelude::Increment;
    pub fn experiment() {
        let mut context = InvokeContext::new();
        context.insert(vec![
            String::from("Hello, world!"),
            String::from("The quick brown fox jumps over the lazy dog."),
            String::from("This is a test."),
        ]);
        let mut scheduler = Scheduler::new();
        scheduler.after_secs(3, inject_with((0i32, ), |num: &mut i32, string: Arc<Vec<String>>, scheduler: &mut Scheduler| {
            let chron = chrono::Local::now();
            println!("Frame {:>2} {} {:>25}", num.increment(), chron.second(), chron.timestamp_millis());
            std::io::stdout().flush().unwrap();
            if *num < 61 {
                Some(Duration::from_secs(1) / 60)
            } else {
                scheduler.after_secs(5, || {
                    println!("Finished!");
                });
                None
            }
        }));
        scheduler.process_blocking(&context);
    }
}
