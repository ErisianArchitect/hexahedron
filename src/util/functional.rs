#[inline]
pub fn pass<T>(value: T) -> T {
    value
}

#[inline]
pub fn eval<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}

#[inline]
pub fn catch<T, E, F: FnOnce() -> Result<T, E>>(f: F) -> Result<T, E> {
    f()
}

#[inline]
pub const fn noop() {}

macro_rules! noop {
    ($( $name:ident <$($t:ident),+$(,)?> ;)+) => {
        $(
            #[inline]
            pub fn $name<$($t),*>($(_: $t),*) {}
        )+
    };
}

noop! {
    noop_1<T0>;
    noop_2<T0, T1>;
    noop_3<T0, T1, T2>;
    noop_4<T0, T1, T2, T3>;
    noop_5<T0, T1, T2, T3, T4>;
    noop_6<T0, T1, T2, T3, T4, T5>;
    noop_7<T0, T1, T2, T3, T4, T5, T6>;
    noop_8<T0, T1, T2, T3, T4, T5, T6, T7>;
    noop_9<T0, T1, T2, T3, T4, T5, T6, T7, T8>;
    noop_10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>;
    noop_11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>;
    noop_12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>;
    noop_13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>;
    noop_14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>;
    noop_15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>;
    noop_16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>;
    noop_17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>;
    noop_18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>;
    noop_19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>;
    noop_20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>;
    noop_21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>;
    noop_22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>;
    noop_23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>;
    noop_24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>;
    noop_25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>;
    noop_26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>;
    noop_27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>;
    noop_28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>;
    noop_29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>;
    noop_30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>;
    noop_31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>;
    noop_32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn nop_test() {
        noop();
        let start = std::time::Instant::now();
        (0..1000000000).for_each(noop_1);
        let elapsed = start.elapsed();
        println!("Time: {:.4}", elapsed.as_secs_f64());
    }
}