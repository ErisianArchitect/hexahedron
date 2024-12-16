use std::{marker::PhantomData, time::Duration};

use crate::invoke::{callback::{Callback, IntoCallback}, scheduler_context::SchedulerContext, task_context::TaskContext, task_response::TaskResponse};



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
    F: Callback<Ctx>,
{
    phantom: PhantomData<Ctx>,
    duration: Duration,
    anchor: EveryTimeAnchor,
    callback: F,
}

pub fn every<T, Ctx, F>(
    duration: Duration,
    anchor: EveryTimeAnchor,
    callback: F,
) -> Every<Ctx, F::Output>
where
    Ctx: SchedulerContext,
    F: IntoCallback<Ctx, T>,
{
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
    F: Callback<Ctx>,
{
    fn invoke(&mut self, task_ctx: TaskContext<'_, Ctx>, _: &mut ()) -> TaskResponse {
        let resp = (self.callback).invoke(task_ctx, &mut ());
        if matches!(resp, TaskResponse::Finish) {
            return TaskResponse::Finish;
        }
        match self.anchor {
            EveryTimeAnchor::Schedule => TaskResponse::AfterScheduled(self.duration),
            EveryTimeAnchor::After => TaskResponse::AfterTaskEnds(self.duration),
            EveryTimeAnchor::Before => TaskResponse::AfterTaskBegan(self.duration),
        }
    }
}