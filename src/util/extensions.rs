use std::ops::{Range, RangeInclusive};
use crate::for_each_int_type;

pub trait Replace {
    fn replace(&mut self, src: Self) -> Self;
}

impl<T> Replace for T {
    /// Replaces `self` with `src`.
    #[inline(always)]
    fn replace(&mut self, src: Self) -> Self {
        std::mem::replace(self, src)
    }
}

mod private {
    pub trait Sealed<T> {}
}

impl<T> private::Sealed<Option<T>> for Option<T> {}

pub trait OptionExtension<T>: private::Sealed<Option<T>> {
    fn then<F: FnOnce(T)>(self, then: F);
}

impl<T> OptionExtension<T> for Option<T> {
    #[inline]
    fn then<F: FnOnce(T)>(self, then: F) {
        if let Some(value) = self {
            then(value);
        }
    }
}

impl private::Sealed<bool> for bool {}

pub trait BoolExtension: private::Sealed<bool> {
    fn select<T>(self, _false: T, _true: T) -> T;
    fn select_fn<T, FF: FnOnce() -> T, TF: FnOnce() -> T>(self, _false: FF, _true: TF) -> T;
    fn toggle(&mut self) -> Self;
    fn toggle_if(&mut self, condition: bool) -> Self;
    fn some<T>(self, value: T) -> Option<T>;
    fn some_fn<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;
    fn some_else<T>(self, value: T) -> Option<T>;
    fn some_else_fn<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;
    fn if_<F: Fn()>(self, _if: F);
    fn if_not<F: Fn()>(self, _not: F);
    fn if_else<R, If: Fn() -> R, Else: Fn() -> R>(self, _if: If, _else: Else) -> R;
}

impl BoolExtension for bool {
    /// Choose a truth value or a false value.
    #[inline]
    fn select<T>(self, _false: T, _true: T) -> T {
        if self {
            _true
        } else {
            _false
        }
    }

    #[inline]
    fn select_fn<T, FF: FnOnce() -> T, TF: FnOnce() -> T>(self, _false: FF, _true: TF) -> T {
        if self {
            _true()
        } else {
            _false()
        }
    }

    /// Inverts the value of the boolean.
    #[inline]
    fn toggle(&mut self) -> Self {
        *self = !*self;
        *self
    }

    #[inline]
    fn toggle_if(&mut self, condition: bool) -> Self {
        if condition {
            *self = !*self
        }
        *self
    }

    /// Returns `Some(some)` if true.
    #[inline]
    fn some<T>(self, value: T) -> Option<T> {
        if self {
            Some(value)
        } else {
            None
        }
    }

    fn some_fn<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        if self {
            Some(f())
        } else {
            None
        }
    }

    /// Returns `Some(some)` if false.
    #[inline]
    fn some_else<T>(self, value: T) -> Option<T> {
        if !self {
            Some(value)
        } else {
            None
        }
    }

    fn some_else_fn<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        if !self {
            Some(f())
        } else {
            None
        }
    }

    #[inline]
    fn if_<F: Fn()>(self, _if: F) {
        if self {
            _if();
        }
    }
    
    #[inline]
    fn if_not<F: Fn()>(self, _not: F) {
        if !self {
            _not();
        }
    }

    /// Like `if-else`, but with closures!
    #[inline]
    fn if_else<R, If: Fn() -> R, Else: Fn() -> R>(self, _if: If, _else: Else) -> R {
        if self {
            _if()
        } else {
            _else()
        }
    }
}

pub trait NumIter: Sized + Copy + private::Sealed<()> {
    fn iter(self) -> Range<Self>;
    fn iter_inclusive(self) -> RangeInclusive<Self>;
    fn iter_to(self, end: Self) -> Range<Self>;
    fn iter_to_inclusive(self, end: Self) -> RangeInclusive<Self>;
}

macro_rules! num_iter_impls {
    ($type:ty) => {
        impl private::Sealed<()> for $type {}
        impl NumIter for $type {
            #[inline]
            fn iter(self) -> Range<Self> {
                0..self
            }

            #[inline]
            fn iter_inclusive(self) -> RangeInclusive<Self> {
                0..=self
            }

            #[inline]
            fn iter_to(self, end: Self) -> Range<Self> {
                self..end
            }

            #[inline]
            fn iter_to_inclusive(self, end: Self) -> RangeInclusive<Self> {
                self..=end
            }
        }
    };
}

for_each_int_type!(num_iter_impls);

pub trait ResultExtension: private::Sealed<Result<(), ()>> {
    type Ok;
    type Error;
    fn handle_err<F: FnMut(Self::Error)>(self, f: F);
    fn try_fn<F: FnMut() -> Self>(f: F) -> Self;
}

impl<T, E> private::Sealed<std::result::Result<(), ()>> for std::result::Result<T, E> {}

impl<T, E> ResultExtension for std::result::Result<T, E> {
    type Ok = T;
    type Error = E;
    #[inline]
    fn handle_err<F: FnMut(E)>(self, mut f: F) {
        if let std::result::Result::Err(err) = self {
            f(err);
        }
    }

    #[inline]
    fn try_fn<F: FnMut() -> Self>(mut f: F) -> Self {
        f()
    }
}