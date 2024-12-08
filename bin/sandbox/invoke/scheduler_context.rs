use super::optional::Optional;

/// The context from which arguments can be retrieved
pub trait SchedulerContext: Sized + 'static {
    // fn get<T: ContextResolvable<Self>>(&mut self) -> Option<T>;
}

impl SchedulerContext for () {
    // fn get<T: ContextResolvable<Self>>(&mut self) -> Option<T> {
    //     None
    // }
}

// pub trait ResolveContext<'ctx, Ctx: SchedulerContext> {
//     fn resolve(ctx: &'ctx mut Ctx) -> Self;
// }

// impl<'ctx, Ctx: SchedulerContext> ResolveContext<'ctx, Ctx> for () {
//     fn resolve(ctx: &'ctx mut Ctx) -> Self {}
// }

// impl<'ctx, Ctx: SchedulerContext> ResolveContext<'ctx, Ctx> for &'ctx mut Ctx {
//     fn resolve(ctx: &'ctx mut Ctx) -> Self {
//         ctx
//     }
// }

// pub enum ResolveResult<T: Sized + 'static> {
//     /// For when the type can't be resolved and this indicates an error.
//     NotFound,
//     /// For when the type can't be resolved but this does not indicate an error, and instead the task should be skipped.
//     Skip,
//     /// For when the type can be resolved.
//     Ok(T),
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResolveError {
    NotFound(&'static str),
    Skip,
}

impl ResolveError {
    #[inline]
    pub const fn err<T>(self) -> ResolveResult<T> {
        Err(self)
    }

    #[inline]
    pub fn not_found<T>() -> ResolveResult<T> {
        Err(Self::NotFound(std::any::type_name::<T>()))
    }

    #[inline]
    pub const fn skip<T>() -> ResolveResult<T> {
        Err(Self::Skip)
    }
}

impl<T> Into<ResolveResult<T>> for ResolveError {
    fn into(self) -> ResolveResult<T> {
        Err(self)
    }
}

pub type ResolveResult<T> = std::result::Result<T, ResolveError>;

pub trait ContextResolvable<Ctx>: Sized + 'static
where Ctx: SchedulerContext {
    fn resolve(context: &mut Ctx) -> ResolveResult<Self>;
}

impl<Ctx, T> ContextResolvable<Ctx> for Optional<T>
where
Ctx: SchedulerContext,
T: ContextResolvable<Ctx> {
    fn resolve(context: &mut Ctx) -> ResolveResult<Self> {
        ResolveResult::Ok(match T::resolve(context) {
            Ok(some) => Optional::Some(some),
            Err(_) => Optional::None,
        })
    }
}

pub struct Skip<T>(pub T);

impl<T: Clone> Clone for Skip<T> {
    fn clone(&self) -> Self {
        Skip(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = source.0.clone()
    }
}

impl<T: Copy> Copy for Skip<T> {}

impl<Ctx, T> ContextResolvable<Ctx> for Skip<T>
where
Ctx: SchedulerContext,
T: ContextResolvable<Ctx> {
    fn resolve(context: &mut Ctx) -> ResolveResult<Self> {
        Ok(match T::resolve(context) {
            Err(_) => return ResolveError::skip(),
            Ok(inner) => Skip(inner),
        })
    }
}

pub trait ResolvableGroup<Ctx>: Sized + 'static
where Ctx: SchedulerContext {
    fn group_resolve(context: &mut Ctx) -> ResolveResult<Self>;
}

macro_rules! group_resolver_impls {
    ($($tn:ident),*) => {
        impl<$($tn,)* Ctx> ResolvableGroup<Ctx> for ($($tn,)*)
        where
        Ctx: SchedulerContext,
        $(
            $tn: ContextResolvable<Ctx>,
        )*
        {
            fn group_resolve(context: &mut Ctx) -> ResolveResult<Self> {
                Ok((
                    $(
                        $tn::resolve(context)?,
                    )*
                ))
            }
        }
    };
    ($([$($tn:ident),*])+) => {
        $(
            group_resolver_impls!{$($tn),*}
        )+
    };
}

group_resolver_impls!{
    []
    [T0]
    [T0, T1]
    [T0, T1, T2]
    [T0, T1, T2, T3]
    [T0, T1, T2, T3, T4]
    [T0, T1, T2, T3, T4, T5]
    [T0, T1, T2, T3, T4, T5, T6]
    [T0, T1, T2, T3, T4, T5, T6, T7]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30]
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31]
}

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
        let tn: &'static str = std::any::type_name::<i32>();
    }
}