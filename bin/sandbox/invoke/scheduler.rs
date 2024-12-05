use std::{collections::{BTreeMap, BinaryHeap}, io::Write, marker::PhantomData, sync::Arc, time::{Duration, Instant}};
use paste::paste;
use super::context::Context;
use super::time_key::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchedulerResponse {
    #[default]
    Finish,
    Immediate,
    After(Duration),
    At(Instant),
}

impl From<()> for SchedulerResponse {
    fn from(value: ()) -> Self {
        SchedulerResponse::Finish
    }
}

impl From<Duration> for SchedulerResponse {
    fn from(value: Duration) -> Self {
        SchedulerResponse::After(value)
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
        SchedulerResponse::At(value)
    }
}

pub struct TaskContext<'a> {
    pub time: Instant,
    pub context: &'a mut Context,
    pub scheduler: &'a mut Scheduler,
}

impl<'a> TaskContext<'a> {

    pub fn at<F>(&mut self, time: Instant, callback: F)
    where F: Callback {
        self.scheduler.at(time, callback);
    }

    /// Schedules callback to be invoked immediately.
    #[inline]
    pub fn now<F>(&mut self, callback: F)
    where F: Callback {
        self.scheduler.schedule_heap.push(TimeKey::now(Box::new(callback)));
    }

    /// Schedules callback to be invoked after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after<F>(&mut self, duration: Duration, callback: F)
    where F: Callback {
        self.scheduler.schedule_heap.push(TimeKey::new(self.time + duration, Box::new(callback)));
    }

    /// Schedules callback to be invoked `micros` microseconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_micros<F>(&mut self, micros: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_micros(micros), callback);
    }

    /// Schedules callback to be invoked `millis` milliseconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_millis<F>(&mut self, millis: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_millis(millis), callback);
    }

    /// Schedules callback to be invoked `nanos` nanoseconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_nanos<F>(&mut self, nanos: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_nanos(nanos), callback);
    }

    /// Schedules callback to be invoked `secs` seconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_secs<F>(&mut self, secs: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs(secs), callback)
    }

    /// Schedules callback to be invoked `secs` seconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_secs_f32<F>(&mut self, secs: f32, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f32(secs), callback);
    }

    /// Schedules callback to be invoked `secs` seconds after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_secs_f64<F>(&mut self, secs: f64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f64(secs), callback);
    }

    /// Schedules callback to be invoked `mins` minutes after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_mins<F>(&mut self, mins: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs(mins * 60), callback);
    }

    /// Schedules callback to be invoked `mins` minuntes after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_mins_f32<F>(&mut self, mins: f32, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f32(mins * 60.0), callback);
    }

    /// Schedules callback to be invoked `mins` minutes after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_mins_f64<F>(&mut self, mins: f64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f64(mins * 60.0), callback);
    }

    /// Schedules callback to be invoked `hours` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_hours<F>(&mut self, hours: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs(hours * 3600), callback);
    }

    /// Schedules callback to be invoked `hours` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_hours_f32<F>(&mut self, hours: f32, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f32(hours * 3600.0), callback);
    }

    /// Schedules callback to be invoked `hours` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_hours_f64<F>(&mut self, hours: f64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f64(hours * 3600.0), callback);
    }

    /// Schedules callback to be invoked `days` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_days<F>(&mut self, days: u64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs(days * 86400), callback);
    }

    /// Schedules callback to be invoked `days` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_days_f32<F>(&mut self, days: f32, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f32(days * 86400.0), callback);
    }

    /// Schedules callback to be invoked `days` after current task's scheduled time.  
    /// For more precise timing, schedule it with the scheduler directly.
    #[inline]
    pub fn after_days_f64<F>(&mut self, days: f64, callback: F)
    where F: Callback {
        self.after(Duration::from_secs_f64(days * 86400.0), callback);
    }
}

pub trait Callback: 'static {
    fn invoke(
        &mut self,
        task_ctx: TaskContext<'_>,
    ) -> SchedulerResponse;
}

pub struct ContextInjector<DataArgs, Args, Output, ContextArg, F>
where
DataArgs: 'static,
ContextArg: 'static,
Args: 'static,
Output: 'static,
F: 'static,
Self: Callback {
    phantom: PhantomData<(Args, Output, ContextArg)>,
    data: DataArgs,
    callback: F,
}

impl<DataArgs, Args, Output, ContextArg, F> ContextInjector<DataArgs, Args, Output, ContextArg, F>
where
DataArgs: 'static,
Args: 'static,
Output: 'static,
ContextArg: 'static,
F: 'static,
Self: Callback {
    pub fn new<NArgs, NOutput, NContextArg, NF>(callback: NF) -> ContextInjector<(), NArgs, NOutput, NContextArg, NF>
    where
    // NArgs: 'static,
    // NOutput: 'static,
    ContextInjector<(), NArgs, NOutput, NContextArg, NF>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data: (),
            callback,
        }
    }

    pub fn with_data<NDataArgs, NArgs, NOutput, NContextArg, NF>(data: NDataArgs, callback: NF) -> ContextInjector<NDataArgs, NArgs, NOutput, NContextArg, NF>
    where
    NF: Fn() -> NOutput + 'static,
    ContextInjector<NDataArgs, NArgs, NOutput, NContextArg, NF>: Callback {
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

/// Creates a [ContextInjector] callback suitable for passing into a [Scheduler].
pub fn inject<Args, Output, ContextArg, F>(callback: F) -> ContextInjector<(), Args, Output, ContextArg, F>
where
ContextInjector<(), Args, Output, ContextArg, F>: Callback {
    ContextInjector {
        phantom: PhantomData,
        data: (),
        callback,
    }
}

/// Creates a [ContextInjector] callback with a data attachment suitable for passing into a [Scheduler].
pub fn inject_with<DataArgs, Args, Output, ContextArg, F>(data: DataArgs, callback: F) -> ContextInjector<DataArgs, Args, Output, ContextArg, F>
where
ContextInjector<DataArgs, Args, Output, ContextArg, F>: Callback {
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
    pub fn at<F>(&mut self, time: Instant, callback: F)
    where F: Callback {
        self.schedule_heap.push(TimeKey::new(time, Box::new(callback)));
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

    fn process_next(&mut self, context: &mut Context) {
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
            SchedulerResponse::After(duration) => {
                self.schedule_heap.push(TimeKey::new(time + duration, value));
            },
            SchedulerResponse::At(instant) => {
                self.schedule_heap.push(TimeKey::new(instant, value));
            },
            SchedulerResponse::Immediate => {
                self.schedule_heap.push(TimeKey::now(value));
            }
        }
    }

    pub fn process_until(&mut self, instant: Instant, context: &mut Context) {
        while let Some(TimeKey { time, value }) = self.schedule_heap.peek() {
            if instant < *time {
                break;
            }
            self.process_next(context);
        }
    }

    /// Process until current time.  
    /// Current time is not updated after each task is processed, so it may be late. Use `process_current()` for more precise timing.
    #[inline]
    pub fn process_until_now(&mut self, context: &mut Context) {
        self.process_until(Instant::now(), context);
    }

    /// Similar to `process_until_now()`, except this method uses the current
    /// time for each processing chunk rather than the same time for each chunk.  
    /// With `process_until_now()`, you may end up processing nodes late.
    #[inline]
    pub fn process_current(&mut self, context: &mut Context) {
        while let Some(TimeKey { time, value }) = self.schedule_heap.peek() {
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

    /// Process tasks until there are no tasks remaining.
    pub fn process_blocking(&mut self, context: &mut Context) {
        const ONE_MS: Duration = Duration::from_millis(1);
        while let Some(time) = self.next_task_time() {
            if Instant::now() < time {
                let diff = time - Instant::now();
                spin_sleep::sleep(diff);
                // if diff > ONE_MS {
                //     spin_sleep::sleep(diff - ONE_MS);
                // }
                // while Instant::now() < time {
                //     std::hint::spin_loop();
                // }
            }
            self.process_current(context);
        }
    }
    
}
