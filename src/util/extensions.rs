use std::ops::Range;
use crate::for_each_int_type;

pub trait SwapVal {
    fn swap(&mut self, swap: Self) -> Self;
}

impl<T> SwapVal for T {
    #[inline(always)]
    fn swap(&mut self, mut swap: Self) -> Self {
        std::mem::swap(self, &mut swap);
        swap
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
    fn toggle(&mut self) -> Self;
    fn some<T>(self, some: T) -> Option<T>;
    fn some_else<T>(self, some: T) -> Option<T>;
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

    /// Inverts the value of the boolean.
    #[inline]
    fn toggle(&mut self) -> Self {
        *self = !*self;
        *self
    }

    /// Returns `Some(some)` if true.
    #[inline]
    fn some<T>(self, some: T) -> Option<T> {
        self.select(Some(some), None)
    }

    /// Returns `Some(some)` if false.
    #[inline]
    fn some_else<T>(self, some: T) -> Option<T> {
        self.select(None, Some(some))
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
    fn iter_to(self, end: Self) -> Range<Self>;
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
            fn iter_to(self, end: Self) -> Range<Self> {
                self..end
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