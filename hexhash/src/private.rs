/// When a trait is constrained to [Sealed], allows for that trait to be sealed from being implemented externally.
pub trait Sealed<T> {}

#[macro_export]
macro_rules! seal {
    (marker = $marker:ty; $($token:tt)*) => {
        hexmacros::mark!(trait = $crate::private::Sealed<$marker>; $($token)*);
    };
    ($($token:tt)*) => {
        hexmacros::mark!(trait = $crate::private::Sealed<()>; $($token)*);
    };
}

#[macro_export]
macro_rules! sealed {
    [$marker:ty] => {
        $crate::private::Sealed<$marker>
    };
    [] => {
        $crate::private::Sealed<()>
    };
}

#[allow(unused)]
pub(crate) use crate::seal;
#[allow(unused)]
pub(crate) use crate::sealed;