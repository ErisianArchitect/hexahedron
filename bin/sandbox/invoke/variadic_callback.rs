use paste::paste;
use super::task_context::{
    TaskContext,
};
use super::callback::Callback;
use super::task_response::TaskResponse;

pub trait VariadicCallbackOnce<Args, Result>: 'static {
    fn call_consume(self, args: Args) -> Result;
}

pub trait VariadicCallbackRef<Args, Result>: 'static {
    fn call_immutable(&self, args: Args) -> Result;
}

pub trait VariadicCallbackMut<Args, Result>: 'static {
    fn call_mutable(&mut self, args: Args) -> Result;
}

macro_rules! variadic_callback_impls {
    ($([ $($arg_t:ident),*$(,)? ])+) => {
        $(
            variadic_callback_impls!{$($arg_t),*}
        )+
    };
    ($($arg_t:ident),*$(,)?) => {
        paste!{
            impl<$($arg_t,)* R, F> VariadicCallbackOnce<($($arg_t,)*), R> for F
            where
            F: FnOnce($($arg_t),*) -> R + 'static {
                #[allow(non_snake_case)]
                fn call_consume(self, args: ($($arg_t,)*)) -> R {
                    let (
                        $(
                            [<_ $arg_t>],
                        )*
                    ) = args;
                    (self)(
                        $(
                            [<_ $arg_t>],
                        )*
                    )
                }
            }
        }
        paste!{
            impl<$($arg_t,)* R, F> VariadicCallbackRef<($($arg_t,)*), R> for F
            where
            F: Fn($($arg_t),*) -> R + 'static {
                #[allow(non_snake_case)]
                fn call_immutable(&self, args: ($($arg_t,)*)) -> R {
                    let (
                        $(
                            [<_ $arg_t>],
                        )*
                    ) = args;
                    (self)(
                        $(
                            [<_ $arg_t>],
                        )*
                    )
                }
            }
        }

        paste!{
            impl<$($arg_t,)* R, F> VariadicCallbackMut<($($arg_t,)*), R> for F
            where
            F: FnMut($($arg_t),*) -> R + 'static {
                #[allow(non_snake_case)]
                fn call_mutable(&mut self, args: ($($arg_t,)*)) -> R {
                    let (
                        $(
                            [<_ $arg_t>],
                        )*
                    ) = args;
                    (self)(
                        $(
                            [<_ $arg_t>],
                        )*
                    )
                }
            }
        }
    };
}

variadic_callback_impls!{
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
}

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
        struct Consumable(String);
        let mut cons = Consumable(String::from("Hello, world!"));
        let mut cb = move |num: i32| {
            println!("{num}, {}", cons.0);
            cons.0 = String::from("The quick brown fox jumps over the lazy dog.");
            num
        };
        cb.call_mutable((32,));
        cb.call_consume((32,));
    }
}