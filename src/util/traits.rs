pub trait StrToOwned {
    fn owned_string(self) -> String;
}

impl StrToOwned for String {
    fn owned_string(self) -> String {
        self
    }
}

impl StrToOwned for &str {
    fn owned_string(self) -> String {
        self.to_owned()
    }
}

impl StrToOwned for &String {
    fn owned_string(self) -> String {
        self.to_owned()
    }
}


pub trait SameType<T> {}

impl<T> SameType<T> for T {}

// Marker for tuples that are 32 items or less.
pub trait SmallTuple {}

pub trait AnyTuple: std::any::Any {}

macro_rules! any_tuples {
    ($($type_ident:ident),*) => {
        impl<$($type_ident : std::any::Any,)*> AnyTuple for ($($type_ident,)*) {}
    };
    ($([$($type_ident:ident),*])*) => {
        $(
            any_tuples!($($type_ident),*);
        )*
    };
}

macro_rules! small_tuples {
    ($($type_ident:ident),*) => {
        impl<$($type_ident,)*> SmallTuple for ($($type_ident,)*) {}
    };
    ($([$($type_ident:ident),*])*) => {
        $(
            small_tuples!($($type_ident),*);
        )*
    };
}

macro_rules! small_tuple_traits {
    ($($macro:ident),*$(,)?) => {
        $(
            $macro!(
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
            );
        )*
    };
}

small_tuple_traits!(
    small_tuples,
    any_tuples,
);