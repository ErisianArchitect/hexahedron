use std::{env::Args, marker::PhantomData, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use super::{
    context_injector::ContextInjector, scheduler_context::*, task_context::TaskContext, task_response::TaskResponse, tuple_combine::TupleJoin, variadic_callback::*
};

pub trait Callback<Ctx>
where
Self: 'static,
Ctx: SchedulerContext {
    #[inline]
    fn invoke(
        &mut self,
        task_ctx: TaskContext<'_, Ctx>,
    ) -> TaskResponse;
}

pub trait IntoCallback<Ctx: SchedulerContext, T>: 'static
where Self::Output: Callback<Ctx> {
    type Output;
    fn into_callback(self) -> Self::Output;
}

pub struct ContextInjectableMarker<Args, Ctx: SchedulerContext = ()>(PhantomData<(Args, Ctx)>);

impl<Args, ContextArg, F, Output, Ctx> IntoCallback<Ctx, ContextInjectableMarker<((), Args, ContextArg, F, Output), Ctx>> for F
where
Ctx: SchedulerContext,
ContextInjector<(), Args, ContextArg, F, Output, Ctx>: Callback<Ctx> {
    type Output = ContextInjector<(), Args, ContextArg, F, Output, Ctx>;
    fn into_callback(self) -> Self::Output {
        ContextInjector::<(), Args, ContextArg, F, Output, Ctx>::new(self)
    }
}

impl<Ctx: SchedulerContext, C: Callback<Ctx>> IntoCallback<Ctx, C> for C {
    type Output = C;
    fn into_callback(self) -> Self::Output {
        self
    }
}

// Used to clear the [Scheduler] of all tasks.
pub struct Clear;

impl<Ctx: SchedulerContext> Callback<Ctx> for Clear {
    fn invoke(
            &mut self,
            task_ctx: TaskContext<'_, Ctx>,
        ) -> TaskResponse {
        task_ctx.scheduler.clear();
        TaskResponse::Finish
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EveryTimeAnchor {
    /// Relative to the scheduled time of the task.
    Schedule,
    /// Relative to the time immediately after the task finishs.
    After,
    /// Relative to the time before the task begins.
    Before,
}

pub struct Every<Ctx, F>
where
Ctx: SchedulerContext,
F: Callback<Ctx> {
    phantom: PhantomData<Ctx>,
    duration: Duration,
    anchor: EveryTimeAnchor,
    callback: F,
}

pub fn every<T, Ctx, F>(duration: Duration, anchor: EveryTimeAnchor, callback: F) -> Every<Ctx, F::Output>
where
Ctx: SchedulerContext,
F: IntoCallback<Ctx, T> {
    Every {
        phantom: PhantomData,
        duration,
        anchor,
        callback: callback.into_callback(),
    }
}

impl<Ctx, F> Callback<Ctx> for Every<Ctx, F>
where
Ctx: SchedulerContext,
F: Callback<Ctx> {
    fn invoke(
            &mut self,
            task_ctx: TaskContext<'_, Ctx>,
        ) -> TaskResponse {
        let resp = (self.callback).invoke(task_ctx);
        match self.anchor {
            EveryTimeAnchor::Schedule => TaskResponse::AfterScheduled(self.duration),
            EveryTimeAnchor::After => TaskResponse::AfterTaskEnds(self.duration),
            EveryTimeAnchor::Before => TaskResponse::AfterTaskBegan(self.duration),
        }
    }
}

pub trait ConditionalPredicate<Args>: 'static {
    fn evaluate(&mut self, args: Args) -> bool;
}

impl<Args, F> ConditionalPredicate<Args> for F
where
F: VariadicCallbackMut<Args, bool> {
    fn evaluate(&mut self, args: Args) -> bool {
        self.call_mutable(args)
    }
}

#[derive(Debug, Clone)]
pub struct Trigger{
    trigger: Arc<AtomicBool>,
    eval_ordering: Ordering,
    update_ordering: Ordering,
}

impl Trigger {
    /// Uses Ordering::Relaxed by default.
    #[inline]
    pub fn new(active: bool) -> Self {
        Self::with_ordering(active, Ordering::Relaxed, Ordering::Relaxed)
    }

    #[inline]
    pub fn with_ordering(active: bool, eval_ordering: Ordering, update_ordering: Ordering) -> Self {
        Self {
            trigger: Arc::new(AtomicBool::new(active)),
            eval_ordering,
            update_ordering,
        }
    }

    #[inline]
    pub fn activate(&self) {
        self.trigger.store(true, self.update_ordering);
    }

    #[inline]
    pub fn deactivate(&self) {
        self.trigger.store(false, self.update_ordering);
    }

    #[inline]
    pub fn swap(&self, active: bool) -> bool {
        self.trigger.swap(active, self.update_ordering)
    }
}

impl ConditionalPredicate<()> for Trigger {
    fn evaluate(&mut self, args: ()) -> bool {
        self.trigger.load(self.eval_ordering)
    }
}

pub struct Conditional<Ctx, F, P, PredArgs>
where
Ctx: SchedulerContext,
F: Callback<Ctx>,
PredArgs: ResolvableGroup<Ctx>,
P: ConditionalPredicate<PredArgs> {
    phantom: PhantomData<(Ctx, PredArgs)>,
    callback: F,
    predicate: P,
}

pub fn conditional<T, Ctx, F, P, PredArgs>(predicate: P, callback: F) -> Conditional<Ctx, F::Output, P, PredArgs>
where
Ctx: SchedulerContext,
F: IntoCallback<Ctx, T>,
PredArgs: ResolvableGroup<Ctx>,
P: ConditionalPredicate<PredArgs> {
    Conditional {
        phantom: PhantomData,
        callback: callback.into_callback(),
        predicate: predicate,
    }
}

impl<Ctx, F, P, PredArgs> Callback<Ctx> for Conditional<Ctx, F, P, PredArgs>
where
Ctx: SchedulerContext,
F: Callback<Ctx>,
PredArgs: ResolvableGroup<Ctx>,
P: ConditionalPredicate<PredArgs> {
    fn invoke(
            &mut self,
            task_ctx: TaskContext<'_, Ctx>,
        ) -> TaskResponse {
        let pred_args = match PredArgs::group_resolve(task_ctx.shared) {
            Ok(args) => args,
            Err(ResolveError::Skip) => return TaskResponse::Finish,
            Err(ResolveError::NotFound(type_name)) => {
                panic!("Type not found: \"{type_name}\"");
            }
        };
        if self.predicate.evaluate(pred_args) {
            self.callback.invoke(task_ctx)
        } else {
            TaskResponse::Finish
        }
    }
}

// Other Callback types:
// constantly
// when

// pub struct VariadicCallback<Ctx, Args, ContextArg, R, F>
// where
// Ctx: SchedulerContext,
// Args: ResolvableGroup<Ctx>,
// ContextArg: 'static {
//     phantom: PhantomData<(Ctx, Args, ContextArg, R)>,
//     callback: F,
// }

// impl<Ctx, Args, R, F> Callback<Ctx> for VariadicCallback<Ctx, Args, TaskContext<'static, Ctx>, R, F>
// where
// Ctx: SchedulerContext,
// Args: ResolvableGroup<Ctx>,
// R: Into<TaskResponse> + 'static,
// for<'a> ((TaskContext<'a, Ctx>,), Args): TupleJoin,
// for<'a> F: VariadicCallbackMut<<((TaskContext<'a, Ctx>,), Args) as TupleJoin>::Joined, R> + 'static {
//     fn invoke(
//             &mut self,
//             task_ctx: TaskContext<'_, Ctx>,
//         ) -> TaskResponse {
//         let args = match Args::group_resolve(task_ctx.shared) {
//             Ok(args) => args,
//             Err(ResolveError::Skip) => return TaskResponse::Finish,
//             Err(ResolveError::NotFound(type_name)) => panic!("type {type_name} not found in context."),
//         };
//         // let ctx = ContextArg::resolve(task_ctx);
//         let args = ((task_ctx,), args).join();
//         self.callback.call_mutable(args).into()
//     }
// }



// impl<Ctx, Args, R, F> Callback<Ctx> for VariadicCallback<Ctx, Args, (), R, F>
// where
// Ctx: SchedulerContext,
// Args: ResolvableGroup<Ctx>,
// R: Into<TaskResponse> + 'static,
// F: VariadicCallbackMut<Args, R> {
//     fn invoke(
//             &mut self,
//             task_ctx: TaskContext<'_, Ctx>,
//         ) -> TaskResponse {
//         let args = match Args::group_resolve(task_ctx.shared) {
//             Ok(args) => args,
//             Err(ResolveError::Skip) => return TaskResponse::Finish,
//             Err(ResolveError::NotFound(type_name)) => panic!("type {type_name} not found in context."),
//         };
//         // let ctx = ContextArg::resolve(task_ctx);
//         // let args = (args, ()).join();
//         self.callback.call_mutable(args).into()
//     }
// }

// pub struct VariadicCallbackMarker<Args, Ctx: SchedulerContext = ()>(PhantomData<(Args, Ctx)>);

// impl<Ctx, Args, R, F> IntoCallback<Ctx, VariadicCallbackMarker<(Args, TaskContext<'static, Ctx>, F, R), Ctx>> for F
// where
// Ctx: SchedulerContext,
// Args: ResolvableGroup<Ctx>,
// R: Into<TaskResponse> + 'static,
// for<'a> ((TaskContext<'a, Ctx>,), Args): TupleJoin,
// for<'a> F: VariadicCallbackMut<<((TaskContext<'a, Ctx>,), Args) as TupleJoin>::Joined, R> {
//     type Output = VariadicCallback<Ctx, Args, TaskContext<'static, Ctx>, R, F>;
//     fn into_callback(self) -> Self::Output {
//         VariadicCallback::<Ctx, Args, TaskContext<'static, Ctx>, R, F> {
//             phantom: PhantomData,
//             callback: self
//         }
//     }
// }

// impl<Ctx, Args, R, F> IntoCallback<Ctx, VariadicCallback<Ctx, Args, (), R, F>> for F
// where
// Ctx: SchedulerContext,
// Args: ResolvableGroup<Ctx>,
// R: Into<TaskResponse> + 'static,
// // for<'a> (Args, (TaskContext<'a, Ctx>,)): TupleJoin,
// for<'a> F: VariadicCallbackMut<Args, R> {
//     type Output = VariadicCallback<Ctx, Args, (), R, F>;
//     fn into_callback(self) -> Self::Output {
//         VariadicCallback::<Ctx, Args, (), R, F> {
//             phantom: PhantomData,
//             callback: self
//         }
//     }
// }