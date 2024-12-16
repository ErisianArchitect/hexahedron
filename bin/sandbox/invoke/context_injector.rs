use paste::paste;
use std::marker::PhantomData;

use super::{
    callback::Callback,
    scheduler_context::SchedulerContext,
    task_response::TaskResponse,
    task_context::TaskContext,
    scheduler_context::*,
};

pub struct ContextInjector<DataArgs, Args, ContextArg, F, Output, Ctx>
where
DataArgs: 'static,
ContextArg: 'static,
Args: 'static,
Output: 'static,
F: 'static,
Ctx: SchedulerContext,
Self: Callback<Ctx> {
    phantom: PhantomData<(Args, Output, ContextArg, Ctx)>,
    data: DataArgs,
    callback: F,
}

impl<DataArgs, Args, ContextArg, F, Output, Ctx> ContextInjector<DataArgs, Args, ContextArg, F, Output, Ctx>
where
DataArgs: 'static,
Args: 'static,
Output: 'static,
ContextArg: 'static,
Ctx: SchedulerContext,
F: 'static,
Self: Callback<Ctx> {
    pub fn new<NArgs, NContextArg, NF, NOutput, NCtx>(callback: NF) -> ContextInjector<(), NArgs, NContextArg, NF, NOutput, NCtx>
    where
    // NArgs: 'static,
    // NOutput: 'static,
    NCtx: SchedulerContext,
    ContextInjector<(), NArgs, NContextArg, NF, NOutput, NCtx>: Callback<NCtx> {
        ContextInjector {
            phantom: PhantomData,
            data: (),
            callback,
        }
    }

    pub fn with_data<NDataArgs, NArgs, NContextArg, NF, NOutput, NCtx>(data: NDataArgs, callback: NF) -> ContextInjector<NDataArgs, NArgs, NContextArg, NF, NOutput, NCtx>
    where
    NF: Fn() -> NOutput + 'static,
    NCtx: SchedulerContext,
    ContextInjector<NDataArgs, NArgs, NContextArg, NF, NOutput, NCtx>: Callback<NCtx> {
        ContextInjector {
            phantom: PhantomData,
            data,
            callback,
        }
    }
}

/// Creates a [ContextInjector] callback with a data attachment suitable for passing into a [Scheduler].
pub fn with<DataArgs, Args, ContextArg, F, Output, Ctx>(data: DataArgs, callback: F) -> ContextInjector<DataArgs, Args, ContextArg, F, Output, Ctx>
where
Ctx: SchedulerContext,
ContextInjector<DataArgs, Args, ContextArg, F, Output, Ctx>: Callback<Ctx> {
    ContextInjector {
        phantom: PhantomData,
        data,
        callback,
    }
}

macro_rules! context_injector_impls {
    (@ctx_arg; WithContext; $context:ident) => {
        $context
    };
    (@ctx_type; WithContext($ctx_ident: ident)) => {
        TaskContext<'_, $ctx_ident>
    };
    (@right_context; ( $($data_type:ident),* ), ( $($arg_type:ident),* ), ($($ctx:ident),*)) => {
        paste!{
            impl<$($data_type,)* $($arg_type,)* R, F, Ctx: SchedulerContext> Callback<Ctx> for ContextInjector<($($data_type,)*), ($($arg_type,)*), ( ($($data_type,)*), ($($arg_type,)*), ($(context_injector_impls!(@ctx_type; $ctx(Ctx)),)*) ), F, R, Ctx>
            where
            R: Into<TaskResponse> + 'static,
            $(
                $data_type: 'static,
            )*
            $(
                $arg_type: ContextResolvable<Ctx>,
            )*
            F: FnMut(
                $(
                    &mut $data_type,
                )*
                $(
                    $arg_type,
                )*
                $(
                    context_injector_impls!(@ctx_type; $ctx(Ctx)),
                )*
            ) -> R {
                #[allow(non_snake_case)]
                fn invoke(
                    &mut self,
                    context: TaskContext<'_, Ctx>,
                    data: &mut (),
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
                            match $arg_type::resolve(context.shared) {
                                Ok(resolved) => resolved,
                                Err(ResolveError::Skip) => return TaskResponse::Continue,
                                Err(ResolveError::NotFound(type_name)) => {
                                    panic!("{type_name} not found in context");
                                }
                            },
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