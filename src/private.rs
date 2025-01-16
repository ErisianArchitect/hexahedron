/// When a trait is constrained to [Sealed], allows for that trait to be sealed from being implemented externally.
pub trait Sealed<T> {}

#[macro_export]
macro_rules! seal {
    ($marker:ty; $($type:ty),+$(,)?) => {
        $(
            impl $crate::private::Sealed<$marker> for $type {}
        )+
    };
    ($($type:ty),+$(,)?) => {
        $(
            impl $crate::private::Sealed<()> for $type {}
        )+
    };
    ($marker:ty; where: $($trait:path),+$(,)?) => {
        impl<T> $crate::private::Sealed<$marker> for T
        where T: $($trait+)+ {}
    };
    (where: $($trait:path),+$(,)?) => {
        impl<T> $crate::private::Sealed<()> for T
        where T: $($trait+)+ {}
    };
}

#[allow(unused)]
pub use crate::seal;

use hexmacros::*;

prototype_macro!(in crate; Type, <T> for (T, T) where );