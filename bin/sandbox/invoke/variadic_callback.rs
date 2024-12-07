use super::task_context::{
    TaskContext,
};
use super::callback::Callback;
use super::task_response::TaskResponse;

pub trait TaskContextType {}

pub trait VariadicCallbackContext<Ctx> {
    #[inline]
    fn resolve(ctx: Ctx) -> Self;
}

impl<T> VariadicCallbackContext<T> for T
where T: TaskContextType {
    #[inline]
    fn resolve(ctx: T) -> Self {
        ctx
    }
}

impl<T> VariadicCallbackContext<T> for () {
    fn resolve(ctx: T) -> Self {}
}

pub trait VariadicCallback<DataArgs, Args, CtxT, CtxArg, R>
where
CtxArg: VariadicCallbackContext<CtxT> {
    #[inline]
    fn invoke(
        &mut self,
        data: &mut DataArgs,
        task_ctx: CtxArg,
    ) -> R;
}

impl<DataArgs, Args, CtxT, CtxArg, R, C> VariadicCallback<(), Args, CtxT, CtxArg, R> for (DataArgs, C)
where 
CtxArg: VariadicCallbackContext<CtxT>,
C: VariadicCallback<DataArgs, Args, CtxT, CtxArg, R> {
    #[inline]
    fn invoke(
            &mut self,
            data: &mut (),
            task_ctx: CtxArg,
        ) -> R {
        self.1.invoke(&mut self.0, task_ctx)
    }
}

// impl<Args, CtxT, CtxArg, R, C> Callback for C
// where
// R: Into<TaskResponse>,
// CtxArg: VariadicCallbackContext<CtxT>,
// C: VariadicCallback<(), Args, CtxT, CtxArg, R> {
//     #[inline]
//     fn invoke(
//             &mut self,
//             task_ctx: TaskContext<'_>,
//         ) -> super::task_response::TaskResponse {
//         self.invoke(&mut (), task_ctx)
//     }
// }
