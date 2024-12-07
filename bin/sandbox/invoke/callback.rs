use super::{task_context::TaskContext, task_response::TaskResponse};

pub trait Callback: 'static {
    #[inline]
    fn invoke(
        &mut self,
        task_ctx: TaskContext<'_>,
    ) -> TaskResponse;
}