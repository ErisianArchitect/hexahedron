#[derive(Debug)]
pub enum Change<T> {
    Unchanged,
    /// Holds the previous value.
    Changed(T),
}

impl<T> Change<T> {
    #[inline]
    pub fn some(self) -> Option<T> {
        if let Self::Changed(old) = self {
            Some(old)
        } else {
            None
        }
    }

    /// Returns true if the value changed.
    #[inline]
    pub fn changed(&self) -> bool {
        matches!(self, Change::Changed(_))
    }

    /// Calls a function with the old value as the argument if the value has
    /// changed.
    #[inline]
    pub fn if_changed<F: FnOnce(T)>(self, f: F) {
        if let Self::Changed(old_value) = self {
            f(old_value);
        }
    }

    #[inline]
    pub fn expect(self, msg: &str) -> T {
        let Self::Changed(previous) = self else {
            panic!("{msg}");
        };
        previous
    }

    #[inline]
    pub fn unwrap(self) -> T {
        self.expect("Attempted to unwrap Change::Unchanged (no previous value present).")
    }
}

impl<T: PartialEq> PartialEq for Change<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Changed(l0), Self::Changed(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<T: Eq> Eq for Change<T> {}

impl<T: PartialEq> Change<T> {
    #[inline]
    pub fn cmp_new(new_value: &T, old_value: T) -> Self {
        if *new_value != old_value {
            Self::Changed(old_value)
        } else {
            Self::Unchanged
        }
    }
}

impl<T: Clone> Clone for Change<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Unchanged => Self::Unchanged,
            Self::Changed(old_value) => Self::Changed(old_value.clone()),
        }
    }
}

impl<T: Clone + Copy> Copy for Change<T> {}

pub trait ReplaceCompare: Sized {
    /// Compares `self` with `src`, and replaces `self` with `src` if
    /// they are not equal. Returns `Change::Changed(old_value)` if the
    /// replacement occurred.
    fn replace_compare(&mut self, src: Self) -> Change<Self>;
}

impl<T: PartialEq> ReplaceCompare for T {
    /// Compares `self` with `src`, and replaces `self` with `src` if
    /// they are not equal. Returns `Change::Changed(old_value)` if the
    /// replacement occurred.
    #[inline]
    fn replace_compare(&mut self, src: Self) -> Change<Self> {
        if *self == src {
            Change::Unchanged
        } else {
            let old_value = std::mem::replace(self, src);
            Change::Changed(old_value)
        }
    }
}