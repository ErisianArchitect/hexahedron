use std::{marker::PhantomData, time::Duration};

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