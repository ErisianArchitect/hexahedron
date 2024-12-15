use std::{
    env::Args, marker::PhantomData, ops::{Deref, DerefMut}, rc::Rc, sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }, time::Duration
};

use super::{
    context_injector::ContextInjector, scheduler_context::*, task_context::TaskContext,
    task_response::TaskResponse,
    variadic_callback::VariadicCallback as VariadicCallbackTrait,
    // tuple_combine::TupleJoin,
    // variadic_callback::*,
};

pub trait Callback<Ctx>
where
    Self: 'static,
    Ctx: SchedulerContext,
{
    fn invoke(&mut self, task_ctx: TaskContext<'_, Ctx>) -> TaskResponse;
}

pub trait IntoCallback<Ctx: SchedulerContext, Marker>: 'static
where
    Self::Output: Callback<Ctx>,
{
    type Output;
    fn into_callback(self) -> Self::Output;
}

pub struct ContextInjectableMarker<Args, Ctx: SchedulerContext = ()>(PhantomData<(Args, Ctx)>);

impl<Args, ContextArg, F, Output, Ctx>
    IntoCallback<Ctx, ContextInjectableMarker<((), Args, ContextArg, F, Output), Ctx>> for F
where
    Ctx: SchedulerContext,
    ContextInjector<(), Args, ContextArg, F, Output, Ctx>: Callback<Ctx>,
{
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
    fn invoke(&mut self, task_ctx: TaskContext<'_, Ctx>) -> TaskResponse {
        task_ctx.scheduler.clear();
        TaskResponse::Finish
    }
}

// #[derive(Clone, Copy)]
// pub struct TriggerBuilder<E: Clone + Copy, U: Clone + Copy>(bool, E, U);

// impl TriggerBuilder<(), ()> {
//     #[inline]
//     pub const fn new(active: bool) -> Self {
//         Self(active, (), ())
//     }

//     #[inline]
//     pub const fn active() -> Self {
//         Self(true, (), ())
//     }

//     #[inline]
//     pub const fn inactive() -> Self {
//         Self(false, (), ())
//     }

//     #[inline]
//     pub const fn eval_ordering(self, ordering: Ordering) -> TriggerBuilder<Ordering, ()> {
//         TriggerBuilder(self.0, ordering, ())
//     }

//     #[inline]
//     pub const fn update_ordering(self, ordering: Ordering) -> TriggerBuilder<(), Ordering> {
//         TriggerBuilder(self.0, (), ordering)
//     }

//     #[inline]
//     pub const fn total_ordering(self, ordering: Ordering) -> TriggerBuilder<Ordering, Ordering> {
//         TriggerBuilder(self.0, ordering, ordering)
//     }
// }

// impl TriggerBuilder<Ordering, ()> {
//     #[inline]
//     pub const fn update_ordering(self, ordering: Ordering) -> TriggerBuilder<Ordering, Ordering> {
//         TriggerBuilder(self.0, self.1, ordering)
//     }
// }

// impl TriggerBuilder<(), Ordering> {
//     #[inline]
//     pub const fn eval_ordering(self, ordering: Ordering) -> TriggerBuilder<Ordering, Ordering> {
//         TriggerBuilder(self.0, ordering, self.2)
//     }
// }

// impl TriggerBuilder<Ordering, Ordering> {
//     #[inline]
//     pub fn build(self) -> Trigger {
//         Trigger::with_ordering(self.0, self.1, self.2)
//     }
// }

// Other Callback types:
// constantly
// when

// pub struct VariadicCallback<Ctx, Args, R, F>
// where
// Ctx: SchedulerContext,
// Args: ResolvableGroup<Ctx> {
//     phantom: PhantomData<(Ctx, Args, R)>,
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

// impl<Ctx, Args, R, F> Callback<Ctx> for VariadicCallback<Ctx, Args, R, F>
// where
// Ctx: SchedulerContext,
// Args: ResolvableGroup<Ctx>,
// R: Into<TaskResponse> + 'static,
// F: VariadicCallbackTrait<Args, R> {
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
//         self.callback.invoke(args).into()
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

// impl<Ctx, Args, R, F> IntoCallback<Ctx, VariadicCallback<Ctx, Args, R, F>> for F
// where
// Ctx: SchedulerContext,
// Args: ResolvableGroup<Ctx>,
// R: Into<TaskResponse> + 'static,
// // for<'a> (Args, (TaskContext<'a, Ctx>,)): TupleJoin,
// for<'a> F: VariadicCallbackTrait<Args, R> {
//     type Output = VariadicCallback<Ctx, Args, R, F>;
//     fn into_callback(self) -> Self::Output {
//         VariadicCallback::<Ctx, Args, R, F> {
//             phantom: PhantomData,
//             callback: self
//         }
//     }
// }
