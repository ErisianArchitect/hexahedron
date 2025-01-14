pub fn eval<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}

pub fn catch<T, E, F: FnOnce() -> Result<T, E>>(f: F) -> Result<T, E> {
    f()
}

pub const fn nop() {}

macro_rules! nop_fns {
    ($( $name:ident <$($t:ident),+$(,)?> ;)+) => {
        $(
            pub fn $name<$($t),*>($(_: $t),*) {}
        )+
    };
}

nop_fns! {
    nop_1<T0>;
    nop_2<T0, T1>;
    nop_3<T0, T1, T2>;
    nop_4<T0, T1, T2, T3>;
    nop_5<T0, T1, T2, T3, T4>;
    nop_6<T0, T1, T2, T3, T4, T5>;
    nop_7<T0, T1, T2, T3, T4, T5, T6>;
    nop_8<T0, T1, T2, T3, T4, T5, T6, T7>;
    nop_9<T0, T1, T2, T3, T4, T5, T6, T7, T8>;
    nop_10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>;
    nop_11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>;
    nop_12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>;
    nop_13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>;
    nop_14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>;
    nop_15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>;
    nop_16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>;
    nop_17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>;
    nop_18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>;
    nop_19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>;
    nop_20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>;
    nop_21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>;
    nop_22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>;
    nop_23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>;
    nop_24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>;
    nop_25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>;
    nop_26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>;
    nop_27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>;
    nop_28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>;
    nop_29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>;
    nop_30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>;
    nop_31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>;
    nop_32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn nop_test() {
        nop();
        let start = std::time::Instant::now();
        (0..1000000000).for_each(nop_1);
        let elapsed = start.elapsed();
        println!("Time: {:.4}", elapsed.as_secs_f64());
    }
}