use std::{marker::PhantomData, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use crate::invoke::{
    callback::{
        Callback,
        IntoCallback
    },
    scheduler_context::{
        ResolvableGroup,
        ResolveError,
        SchedulerContext
    },
    task_context::TaskContext,
    task_response::TaskResponse,
    variadic_callback::VariadicCallback
};

pub trait ConditionalPredicate<Args>: 'static {
    fn evaluate(&mut self, args: Args) -> bool;
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
            _: &mut (),
        ) -> TaskResponse {
        let pred_args = match PredArgs::group_resolve(task_ctx.shared) {
            Ok(args) => args,
            Err(ResolveError::Skip) => return TaskResponse::Continue,
            Err(ResolveError::NotFound(type_name)) => {
                panic!("Type not found: \"{type_name}\"");
            }
        };
        if self.predicate.evaluate(pred_args) {
            self.callback.invoke(task_ctx, &mut ())
        } else {
            TaskResponse::Continue
        }
    }
}

impl<Args, F> ConditionalPredicate<Args> for F
where
F: VariadicCallback<Args, bool> {
    fn evaluate(&mut self, args: Args) -> bool {
        self.invoke(args)
    }
}

#[derive(Debug, Clone)]
pub struct Trigger {
    trigger: Arc<AtomicBool>,
    eval_ordering: Ordering,
    update_ordering: Ordering,
}

impl Trigger {

    /// Uses [Ordering::Relaxed] by default.
    #[inline]
    pub fn new(active: bool) -> Self {
        Self::with_ordering(active, Ordering::Relaxed, Ordering::Relaxed)
    }

    #[inline]
    pub fn with_ordering(active: bool, eval_ordering: Ordering, update_ordering: Ordering) -> Self {
        Self::from_parts(Arc::new(AtomicBool::new(active)), eval_ordering, update_ordering)
    }

    #[inline]
    pub fn from_parts(trigger: Arc<AtomicBool>, eval_ordering: Ordering, update_ordering: Ordering) -> Self {
        Self {
            trigger,
            eval_ordering,
            update_ordering
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

    /// Clones the inner `Arc<AtomicBool>`.
    #[inline]
    pub fn clone_inner(&self) -> Arc<AtomicBool> {
        self.trigger.clone()
    }
}

impl ConditionalPredicate<()> for Trigger {
    fn evaluate(&mut self, args: ()) -> bool {
        self.trigger.load(self.eval_ordering)
    }
}