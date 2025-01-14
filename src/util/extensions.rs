use std::ops::{Range, RangeInclusive};
use crate::for_each_int_type;

/// [Replace] allows for in-place replacement of values.
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

/// [TakeReplace] allows for in-place replacement of values using a transformer function.
pub trait TakeReplace: Sized {
    fn take_replace<F: FnOnce(Self) -> Self>(&mut self, replace: F);
}

impl<T: Sized> TakeReplace for T {
    /// Takes the value and replaces it using a function that takes the value as input and returns the new value.
    #[inline(always)]
    fn take_replace<F: FnOnce(Self) -> Self>(&mut self, replace: F) {
        unsafe {
            std::ptr::write(self, replace(std::ptr::read(self)));
        }
    }
}

/// Private module contained the [Sealed] trait.
mod private {
    pub trait Sealed<T> {}
}

impl<T> private::Sealed<Option<T>> for Option<T> {}

/// An extension to the [Option] type.
pub trait OptionExtension<T>: private::Sealed<Option<T>> {
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

impl private::Sealed<bool> for bool {}

/// An extension to the [bool] type.
pub trait BoolExtension: private::Sealed<bool> {
    fn select<T>(self, _false: T, _true: T) -> T;
    fn select_fn<T, FF: FnOnce() -> T, TF: FnOnce() -> T>(self, _false: FF, _true: TF) -> T;
    fn mark(&mut self) -> bool;
    fn mark_if(&mut self, condition: bool) -> bool;
    fn unmark(&mut self) -> bool;
    fn unmark_if(&mut self, condition: bool) -> bool;
    fn toggle(&mut self) -> Self;
    fn toggle_if(&mut self, condition: bool) -> Self;
    fn if_<F: FnOnce()>(self, _if: F);
    fn if_not<F: FnOnce()>(self, _not: F);
    fn if_else<R, If: FnOnce() -> R, Else: FnOnce() -> R>(self, _if: If, _else: Else) -> R;
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

    /// Execute and return the value of _false or _true depending if self is false or true.
    #[inline]
    fn select_fn<T, FF: FnOnce() -> T, TF: FnOnce() -> T>(self, _false: FF, _true: TF) -> T {
        if self {
            _true()
        } else {
            _false()
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
            !std::mem::replace(self, true)
        } else {
            false
        }
    }

    /// Sets value to false and returns true if it was previously true.
    #[inline]
    fn unmark(&mut self) -> bool {
        std::mem::replace(self, false)
    }

    /// If the condition is met, sets the value to false and returns true if the value was changed.
    #[inline]
    fn unmark_if(&mut self, condition: bool) -> bool {
        if condition {
            std::mem::replace(self, false)
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
    fn if_<F: FnOnce()>(self, _if: F) {
        if self { _if() }
    }
    
    /// `if !self { _not() }`
    #[inline]
    fn if_not<F: FnOnce()>(self, _not: F) {
        if !self { _not() }
    }

    /// Like `if-else`, but with closures!
    #[inline]
    fn if_else<R, If: FnOnce() -> R, Else: FnOnce() -> R>(self, _if: If, _else: Else) -> R {
        self.select_fn(_else, _if)
    }
}

/// An extension to integer types for iteration.
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
    };
}

for_each_int_type!(num_iter_impls);

/// An extension for integer types for incrementation.
pub trait Increment: private::Sealed<()> {
    /// Increment and return the result of incrementation.
    fn increment(&mut self) -> Self;
    /// Increment and return the value prior to incrementation.
    fn post_increment(&mut self) -> Self;
}

/// An extension for integer types for decrementation.
pub trait Decrement: private::Sealed<()> {
    /// Decrement and return the result of decrementation.
    fn decrement(&mut self) -> Self;
    /// Decrement and return the value prior to decrementation.
    fn post_decrement(&mut self) -> Self;
}

macro_rules! inc_dec_impls {
    ($type:ty) => {
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

for_each_int_type!(inc_dec_impls);

/// An extension for [Result].
pub trait ResultExtension: private::Sealed<Result<(), ()>> {
    type Ok;
    type Error;
    fn handle_err<F: FnOnce(Self::Error)>(self, f: F);
    fn try_fn<F: FnOnce() -> Self>(f: F) -> Self;
}

impl<T, E> private::Sealed<std::result::Result<(), ()>> for std::result::Result<T, E> {}

impl<T, E> ResultExtension for std::result::Result<T, E> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bool_ext_test() {
        let mut edit = false;
        debug_assert!(edit.mark_if(true));
        let mut executed = false;
        edit.if_(|| {
            executed.mark();
        });
        debug_assert!(executed);
        debug_assert!(edit);
        assert_eq!(edit.select("false", "true"), "true");
        debug_assert!(!edit.unmark_if(false));
        debug_assert!(edit);
        debug_assert!(edit.unmark_if(true));
        let mut executed = false;
        edit.if_not(|| {
            executed.mark();
        });
        debug_assert!(executed);
        debug_assert!(!edit);
        assert_eq!(edit.select("false", "true"), "false");
    }

    #[test]
    fn num_iter_tests() {
        let mut to_2 = 2.iter();
        debug_assert_eq!(to_2.next(), Some(0));
        debug_assert_eq!(to_2.next(), Some(1));
        debug_assert_eq!(to_2.next(), None);
        let mut to_1_inc = 1.iter_inclusive();
        debug_assert_eq!(to_1_inc.next(), Some(0));
        debug_assert_eq!(to_1_inc.next(), Some(1));
        debug_assert_eq!(to_1_inc.next(), None);
        let mut _2_to_4 = 2.iter_to(4);
        debug_assert_eq!(_2_to_4.next(), Some(2));
        debug_assert_eq!(_2_to_4.next(), Some(3));
        debug_assert_eq!(_2_to_4.next(), None);
        let mut _2_to_3_inc = 2.iter_to_inclusive(3);
        debug_assert_eq!(_2_to_3_inc.next(), Some(2));
        debug_assert_eq!(_2_to_3_inc.next(), Some(3));
        debug_assert_eq!(_2_to_3_inc.next(), None);
        let mut rev_0_to_4 = 4.iter().rev();
        debug_assert_eq!(rev_0_to_4.next(), Some(3));
        debug_assert_eq!(rev_0_to_4.next(), Some(2));
        debug_assert_eq!(rev_0_to_4.next(), Some(1));
        debug_assert_eq!(rev_0_to_4.next(), Some(0));
        debug_assert_eq!(rev_0_to_4.next(), None);
    }

    #[test]
    fn take_replace_test() {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Options {
            Apple(String),
            Banana(String, usize),
        }

        let mut option = Options::Apple(String::from("hello, world"));
        assert_eq!(option, Options::Apple(String::from("hello, world")));
        option.take_replace(|option| match option {
            Options::Apple(text) => Options::Banana(text, 0),
            Options::Banana(text, count) => Options::Banana(text, count + 1),
        });
        assert_eq!(option, Options::Banana(String::from("hello, world"), 0));
        option.take_replace(|option| match option {
            Options::Apple(text) => Options::Banana(text, 0),
            Options::Banana(text, count) => Options::Banana(text, count + 1),
        });
        assert_eq!(option, Options::Banana(String::from("hello, world"), 1));
    }
}