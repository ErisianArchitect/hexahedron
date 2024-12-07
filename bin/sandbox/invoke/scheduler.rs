use std::{any::TypeId, collections::{BTreeMap, BinaryHeap}, io::Write, marker::PhantomData, sync::Arc, time::{Duration, Instant}};
use paste::paste;
use super::context::SharedState;
use super::time_key::*;
use super::task_context::TaskContext;
use super::task_response::TaskResponse;
use super::callback::Callback;

// context_type

#[derive(Default)]
pub struct Scheduler {
    pub(crate) schedule_heap: BinaryHeap<TimeKey>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            schedule_heap: BinaryHeap::new(),
        }
    }

    #[inline]
    pub fn at<I, F>(&mut self, time: Instant, callback: F)
    where F: BoxableCallback<I> {
        self.schedule_heap.push(TimeKey::new(time, callback.into_box()));
    }

    #[inline]
    pub fn after<I, F>(&mut self, duration: Duration, callback: F)
    where F: BoxableCallback<I> {
        self.at(Instant::now() + duration, callback);
    }

    #[inline]
    pub fn now<I, F>(&mut self, callback: F)
    where F: BoxableCallback<I> {
        self.at(Instant::now(), callback);
    }

    #[inline]
    pub fn after_micros<I, F>(&mut self, micros: u64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_micros(micros), callback);
    }

    #[inline]
    pub fn after_millis<I, F>(&mut self, millis: u64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_millis(millis), callback);
    }

    #[inline]
    pub fn after_nanos<I, F>(&mut self, nanos: u64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_nanos(nanos), callback);
    }

    #[inline]
    pub fn after_secs<I, F>(&mut self, secs: u64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs(secs), callback)
    }

    #[inline]
    pub fn after_secs_f32<I, F>(&mut self, secs: f32, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs_f32(secs), callback);
    }

    #[inline]
    pub fn after_secs_f64<I, F>(&mut self, secs: f64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs_f64(secs), callback);
    }

    #[inline]
    pub fn after_mins<I, F>(&mut self, mins: u64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs(mins * 60), callback);
    }

    #[inline]
    pub fn after_mins_f32<I, F>(&mut self, mins: f32, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs_f32(mins * 60.0), callback);
    }

    #[inline]
    pub fn after_mins_f64<I, F>(&mut self, mins: f64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs_f64(mins * 60.0), callback);
    }

    #[inline]
    pub fn after_hours<I, F>(&mut self, hours: u64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs(hours * 3600), callback);
    }

    #[inline]
    pub fn after_hours_f32<I, F>(&mut self, hours: f32, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs_f32(hours * 3600.0), callback);
    }

    #[inline]
    pub fn after_hours_f64<I, F>(&mut self, hours: f64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs_f64(hours * 3600.0), callback);
    }

    #[inline]
    pub fn after_days<I, F>(&mut self, days: u64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs(days * 86400), callback);
    }

    #[inline]
    pub fn after_days_f32<I, F>(&mut self, days: f32, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs_f32(days * 86400.0), callback);
    }

    #[inline]
    pub fn after_days_f64<I, F>(&mut self, days: f64, callback: F)
    where F: BoxableCallback<I> {
        self.after(Duration::from_secs_f64(days * 86400.0), callback);
    }

    pub(crate) fn process_next(&mut self, shared: &mut SharedState) {
        let Some(TimeKey { time, callback: mut value }) = self.schedule_heap.pop() else {
            panic!("No task in heap.");
        };
        let task_context = TaskContext {
            time,
            shared,
            scheduler: self,
        };
        match value.invoke(task_context) {
            TaskResponse::Finish => (),
            TaskResponse::After(duration) => {
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

    pub fn process_until(&mut self, instant: Instant, shared: &mut SharedState) {
        while let Some(TimeKey { time, callback: value }) = self.schedule_heap.peek() {
            if instant < *time {
                break;
            }
            self.process_next(shared);
        }
    }

    /// Process until current time.  
    /// Current time is not updated after each task is processed, so it may be late. Use `process_current()` for more precise timing.
    #[inline]
    pub fn process_until_now(&mut self, shared: &mut SharedState) {
        self.process_until(Instant::now(), shared);
    }

    /// Similar to `process_until_now()`, except this method uses the current
    /// time for each processing chunk rather than the same time for each chunk.  
    /// With `process_until_now()`, you may end up processing nodes late.
    #[inline]
    pub fn process_current(&mut self, context: &mut SharedState) {
        while let Some(TimeKey { time, callback: value }) = self.schedule_heap.peek() {
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
    pub fn time_until_next_task(&self) -> Option<Duration> {
        let Some(TimeKey { time, .. }) = self.schedule_heap.peek() else {
            return None;
        };
        Some(time.duration_since(Instant::now()))
    }

    /// Process tasks until there are no tasks remaining, blocking the current thread in the process.
    pub fn process_blocking(&mut self, context: &mut SharedState) {
        while let Some(time) = self.next_task_time() {
            spin_sleep::sleep_until(time);
            self.process_current(context);
        }
    }

    pub fn clear(&mut self) {
        self.schedule_heap.clear();
    }
}

// pub fn group<
//     TaskCtx,
//     Args0, Ctx0, F0, R0,
//     Args1, Ctx1, F1, R1,
// >((f0, f1): (F0, F1))
// where
// TaskCtx: TaskContextType,
// R0: Into<TaskResponse>,
// Ctx0: VariadicCallbackContext<TaskCtx>,
// F0: VariadicCallback<(), Args0, TaskCtx, Ctx0, R0>,
// R1: Into<TaskResponse>,
// Ctx1: VariadicCallbackContext<TaskCtx>,
// F0: VariadicCallback<(), Args1, TaskCtx, Ctx1, R1>,
// {
    
// }

pub trait BoxableCallback<T>: 'static
where Self::Output: Callback {
    type Output;
    fn into_box(self) -> Box<Self::Output>;
}

impl<Args, ContextArg, F, Output> BoxableCallback<(Args, ContextArg, F, Output)> for F
where
ContextInjector<(), Args, ContextArg, F, Output>: Callback {
    type Output = ContextInjector<(), Args, ContextArg, F, Output>;
    fn into_box(self) -> Box<Self::Output> {
        Box::new(ContextInjector::<(), Args, ContextArg, F, Output>::new(self))
    }
}

impl<C: Callback> BoxableCallback<C> for C {
    type Output = Self;
    fn into_box(self) -> Box<Self::Output> {
        Box::new(self)
    }
}

// It might look like this isn't being used, but it is. It's in the conject_injector_impls macro.
pub trait ContextArg {
    fn resolve(context: &SharedState) -> Self;
}

impl<T> ContextArg for Option<Arc<T>>
where T: Send + Sync + 'static {
    fn resolve(context: &SharedState) -> Self {
        context.get()
    }
}

impl<T> ContextArg for Arc<T>
where T: Send + Sync + 'static {
    fn resolve(context: &SharedState) -> Self {
        context.get().expect(format!("Failed to resolve argument of type \"{}\"", std::any::type_name::<Arc<T>>()).as_ref())
    }
}

pub struct ContextInjector<DataArgs, Args, ContextArg, F, Output>
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

impl<DataArgs, Args, ContextArg, F, Output> ContextInjector<DataArgs, Args, ContextArg, F, Output>
where
DataArgs: 'static,
Args: 'static,
Output: 'static,
ContextArg: 'static,
F: 'static,
Self: Callback {
    pub fn new<NArgs, NContextArg, NF, NOutput>(callback: NF) -> ContextInjector<(), NArgs, NContextArg, NF, NOutput>
    where
    // NArgs: 'static,
    // NOutput: 'static,
    ContextInjector<(), NArgs, NContextArg, NF, NOutput>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data: (),
            callback,
        }
    }

    pub fn with_data<NDataArgs, NArgs, NContextArg, NF, NOutput>(data: NDataArgs, callback: NF) -> ContextInjector<NDataArgs, NArgs, NContextArg, NF, NOutput>
    where
    NF: Fn() -> NOutput + 'static,
    ContextInjector<NDataArgs, NArgs, NContextArg, NF, NOutput>: Callback {
        ContextInjector {
            phantom: PhantomData,
            data,
            callback,
        }
    }
}

impl<Args, Context, F, R> From<F> for ContextInjector<(), Args, Context, F, R>
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

// /// Creates a [ContextInjector] callback suitable for passing into a [Scheduler].
// pub fn inject<Args, ContextArg, F, Output>(callback: F) -> ContextInjector<(), Args, ContextArg, F, Output>
// where
// ContextInjector<(), Args, ContextArg, F, Output>: Callback {
//     ContextInjector {
//         phantom: PhantomData,
//         data: (),
//         callback,
//     }
// }

/// Creates a [ContextInjector] callback with a data attachment suitable for passing into a [Scheduler].
pub fn with<DataArgs, Args, ContextArg, F, Output>(data: DataArgs, callback: F) -> ContextInjector<DataArgs, Args, ContextArg, F, Output>
where
ContextInjector<DataArgs, Args, ContextArg, F, Output>: Callback {
    ContextInjector {
        phantom: PhantomData,
        data,
        callback,
    }
}

/// Used to clear the [Scheduler] of all tasks.
pub struct Clear;

impl Callback for Clear {
    fn invoke(
            &mut self,
            task_ctx: TaskContext<'_>,
        ) -> TaskResponse {
        task_ctx.scheduler.clear();
        TaskResponse::Finish
    }
}

macro_rules! context_injector_impls {
    (@ctx_arg; WithContext; $context:ident) => {
        $context
    };
    (@ctx_type; WithContext) => {
        TaskContext<'_>
    };
    (@right_context; ( $($data_type:ident),* ), ( $($arg_type:ident),* ), ($($ctx:ident),*)) => {
        paste!{
            impl<$($data_type,)* $($arg_type,)* R, F> Callback for ContextInjector<($($data_type,)*), ($($arg_type,)*), ( ($($data_type,)*), ($($arg_type,)*), ($(context_injector_impls!(@ctx_type; $ctx),)*) ), F, R>
            where
            R: Into<TaskResponse>,
            $(
                $data_type: 'static,
            )*
            $(
                $arg_type: ContextArg,
            )*
            F: FnMut(
                $(
                    &mut $data_type,
                )*
                $(
                    $arg_type,
                )*
                $(
                    context_injector_impls!(@ctx_type; $ctx),
                )*
            ) -> R + 'static {
                #[allow(non_snake_case)]
                fn invoke(
                    &mut self,
                    context: TaskContext<'_>,
                ) -> TaskResponse {
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
                            $arg_type::resolve(context.shared),
                        )*
                        $(
                            context_injector_impls!(@ctx_arg; $ctx; context),
                        )*
                    ).into()
                }
            }
        }
    };
    (($($data_type:ident),*), ($($arg_type:ident),*)) => {
        context_injector_impls!{@right_context; ( $($data_type),* ), ( $($arg_type),* ), ()}
        context_injector_impls!{@right_context; ( $($data_type),* ), ( $($arg_type),* ), (WithContext)}
    };
    ($([($($data_type:ident),*), ($($arg_type:ident),*)])+) => {
        $(
            context_injector_impls!(($($data_type),*), ($($arg_type),*));
        )+
    };
}

include!("injector_impls.rs");

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
    }
}