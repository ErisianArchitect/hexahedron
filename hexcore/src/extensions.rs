use std::ops::{Range, RangeInclusive};
use crate::macros::for_each_int_type;
use crate::private::*;

pub trait ToSome: Sized {
    fn some(self) -> Option<Self>;
}

impl<T: Sized> ToSome for T {
    #[inline]
    fn some(self) -> Option<Self> {
        Some(self)
    }
}

pub trait ArrayOfOne: Sized {
    fn array_of_one(self) -> [Self; 1];
}

impl<T: Sized> ArrayOfOne for T {
    fn array_of_one(self) -> [Self; 1] {
        [self]
    }
}

pub trait AsSliceOfOne: Sized {
    fn as_slice_of_one(&self) -> &[Self];
}

impl<T: Sized> AsSliceOfOne for T {
    fn as_slice_of_one(&self) -> &[Self] {
        unsafe {
            std::slice::from_raw_parts(self, 1)
        }
    }
}

pub trait AsSliceOfOneMut: Sized {
    fn as_slice_of_one_mut(&mut self) -> &mut [Self];
}

impl<T: Sized> AsSliceOfOneMut for T {
    fn as_slice_of_one_mut(&mut self) -> &mut [Self] {
        unsafe {
            std::slice::from_raw_parts_mut(self, 1)
        }
    }
}

pub trait TupleOfOne {
    fn tuple_of_one(self) -> (Self,);
}

impl<T: Sized> TupleOfOne for T {
    fn tuple_of_one(self) -> (Self,) {
        (self,)
    }
}

/// [Replace] allows for in-place replacement of values.
pub trait Replace: Sized {
    fn replace(&mut self, src: Self) -> Self;
}

impl<T: Sized> Replace for T {
    /// Replaces `self` with `src`.
    #[inline(always)]
    fn replace(&mut self, src: Self) -> Self {
        std::mem::replace(self, src)
    }
}

/// [ReplaceWith] allows for in-place replacement of values using a transformer function.
pub trait ReplaceWith: Sized {
    fn replace_with<F: FnOnce(Self) -> Self>(&mut self, replace: F);
}

impl<T: Sized> ReplaceWith for T {
    /// Takes the value and replaces it using a function that takes the value as input and returns the new value.
    #[inline(always)]
    fn replace_with<F: FnOnce(Self) -> Self>(&mut self, replace: F) {
        unsafe {
            std::ptr::write(self, replace(std::ptr::read(self)));
        }
    }
}

seal!(marker = Option<T>; <T> for Option<T>);

/// An extension to the [Option] type.
pub trait OptionExtension<T>: Sealed<Option<T>> {
    fn then<F: FnOnce(T)>(self, then: F);
    fn drop(&mut self);
}

impl<T> OptionExtension<T> for Option<T> {
    /// Calls function with inner value as argument if `self` is [Some].
    #[inline]
    fn then<F: FnOnce(T)>(self, then: F) {
        if let Some(value) = self {
            then(value);
        }
    }

    /// If `self` is [Some], take the value and drop it, replacing it with [None].
    #[inline]
    fn drop(&mut self) {
        drop(self.take())
    }
}

seal!(marker = bool; bool);

/// An extension to the [bool] type.
pub trait BoolExtension: Sealed<bool> {
    fn select<T>(self, true_: T, false_: T) -> T;
    fn select_fn<T, TF: FnOnce() -> T, FF: FnOnce() -> T>(self, true_: TF, false_: FF) -> T;
    fn select_unary<T, V, TF: FnOnce(V) -> T, FF: FnOnce(V) -> T>(self, value: V, true_: TF, false_: FF) -> T;
    fn mark(&mut self) -> bool;
    fn mark_if(&mut self, condition: bool) -> bool;
    fn unmark(&mut self) -> bool;
    fn unmark_if(&mut self, condition: bool) -> bool;
    fn toggle(&mut self) -> Self;
    fn toggle_if(&mut self, condition: bool) -> Self;
    fn if_<F: FnOnce()>(self, if_: F);
    fn if_not<F: FnOnce()>(self, not_: F);
    fn if_else<R, If: FnOnce() -> R, Else: FnOnce() -> R>(self, if_: If, else_: Else) -> R;
}

impl BoolExtension for bool {
    /// Choose a truth value or a false value.
    #[inline]
    fn select<T>(self, true_: T, false_: T) -> T {
        if self {
            true_
        } else {
            false_
        }
    }

    /// Execute and return the value of _false or _true depending if self is false or true.
    #[inline]
    fn select_fn<T, TF: FnOnce() -> T, FF: FnOnce() -> T>(self, true_: TF, false_: FF) -> T {
        if self {
            true_()
        } else {
            false_()
        }
    }

    #[inline]
    fn select_unary<T, V, TF: FnOnce(V) -> T, FF: FnOnce(V) -> T>(self, value: V, true_: TF, false_: FF) -> T {
        if self {
            true_(value)
        } else {
            false_(value)
        }
    }

    /// Sets value to true and returns true if it was previously false.
    #[inline]
    fn mark(&mut self) -> bool {
        !std::mem::replace(self, true)
    }

    /// If the condition is met, sets the value to true and returns true if the value was changed.
    #[inline]
    fn mark_if(&mut self, condition: bool) -> bool {
        if condition {
            let changed = !*self;
            *self = true;
            changed
        } else {
            false
        }
    }

    /// Sets value to false and returns true if it was previously true.
    #[inline]
    fn unmark(&mut self) -> bool {
        let changed = *self;
        *self = false;
        changed
    }

    /// If the condition is met, sets the value to false and returns true if the value was changed.
    #[inline]
    fn unmark_if(&mut self, condition: bool) -> bool {
        if condition {
            let changed = *self;
            *self = false;
            changed
        } else {
            false
        }
    }

    /// Inverts the value of the boolean and returns the new value.
    #[inline]
    fn toggle(&mut self) -> Self {
        *self = !*self;
        *self
    }

    /// Toggle value if condition is met and returns the new value.
    #[inline]
    fn toggle_if(&mut self, condition: bool) -> Self {
        if condition {
            *self = !*self
        }
        *self
    }

    /// `if self { _if() }`
    #[inline]
    fn if_<F: FnOnce()>(self, if_: F) {
        if self { if_() }
    }
    
    /// `if !self { _not() }`
    #[inline]
    fn if_not<F: FnOnce()>(self, not_: F) {
        if !self { not_() }
    }

    /// Like `if-else`, but with closures!
    #[inline]
    fn if_else<R, If: FnOnce() -> R, Else: FnOnce() -> R>(self, if_: If, else_: Else) -> R {
        self.select_fn(if_, else_)
    }
}

/// An extension to integer types for iteration.
pub trait NumIter: Sized + Copy + Sealed<()> {
    fn iter(self) -> Range<Self>;
    fn iter_inclusive(self) -> RangeInclusive<Self>;
    fn iter_to(self, end: Self) -> Range<Self>;
    fn iter_to_inclusive(self, end: Self) -> RangeInclusive<Self>;
}

/// An extension for integer types for incrementation.
pub trait Increment: Sealed<()> {
    /// Increment and return the result of incrementation.
    fn increment(&mut self) -> Self;
    /// Increment and return the value prior to incrementation.
    fn post_increment(&mut self) -> Self;
}

/// An extension for integer types for decrementation.
pub trait Decrement: Sealed<()> {
    /// Decrement and return the result of decrementation.
    fn decrement(&mut self) -> Self;
    /// Decrement and return the value prior to decrementation.
    fn post_decrement(&mut self) -> Self;
}

macro_rules! num_impls {
    ($type:ty) => {
        seal!($type);
        impl NumIter for $type {
            #[doc = "Returns a [Range] from [0] to [self]."]
            #[inline]
            fn iter(self) -> Range<Self> {
                0..self
            }

            #[doc = "Returns a [RangeInclusive] from [0] to [self]."]
            #[inline]
            fn iter_inclusive(self) -> RangeInclusive<Self> {
                0..=self
            }

            #[doc = "Returns a [Range] from [self] to [end]."]
            #[inline]
            fn iter_to(self, end: Self) -> Range<Self> {
                self..end
            }

            #[doc = "Returns a [RangeInclusive] from [self] to [end]."]
            #[inline]
            fn iter_to_inclusive(self, end: Self) -> RangeInclusive<Self> {
                self..=end
            }
        }

        impl Increment for $type {
            #[doc = "Increment [self] by [1] and return the result."]
            #[inline]
            fn increment(&mut self) -> Self {
                *self += 1;
                *self
            }

            #[doc = "Increment [self] by [1] and return the value before incrementation."]
            #[inline]
            fn post_increment(&mut self) -> Self {
                let original = *self;
                *self += 1;
                original
            }
        }

        impl Decrement for $type {
            #[doc = "Decrement [self] by [1] and return the result."]
            #[inline]
            fn decrement(&mut self) -> Self {
                *self -= 1;
                *self
            }

            #[doc = "Decrement [self] by [1] and return the value before decrementation."]
            #[inline]
            fn post_decrement(&mut self) -> Self {
                let original = *self;
                *self -= 1;
                original
            }
        }
    };
}

for_each_int_type!(num_impls);

/// An extension for [Result].
pub trait ResultExtension: Sealed<Result<(), ()>> {
    type Ok;
    type Error;
    fn handle_err<F: FnOnce(Self::Error)>(self, f: F);
    fn try_fn<F: FnOnce() -> Self>(f: F) -> Self;
}

seal!(marker = Result<(), ()>; <T, E> for Result<T, E>);

impl<T, E> ResultExtension for Result<T, E> {
    type Ok = T;
    type Error = E;
    /// For when you want to ignore the return value of a result but you also want to handle the error if there is one.
    #[inline]
    fn handle_err<F: FnOnce(E)>(self, f: F) {
        if let std::result::Result::Err(err) = self {
            f(err);
        }
    }

    /// Calls a fallible function and returns the result.
    #[inline]
    fn try_fn<F: FnOnce() -> Self>(f: F) -> Self {
        f()
    }
}