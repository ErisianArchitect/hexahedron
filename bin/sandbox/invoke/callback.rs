use super::{
    task_context::TaskContext,
    task_response::TaskResponse,
    scheduler_context::*,
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